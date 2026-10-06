use std::sync::Arc;

use nannou::wgpu::TextureView;
use tao::{dpi::PhysicalSize, window::Window};
use uuid::Uuid;

use crate::wgpu::{display::WgpuDisplay, handle::WgpuHandle};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DisplayId(pub Uuid);

#[derive(Debug)]
pub struct Display {
    id: DisplayId,
    window: Arc<Window>,
    wgpu_display: WgpuDisplay,
}

impl Display {
    pub(crate) async fn new(
        wgpu_handle: Arc<WgpuHandle>,
        id: DisplayId,
        window: Arc<Window>,
    ) -> Self {
        let size = window.inner_size();

        let wgpu_display =
            WgpuDisplay::new(wgpu_handle, window.clone(), size.width, size.height).await;

        Self {
            id,
            window,
            wgpu_display,
        }
    }

    pub fn id(&self) -> &DisplayId {
        &self.id
    }

    pub fn window(&self) -> &Arc<Window> {
        &self.window
    }

    pub fn resize_surface(&mut self, size: PhysicalSize<u32>) {
        self.wgpu_display.resize(size.width, size.height);
    }

    pub fn set_source_texture(&mut self, texture_view: Option<&TextureView>) {
        self.wgpu_display.set_source_texture(texture_view);
    }

    pub(crate) fn render(&self) {
        self.wgpu_display.render();
    }
}
