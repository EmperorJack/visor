use std::sync::Arc;

use nannou::wgpu::TextureView;
use tao::{
    dpi::PhysicalSize,
    platform::macos::WindowExtMacOS,
    window::{Window, WindowId},
};
use visor_wgpu::display::Display as WgpuDisplay;

pub struct Display {
    window: Arc<Window>,
    wgpu_display: WgpuDisplay,
}

impl Display {
    pub async fn new(wgpu_instance: &nannou::wgpu::Instance, window: Arc<Window>) -> Self {
        let size = window.inner_size();

        let wgpu_display = WgpuDisplay::new(wgpu_instance, &window, size.width, size.height).await;

        Self {
            window,
            wgpu_display,
        }
    }

    pub fn window_id(&self) -> WindowId {
        self.window.id()
    }

    pub fn focus(&self) {
        self.window.set_focus();
    }

    pub fn is_fullscreen(&self) -> bool {
        #[cfg(target_os = "macos")]
        return self.window.simple_fullscreen();

        #[cfg(not(target_os = "macos"))]
        {
            self.window.fullscreen().is_some()
        }
    }

    pub fn set_fullscreen(&self, enabled: bool) {
        // Note: simple fullscreen hides the menu bar in other spaces, which is not great for a VJing app
        // TODO: open an issue with the tao library
        #[cfg(target_os = "macos")]
        self.window.set_simple_fullscreen(enabled);

        #[cfg(not(target_os = "macos"))]
        self.window.set_fullscreen(if enabled {
            Some(tao::window::Fullscreen::Borderless(None))
        } else {
            None
        });
    }

    pub fn handle_resize(&mut self, size: PhysicalSize<u32>) {
        self.wgpu_display.resize(size.width, size.height);
    }

    pub fn set_source_texture(&mut self, texture_view: Option<&TextureView>) {
        self.wgpu_display.set_source_texture(texture_view);
    }

    pub fn render(&mut self) {
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
