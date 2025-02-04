use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use tao::window::{Window, WindowId};
use tokio::{
    runtime::{Handle, Runtime},
    task::JoinSet,
};
use uuid::Uuid;
use visor_display::display_manager::DisplayManager;
use visor_runtime::startup_snapshot::{StartupSnapshot, STARTUP_SNAPSHOT_CELL};
use visor_wgpu::{handle::WgpuHandle, render_texture::RenderTexture};

use crate::{
    draw::Draw,
    plugin::{load_plugin, LoadedPlugin, Plugin},
    sketch::Sketch,
    sketch_store::SketchStore,
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
    sketch_stores: Option<HashMap<SketchId, SketchStore>>,
    render_textures: HashMap<RenderTextureId, RenderTexture>,
    display_manager: DisplayManager,
    wgpu_handle: Arc<WgpuHandle>,
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
            });

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
                        required_features: nannou::wgpu::Features::empty(),
                        required_limits: nannou::wgpu::Limits::default(),
                        label: None,
                    },
                    None,
                )
                .await
                .expect("Unexpected: could not connect to wgpu device")
        });

        let wgpu_handle = Arc::new(WgpuHandle {
            instance: wgpu_instance,
            device: wgpu_device,
            queue: wgpu_queue,
        });

        let display_manager = DisplayManager::new(runtime_handle.clone(), wgpu_handle.clone());

        let mut engine = Engine {
            _runtime: runtime,
            runtime_handle,
            sketches: Default::default(),
            sketch_stores: Some(Default::default()),
            render_textures: Default::default(),
            display_manager,
            wgpu_handle,
        };

        for plugin in Self::plugins() {
            plugin.build(&mut engine, &ENGINE_STORE);
        }

        println!("[Engine] {} plugins loaded.", Self::plugins().len());

        println!("[Engine] Visor engine ready!");

        engine
    }

    pub fn update(&mut self) {
        let unbuilt_sketch_ids: Vec<SketchId> = self
            .sketches
            .values()
            .filter(|sketch| !sketch.is_built())
            .map(|sketch| *sketch.id())
            .collect();

        if !unbuilt_sketch_ids.is_empty() {
            let mut sketch_stores = self
                .sketch_stores
                .take()
                .expect("Unexpected: sketch stores should be defined!");

            for sketch_id in unbuilt_sketch_ids {
                let store = sketch_stores
                    .get_mut(&sketch_id)
                    .expect("Unexpected: could not find sketch store");

                for plugin in Self::plugins() {
                    plugin.build_sketch(&sketch_id, self, &ENGINE_STORE, store);
                }

                self.sketches
                    .get_mut(&sketch_id)
                    .expect("Unexpected: could not find sketch")
                    .mark_built();
            }

            self.sketch_stores = Some(sketch_stores);
        }

        for plugin in Self::plugins() {
            plugin.before_engine_update(self, &ENGINE_STORE);
        }

        let mut sketch_stores = self
            .sketch_stores
            .take()
            .expect("Unexpected: sketch stores should be defined!");

        self.runtime_handle.block_on(async {
            let mut join_set = JoinSet::new();

            for sketch in self
                .sketches
                .values()
                .filter(|sketch| sketch.is_enabled() && sketch.is_built())
            {
                let store = sketch_stores
                    .remove(sketch.id())
                    .expect("Unexpected: could not find sketch store");

                let result_receiver = sketch.request_update(store);

                join_set.spawn(async move {
                    result_receiver
                        .await
                        .expect("Unexpected: error occurred during sketch update")
                });
            }

            while let Some(result) = join_set.join_next().await {
                let result =
                    result.expect("Unexpected: could not join the next sketch update result");

                self.sketches
                    .get_mut(&result.id)
                    .expect("Unexpected: could not find sketch")
                    .set_errors(result.compile_error, result.runtime_error);

                sketch_stores.insert(result.id, result.store);
            }
        });

        self.sketch_stores = Some(sketch_stores);

        let mut encoder = self.wgpu_handle.device.create_command_encoder(
            &nannou::wgpu::CommandEncoderDescriptor {
                label: Some("Engine texture render encoder"),
            },
        );

        for sketch in self.sketches.values().filter(|sketch| sketch.is_enabled()) {
            if let Some(render_texture_id) = sketch.get_target_render_texture_id() {
                let render_texture = self
                    .render_textures
                    .get_mut(render_texture_id)
                    // TODO: handle error
                    .expect("Engine error: no render texture found for given id!");

                render_texture.render(&sketch.draw().inner, &mut encoder);
            }
        }

        for plugin in Self::plugins() {
            plugin.engine_render(self, &ENGINE_STORE, &mut encoder);
        }

        self.wgpu_handle.queue.submit(Some(encoder.finish()));

        self.display_manager.render();

        for plugin in Self::plugins() {
            plugin.after_engine_update(self, &ENGINE_STORE);
        }
    }

    pub fn create_sketch(&mut self, file_path: PathBuf) -> &Sketch {
        let id = SketchId(Uuid::new_v4());

        let draw = Draw::default();

        let sketch = Sketch::new(self.runtime_handle.clone(), id, file_path, draw.clone());

        let mut store = SketchStore::default();
        store.set(draw);

        self.sketches.insert(id, sketch);

        self.sketch_stores
            .as_mut()
            .expect("Unexpected: sketch stores should be defined!")
            .insert(id, store);

        self.sketches
            .get(&id)
            .expect("Unexpected: sketch not found")
    }

    // TODO: refactor sketch creation to use builder pattern
    pub fn create_sketch_with_id(&mut self, id: SketchId, file_path: PathBuf) -> SketchId {
        let draw = Draw::default();

        let sketch = Sketch::new(self.runtime_handle.clone(), id, file_path, draw.clone());

        let mut store = SketchStore::default();
        store.set(draw);

        self.sketches.insert(id, sketch);

        self.sketch_stores
            .as_mut()
            .expect("Unexpected: sketch stores should be defined!")
            .insert(id, store);

        id
    }

    pub fn remove_sketch(&mut self, id: &SketchId) {
        self.sketches.remove(id);

        self.sketch_stores
            .as_mut()
            .expect("Unexpected: sketch stores should be defined!")
            .remove(id);
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
            RenderTexture::new(self.wgpu_handle.clone(), id, width, height).await
        });

        self.render_textures.entry(id).or_insert(render_texture)
    }

    pub fn render_textures(&self) -> &HashMap<RenderTextureId, RenderTexture> {
        &self.render_textures
    }

    pub fn render_textures_mut(&mut self) -> &mut HashMap<RenderTextureId, RenderTexture> {
        &mut self.render_textures
    }

    // TODO: refactor display creation to use builder pattern
    pub fn create_display(&mut self, window: Arc<Window>) -> &Display {
        let id = DisplayId(Uuid::new_v4());

        self.display_manager.add_display(id, window)
    }

    pub fn create_display_with_id(&mut self, id: DisplayId, window: Arc<Window>) -> &Display {
        self.display_manager.add_display(id, window)
    }

    pub fn manage_display(&mut self, display: Display) -> &Display {
        self.display_manager.manage_display(display)
    }

    pub fn remove_display(&mut self, id: &DisplayId) {
        self.display_manager.remove_display(id)
    }

    pub fn displays(&self) -> &HashMap<DisplayId, Display> {
        self.display_manager.displays()
    }

    pub fn displays_mut(&mut self) -> &mut HashMap<DisplayId, Display> {
        self.display_manager.displays_mut()
    }

    pub fn display_id_for_window_id(&self, window_id: &WindowId) -> &DisplayId {
        self.display_manager.display_id_for_window_id(window_id)
    }

    pub(crate) fn plugins() -> &'static Vec<LoadedPlugin> {
        PLUGINS_CELL
            .get()
            .expect("Unexpected: plugins cell not initialised yet")
    }

    pub fn store(&self) -> &'static Store {
        &ENGINE_STORE
    }

    pub fn sketch_stores(&self) -> &HashMap<SketchId, SketchStore> {
        self.sketch_stores
            .as_ref()
            .expect("Unexpected: sketch stores should be defined!")
    }

    pub fn sketch_stores_mut(&mut self) -> &mut HashMap<SketchId, SketchStore> {
        self.sketch_stores
            .as_mut()
            .expect("Unexpected: sketch stores should be defined!")
    }

    pub fn wgpu_handle(&self) -> &Arc<WgpuHandle> {
        &self.wgpu_handle
    }
}
