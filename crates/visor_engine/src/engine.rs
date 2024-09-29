use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use tao::{
    event::WindowEvent,
    window::{Window, WindowBuilder, WindowId},
};
use tokio::{runtime::Runtime, sync::mpsc, task::JoinSet};
use uuid::Uuid;
use visor_display::display_manager::{DisplayId, DisplayManager};
use visor_runtime::startup_snapshot::{StartupSnapshot, STARTUP_SNAPSHOT_CELL};
use visor_wgpu::render_texture::RenderTexture;

use crate::{
    plugin::Plugin,
    sketch::{Sketch, SketchId},
    store::Store,
};

// TODO: see if there is a more elegant way to achieve this
pub trait WindowCreator {
    fn create_window(&self, window_builder: WindowBuilder) -> Arc<Window>;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RenderTextureId(Uuid);

static PLUGINS_CELL: OnceLock<Vec<Box<dyn Plugin>>> = OnceLock::new();

static ENGINE_STORE: Store = Store::new();

pub struct Engine {
    runtime: Arc<Runtime>,
    sketches: HashMap<SketchId, Sketch>,
    render_textures: HashMap<RenderTextureId, RenderTexture>,
    display_manager: DisplayManager,
    window_creator: Option<Box<dyn WindowCreator>>,
    tao_window_event_sender: mpsc::UnboundedSender<(WindowId, WindowEvent<'static>)>,
    tao_window_event_receiver: mpsc::UnboundedReceiver<(WindowId, WindowEvent<'static>)>,
    wgpu_instance: nannou::wgpu::Instance,
    wgpu_device: nannou::wgpu::Device,
    wgpu_queue: nannou::wgpu::Queue,
}

impl Engine {
    pub fn new(
        runtime: Option<Runtime>,
        window_creator: Option<Box<dyn WindowCreator>>,
        plugins: Vec<Box<dyn Plugin>>,
    ) -> Self {
        let runtime: Arc<Runtime> = runtime
            .unwrap_or(
                tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .expect("Unexpected: could not create tokio runtime"),
            )
            .into();

        let (tao_window_event_sender, tao_window_event_receiver) = mpsc::unbounded_channel();

        STARTUP_SNAPSHOT_CELL.get_or_init(|| {
            let plugin_extensions = plugins.iter().map(|plugin| plugin.extension()).collect();

            StartupSnapshot::new(plugin_extensions)
        });

        PLUGINS_CELL.get_or_init(|| plugins);

        let wgpu_instance = nannou::wgpu::Instance::default();

        let (wgpu_device, wgpu_queue) = runtime.block_on(async {
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

        let mut engine = Engine {
            runtime: runtime.clone(),
            sketches: Default::default(),
            render_textures: Default::default(),
            display_manager: DisplayManager::new(runtime),
            window_creator,
            tao_window_event_sender,
            tao_window_event_receiver,
            wgpu_instance,
            wgpu_device,
            wgpu_queue,
        };

        for plugin in Self::plugins() {
            plugin.build(&mut engine, Self::store());
        }

        engine
    }

    pub fn tao_window_event_sender(
        &self,
    ) -> mpsc::UnboundedSender<(WindowId, WindowEvent<'static>)> {
        self.tao_window_event_sender.clone()
    }

    pub fn update(&mut self) {
        for plugin in Self::plugins() {
            plugin.engine_update(self, Self::store());
        }

        while let Ok((window_id, event)) = self.tao_window_event_receiver.try_recv() {
            self.display_manager
                .handle_tao_window_event(&window_id, &event);
        }

        self.runtime.block_on(async {
            let mut join_set = JoinSet::new();

            for sketch in self.sketches.values() {
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

        for sketch in self.sketches.values() {
            if let Some(render_texture_id) = sketch.target_render_texture_id() {
                let render_texture = self
                    .render_textures
                    .get_mut(render_texture_id)
                    // TODO: handle error
                    .expect("Engine error: no render texture found for given id!");

                render_texture.render(&sketch.get_draw().inner, &self.wgpu_device, &mut encoder);
            }
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

        self.runtime.block_on(async {
            let result_receiver = sketch.request_compile().await;

            result_receiver
                .await
                .expect("Unexpected: error occurred during sketch compile");
        });
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
            .runtime
            .block_on(async { RenderTexture::new(&self.wgpu_device, width, height).await });

        self.render_textures.insert(id, render_texture);

        id
    }

    pub fn create_display(&mut self, title: String, width: u32, height: u32) -> DisplayId {
        if let Some(window_creator) = &self.window_creator {
            let window_builder = WindowBuilder::new()
                .with_title(title)
                .with_inner_size(tao::dpi::PhysicalSize::new(width, height));

            let window = window_creator.create_window(window_builder);

            self.display_manager
                .add_display(&self.wgpu_instance, window)
        } else {
            panic!("Engine error: cannot create display without a window creator, make sure to call with_window_creator when building the engine!")
        }
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

    pub(crate) fn plugins() -> &'static Vec<Box<dyn Plugin>> {
        PLUGINS_CELL
            .get()
            .expect("Unexpected: plugins cell not initialised yet")
    }

    pub(crate) fn store() -> &'static Store {
        &ENGINE_STORE
    }

    pub fn get_from_store<T: Send + Sync + 'static>(&self) -> &T {
        Self::store().get()
    }
}
