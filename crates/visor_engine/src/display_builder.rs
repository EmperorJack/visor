use std::sync::Arc;

use tao::window::Window;
use uuid::Uuid;

use crate::{
    display::{Display, DisplayId},
    engine::Engine,
    wgpu::handle::WgpuHandle,
};

pub struct DisplayBuilder {
    id: Option<DisplayId>,
    window: Arc<Window>,
}

impl DisplayBuilder {
    pub fn new(window: Arc<Window>) -> Self {
        Self { id: None, window }
    }

    pub fn with_id(mut self, id: DisplayId) -> Self {
        self.id = Some(id);
        self
    }

    async fn build_display(self, wgpu_handle: Arc<WgpuHandle>) -> Display {
        let id = self.id.unwrap_or(DisplayId(Uuid::new_v4()));

        Display::new(wgpu_handle, id, self.window).await
    }

    pub fn build(self, engine: &mut Engine) -> &Display {
        let display = engine
            .runtime_handle
            .block_on(async { self.build_display(engine.wgpu_handle().clone()).await });

        engine.manage_display(display)
    }

    pub async fn build_raw(self, wgpu_handle: Arc<WgpuHandle>) -> Display {
        self.build_display(wgpu_handle).await
    }
}
