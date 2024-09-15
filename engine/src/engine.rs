use std::{collections::HashMap, sync::Arc};

use display::display_manager::{DisplayId, DisplayManager};
use tao::window::{Window, WindowBuilder};
use tokio::runtime::Runtime;
use uuid::Uuid;
use wgpu::{instance::Instance, render_texture::RenderTexture};

use crate::stats::Stats;

// TODO: see if there is a more elegant way to achieve this
pub trait WindowCreator {
    fn create_window(&self, window_builder: WindowBuilder) -> Arc<Window>;
}

#[derive(Default)]
pub struct EngineBuilder {
    runtime: Option<Runtime>,
    window_creator: Option<Box<dyn WindowCreator>>,
}

impl EngineBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_runtime(mut self, runtime: Runtime) -> Self {
        self.runtime = Some(runtime);
        self
    }

    pub fn with_window_creator(mut self, callback: Box<dyn WindowCreator>) -> Self {
        self.window_creator = Some(callback);
        self
    }

    pub fn build(self) -> Engine {
        let runtime: Arc<Runtime> = self
            .runtime
            .unwrap_or(
                tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .expect("Unexpected: could not create tokio runtime"),
            )
            .into();

        Engine {
            runtime: runtime.clone(),
            render_textures: Default::default(),
            display_manager: DisplayManager::new(runtime),
            window_creator: self.window_creator,
            stats: Stats::new(),
            wgpu_instance: Instance::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RenderTextureId(Uuid);

pub struct Engine {
    runtime: Arc<Runtime>,
    render_textures: HashMap<RenderTextureId, RenderTexture>,
    display_manager: DisplayManager,
    window_creator: Option<Box<dyn WindowCreator>>,
    stats: Stats,
    wgpu_instance: Instance,
}

impl Engine {
    pub fn update(&mut self) {
        self.stats.before_update();

        self.display_manager.render();

        self.stats.after_update();
    }

    pub fn create_render_texture(&mut self, width: u32, height: u32) -> RenderTextureId {
        let id = RenderTextureId(Uuid::new_v4());

        let render_texture = self
            .runtime
            .block_on(async { RenderTexture::new(&self.wgpu_instance, width, height).await });

        self.render_textures.insert(id, render_texture);

        id
    }

    pub fn create_display(
        &mut self,
        title: String,
        width: u32,
        height: u32,
        render_texture_id: &RenderTextureId,
    ) -> DisplayId {
        let render_texture = self
            .render_textures
            .get(render_texture_id)
            .expect("Engine error: no render texture found for given id!"); // TODO: handle error

        if let Some(window_creator) = &self.window_creator {
            let window_builder = WindowBuilder::new()
                .with_title(title)
                .with_inner_size(tao::dpi::PhysicalSize::new(width, height));

            let window = window_creator.create_window(window_builder);

            self.display_manager.add_display(
                &self.wgpu_instance,
                window,
                render_texture.texture_view(),
            )
        } else {
            panic!("Engine error: cannot create display without a window creator, make sure to call with_window_creator when building the engine!")
        }
    }
}
