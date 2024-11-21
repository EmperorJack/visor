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

pub use visor_display::display_manager::DisplayId;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RenderTextureId(Uuid);

static PLUGINS_CELL: OnceLock<Vec<LoadedPlugin>> = OnceLock::new();

pub struct Engine {
    _runtime: Option<Runtime>,
    runtime_handle: Arc<Handle>,
    sketches: HashMap<SketchId, Sketch>,
    render_textures: HashMap<RenderTextureId, RenderTexture>,
    display_manager: DisplayManager,
    tao_window_event_sender: mpsc::UnboundedSender<(WindowId, WindowEvent<'static>)>,
    tao_window_event_receiver: mpsc::UnboundedReceiver<(WindowId, WindowEvent<'static>)>,
    wgpu_instance: nannou::wgpu::Instance,
    wgpu_device: nannou::wgpu::Device,
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
            .map(|runtime_handle| (None, Arc::new(runtime_handle)))
            .unwrap_or_else(|| {
                let runtime = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .expect("Unexpected: could not create tokio runtime");

                let runtime_handle = runtime.handle().clone();

                (Some(runtime), Arc::new(runtime_handle))
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
            wgpu_device,
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
                        .expect("Unexpected: error occurred during sketch update");
                });
            }

            while (join_set.join_next().await).is_some() {}
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

                render_texture.render(&sketch.get_draw(), &self.wgpu_device, &mut encoder);
            }
        }

        for plugin in Self::plugins() {
            plugin.engine_render(self, &ENGINE_STORE, &mut encoder);
        }

        self.wgpu_queue.submit(Some(encoder.finish()));

        self.display_manager.render();
    }

    pub fn create_sketch(&mut self, file_path: PathBuf) -> SketchId {
        let id = SketchId(Uuid::new_v4());

        let sketch = Sketch::new(id, file_path);

        self.sketches.insert(id, sketch);

        id
    }

    pub fn recompile_sketch(&mut self, sketch_id: &SketchId) {
        let sketch = self
            .sketches
            .get_mut(sketch_id)
            .expect("Engine error: no sketch found for given id!"); // TODO: handle error

        self.runtime_handle.block_on(async {
            let result_receiver = sketch.request_compile().await;

            result_receiver
                .await
                .expect("Unexpected: error occurred during sketch compile");
        });
    }

    pub fn get_sketches(&self) -> &HashMap<SketchId, Sketch> {
        &self.sketches
    }

    pub fn set_sketch_enabled(&mut self, sketch_id: &SketchId, is_enabled: bool) {
        let sketch = self
            .sketches
            .get_mut(sketch_id)
            .expect("Engine error: no sketch found for given id!"); // TODO: handle error

        sketch.set_enabled(is_enabled);
    }

    pub fn set_sketch_target_render_texture_id(
        &mut self,
        sketch_id: &SketchId,
        render_texture_id: Option<&RenderTextureId>,
    ) {
        let sketch = self
            .sketches
            .get_mut(sketch_id)
            .expect("Engine error: no sketch found for given id!"); // TODO: handle error

        if let Some(render_texture_id) = render_texture_id {
            if !self.render_textures.contains_key(render_texture_id) {
                panic!("Engine error: no render texture found for given id!"); // TODO: handle error
            }
        }

        sketch.set_target_render_texture_id(render_texture_id);
    }

    pub fn create_render_texture(&mut self, width: u32, height: u32) -> RenderTextureId {
        let id = RenderTextureId(Uuid::new_v4());

        let render_texture = self
            .runtime_handle
            .block_on(async { RenderTexture::new(&self.wgpu_device, width, height).await });

        self.render_textures.insert(id, render_texture);

        id
    }

    pub fn render_to_texture(
        &mut self,
        render_texture_id: &RenderTextureId,
        draw: &nannou::Draw,
        encoder: &mut nannou::wgpu::CommandEncoder,
    ) {
        let render_texture = self
            .render_textures
            .get_mut(render_texture_id)
            // TODO: handle error
            .expect("Engine error: no render texture found for given id!");

        render_texture.render(draw, &self.wgpu_device, encoder);
    }

    pub fn create_display(&mut self, window: Arc<Window>) -> DisplayId {
        self.display_manager
            .add_display(&self.wgpu_instance, window)
    }

    pub fn set_display_source_texture(
        &mut self,
        display_id: &DisplayId,
        render_texture_id: Option<&RenderTextureId>,
    ) {
        let render_texture_view = render_texture_id.map(|render_texture_id| {
            self.render_textures
                .get(render_texture_id)
                .expect("Engine error: no render texture found for given id!") // TODO: handle error
                .texture_view()
        });

        self.display_manager
            .set_display_source_texture(display_id, render_texture_view.as_ref());
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
