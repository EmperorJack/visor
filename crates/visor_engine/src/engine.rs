use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use tao::{
    event::WindowEvent,
    window::{Window, WindowId},
};
use tokio::{
    runtime::{Handle, Runtime},
    sync::mpsc,
    task::JoinSet,
};
use uuid::Uuid;
use visor_display::display_manager::DisplayManager;
use visor_runtime::startup_snapshot::{StartupSnapshot, STARTUP_SNAPSHOT_CELL};
use visor_wgpu::render_texture::RenderTexture;

use crate::{
    plugin::{load_plugin, LoadedPlugin, Plugin},
    sketch::Sketch,
    store::{Store, ENGINE_STORE},
};

pub use crate::sketch::SketchId;

pub use visor_display::display::{Display, DisplayId};
pub use visor_wgpu::render_texture::RenderTextureId;

static PLUGINS_CELL: OnceLock<Vec<LoadedPlugin>> = OnceLock::new();

pub struct Engine {
    _runtime: Option<Runtime>,
    runtime_handle: Handle,
    sketches: HashMap<SketchId, Sketch>,
    render_textures: HashMap<RenderTextureId, RenderTexture>,
    display_manager: DisplayManager,
    tao_window_event_sender: mpsc::UnboundedSender<(WindowId, WindowEvent<'static>)>,
    tao_window_event_receiver: mpsc::UnboundedReceiver<(WindowId, WindowEvent<'static>)>,
    wgpu_instance: nannou::wgpu::Instance,
    wgpu_device: Arc<nannou::wgpu::Device>,
    wgpu_queue: nannou::wgpu::Queue,
}

impl Engine {
    pub fn new(
        runtime_handle: Option<Handle>,
        plugins: Vec<Box<dyn Plugin>>,
        linked_plugin_paths: Vec<PathBuf>,
    ) -> Self {
        // TODO: use a logging crate
        println!("[Engine] Setting up Visor engine...");

        let (runtime, runtime_handle) = runtime_handle
            .map(|runtime_handle| (None, runtime_handle))
            .unwrap_or_else(|| {
                let runtime = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .expect("Unexpected: could not create tokio runtime");

                let runtime_handle = runtime.handle().clone();

                (Some(runtime), runtime_handle)
            })
            .into();

        let (tao_window_event_sender, tao_window_event_receiver) = mpsc::unbounded_channel();

        let compiled_plugins: Vec<LoadedPlugin> =
            plugins.into_iter().map(LoadedPlugin::Compiled).collect();

        let linked_plugins: Vec<LoadedPlugin> = linked_plugin_paths
            .into_iter()
            .map(|path| {
                let plugin = unsafe { load_plugin(&path) }.unwrap_or_else(|_| {
                    panic!(
                        "Unexpected: could not load plugin at path {}",
                        path.display()
                    )
                });

                LoadedPlugin::Linked(plugin)
            })
            .collect();

        let all_plugins: Vec<LoadedPlugin> =
            compiled_plugins.into_iter().chain(linked_plugins).collect();

        STARTUP_SNAPSHOT_CELL.get_or_init(|| {
            let plugin_extensions = all_plugins
                .iter()
                .map(|plugin| plugin.extension())
                .collect();

            StartupSnapshot::new(plugin_extensions)
        });

        PLUGINS_CELL.get_or_init(|| all_plugins);

        let wgpu_instance = nannou::wgpu::Instance::default();

        let (wgpu_device, wgpu_queue) = runtime_handle.block_on(async {
            let adapter = wgpu_instance
                .request_adapter(&nannou::wgpu::RequestAdapterOptions {
                    power_preference: nannou::wgpu::PowerPreference::HighPerformance,
                    compatible_surface: None,
                    force_fallback_adapter: false,
                })
                .await
                .expect("Unexpected: could not request wgpu adapter");

            adapter
                .request_device(
                    &nannou::wgpu::DeviceDescriptor {
                        features: nannou::wgpu::Features::empty(),
                        limits: nannou::wgpu::Limits::default(),
                        label: None,
                    },
                    None,
                )
                .await
                .expect("Unexpected: could not connect to wgpu device")
        });

        let display_manager = DisplayManager::new(runtime_handle.clone());

        let mut engine = Engine {
            _runtime: runtime,
            runtime_handle,
            sketches: Default::default(),
            render_textures: Default::default(),
            display_manager,
            tao_window_event_sender,
            tao_window_event_receiver,
            wgpu_instance,
            wgpu_device: Arc::new(wgpu_device),
            wgpu_queue,
        };

        for plugin in Self::plugins() {
            plugin.build(&mut engine, &ENGINE_STORE);
        }

        println!("[Engine] {} plugins loaded.", Self::plugins().len());

        println!("[Engine] Visor engine ready!");

        engine
    }

    pub fn tao_window_event_sender(
        &self,
    ) -> mpsc::UnboundedSender<(WindowId, WindowEvent<'static>)> {
        self.tao_window_event_sender.clone()
    }

    pub fn update(&mut self) {
        for plugin in Self::plugins() {
            plugin.engine_update(self, &ENGINE_STORE);
        }

        while let Ok((window_id, event)) = self.tao_window_event_receiver.try_recv() {
            self.display_manager
                .handle_tao_window_event(&window_id, &event);
        }

        self.runtime_handle.block_on(async {
            let mut join_set = JoinSet::new();

            for sketch in self.sketches.values().filter(|sketch| sketch.is_enabled()) {
                let result_receiver = sketch.request_update().await;

                join_set.spawn(async move {
                    result_receiver
                        .await
                        .expect("Unexpected: error occurred during sketch update")
                });
            }

            while let Some(sketch_errors) = join_set.join_next().await {
                let sketch_errors = sketch_errors
                    .expect("Unexpected: could not join the next sketch update result");

                self.sketches
                    .get_mut(&sketch_errors.id)
                    .expect("Unexpected: could not find sketch")
                    .set_errors(sketch_errors);
            }
        });

        let mut encoder =
            self.wgpu_device
                .create_command_encoder(&nannou::wgpu::CommandEncoderDescriptor {
                    label: Some("Engine texture render encoder"),
                });

        for sketch in self.sketches.values().filter(|sketch| sketch.is_enabled()) {
            if let Some(render_texture_id) = sketch.get_target_render_texture_id() {
                let render_texture = self
                    .render_textures
                    .get_mut(render_texture_id)
                    // TODO: handle error
                    .expect("Engine error: no render texture found for given id!");

                render_texture.render(&sketch.draw(), &mut encoder);
            }
        }

        for plugin in Self::plugins() {
            plugin.engine_render(self, &ENGINE_STORE, &mut encoder);
        }

        self.wgpu_queue.submit(Some(encoder.finish()));

        self.display_manager.render();
    }

    pub fn create_sketch(&mut self, file_path: PathBuf) -> &Sketch {
        let id = SketchId(Uuid::new_v4());

        let sketch = Sketch::new(self.runtime_handle.clone(), id, file_path);

        self.sketches.entry(id).or_insert(sketch)
    }

    // TODO: refactor sketch creation to use builder pattern
    pub fn create_sketch_with_id(&mut self, id: SketchId, file_path: PathBuf) -> SketchId {
        let sketch = Sketch::new(self.runtime_handle.clone(), id, file_path);

        self.sketches.insert(id, sketch);

        id
    }

    pub fn sketches(&self) -> &HashMap<SketchId, Sketch> {
        &self.sketches
    }

    pub fn sketches_mut(&mut self) -> &mut HashMap<SketchId, Sketch> {
        &mut self.sketches
    }

    pub fn create_render_texture(&mut self, width: u32, height: u32) -> &RenderTexture {
        let id = RenderTextureId(Uuid::new_v4());

        let render_texture = self.runtime_handle.block_on(async {
            RenderTexture::new(id, self.wgpu_device.clone(), width, height).await
        });

        self.render_textures.entry(id).or_insert(render_texture)
    }

    pub fn render_textures(&self) -> &HashMap<RenderTextureId, RenderTexture> {
        &self.render_textures
    }

    pub fn render_textures_mut(&mut self) -> &mut HashMap<RenderTextureId, RenderTexture> {
        &mut self.render_textures
    }

    pub fn create_display(&mut self, window: Arc<Window>) -> &Display {
        self.display_manager
            .add_display(&self.wgpu_instance, window)
    }

    pub fn displays(&self) -> &HashMap<DisplayId, Display> {
        &self.display_manager.displays()
    }

    pub fn displays_mut(&mut self) -> &mut HashMap<DisplayId, Display> {
        self.display_manager.displays_mut()
    }

    pub(crate) fn plugins() -> &'static Vec<LoadedPlugin> {
        PLUGINS_CELL
            .get()
            .expect("Unexpected: plugins cell not initialised yet")
    }

    pub fn store(&self) -> &'static Store {
        &ENGINE_STORE
    }

    pub fn wgpu_instance(&self) -> &nannou::wgpu::Instance {
        &self.wgpu_instance
    }

    pub fn wgpu_device(&self) -> &nannou::wgpu::Device {
        &self.wgpu_device
    }

    pub fn wgpu_queue(&self) -> &nannou::wgpu::Queue {
        &self.wgpu_queue
    }
}
