use std::sync::Arc;

use display::display::Display;
use tao::window::{Window, WindowBuilder};
use tokio::runtime::Runtime;
use wgpu::{instance::Instance, render_texture::RenderTexture};

use crate::stats::Stats;

#[derive(Default)]
pub struct EngineBuilder {
    runtime: Option<Runtime>,
}

impl EngineBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_runtime(mut self, runtime: Runtime) -> Self {
        self.runtime = Some(runtime);
        self
    }

    pub fn build(self) -> Engine {
        let runtime = self.runtime.unwrap_or(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Unexpected: could not create tokio runtime"),
        );

        Engine::new(runtime)
    }
}

pub struct Engine {
    runtime: Runtime,
    displays: Vec<Display>,
    stats: Stats,
    wgpu_instance: Instance,
}

impl Engine {
    fn new(runtime: Runtime) -> Self {
        Self {
            runtime,
            displays: Vec::new(),
            stats: Stats::new(),
            wgpu_instance: Instance::default(),
        }
    }

    pub fn update(&mut self) {
        self.stats.before_update();

        for display in &mut self.displays {
            display.render();
        }

        self.stats.after_update();
    }

    pub fn create_render_texture(&self, width: u32, height: u32) -> RenderTexture {
        // TODO: store render textures in engine struct
        self.runtime
            .block_on(async { RenderTexture::new(&self.wgpu_instance, width, height).await })
    }

    pub fn create_display<F>(
        &mut self,
        title: String,
        width: u32,
        height: u32,
        render_texture: &RenderTexture,
        create_window_callback: F,
    ) where
        F: Fn(WindowBuilder) -> Arc<Window>,
    {
        let window_builder = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(tao::dpi::PhysicalSize::new(width, height));

        let window = create_window_callback(window_builder);

        let display = self
            .runtime
            .block_on(async {
                Display::new(&self.wgpu_instance, window, render_texture.texture_view()).await
            })
            .into();

        self.displays.push(display);
    }
}
