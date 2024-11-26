use std::sync::Arc;

use nannou::wgpu::TextureView;
use tao::{dpi::PhysicalSize, window::Window};
use uuid::Uuid;
use visor_wgpu::display::Display as WgpuDisplay;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct DisplayId(pub Uuid);

pub struct Display {
    id: DisplayId,
    window: Arc<Window>,
    wgpu_display: WgpuDisplay,
}

impl Display {
    pub async fn new(
        id: DisplayId,
        wgpu_instance: &nannou::wgpu::Instance,
        window: Arc<Window>,
    ) -> Self {
        let size = window.inner_size();

        let wgpu_display = WgpuDisplay::new(wgpu_instance, &window, size.width, size.height).await;

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

    pub(crate) fn render(&mut self) {
        match self.wgpu_display.render() {
            Ok(()) => {}
            Err(nannou::wgpu::SurfaceError::Lost) => {
                eprintln!("[Engine] Surface error: display surface texture lost!");

                let size = self.window.inner_size();
                self.wgpu_display.resize(size.width, size.height);
            }
            Err(nannou::wgpu::SurfaceError::OutOfMemory) => {
                eprintln!("[Engine] Surface error: out of memory!");

                panic!("Surface error: out of memory!")
            }
            Err(e) => {
                eprintln!("[Engine] Surface error: {:?}", e);
            }
        }
    }
}
