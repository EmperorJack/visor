use std::sync::Arc;

use tao::rwh_06;

use crate::wgpu::handle::WgpuHandle;

#[derive(Debug)]
pub struct WgpuDisplay {
    surface: nannou::wgpu::Surface<'static>,
    surface_format: nannou::wgpu::TextureFormat,
    surface_config: nannou::wgpu::SurfaceConfiguration,
    source_texture_reshaper: Option<nannou::wgpu::TextureReshaper>,
    wgpu_handle: Arc<WgpuHandle>,
}

impl WgpuDisplay {
    pub async fn new<W>(wgpu: Arc<WgpuHandle>, window: W, width: u32, height: u32) -> Self
    where
        W: rwh_06::HasWindowHandle + rwh_06::HasDisplayHandle + Send + Sync + 'static,
    {
        let surface = wgpu
            .instance
            .create_surface(window)
            .expect("Unexpeced: could not create wgpu surface");

        let adapter = wgpu
            .instance
            .request_adapter(&nannou::wgpu::RequestAdapterOptions {
                power_preference: nannou::wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Unexpected: could not request wgpu adapter");

        let surface_capabilities = surface.get_capabilities(&adapter);

        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);

        let surface_config = nannou::wgpu::SurfaceConfiguration {
            usage: nannou::wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&wgpu.device, &surface_config);

        Self {
            surface,
            surface_format,
            surface_config,
            source_texture_reshaper: None,
            wgpu_handle: wgpu,
        }
    }

    pub fn set_source_texture(&mut self, texture_view: Option<&nannou::wgpu::TextureView>) {
        self.source_texture_reshaper = texture_view.map(|texture_view| {
            nannou::wgpu::TextureReshaper::new(
                &self.wgpu_handle.device,
                texture_view,
                1,
                texture_view.sample_type(),
                1,
                self.surface_format,
            )
        });
    }

    pub fn render(&self) -> Result<(), nannou::wgpu::SurfaceError> {
        if let Some(source_texture_reshaper) = &self.source_texture_reshaper {
            return self.surface.get_current_texture().map(|surface_texture| {
                let mut encoder = self.wgpu_handle.device.create_command_encoder(
                    &nannou::wgpu::CommandEncoderDescriptor {
                        label: Some("Display surface texture render encoder"),
                    },
                );

                let surface_texture_view = surface_texture
                    .texture
                    .create_view(&nannou::wgpu::TextureViewDescriptor::default());

                source_texture_reshaper.encode_render_pass(&surface_texture_view, &mut encoder);

                self.wgpu_handle.queue.submit(Some(encoder.finish()));

                surface_texture.present();
            });
        };

        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;

            self.surface
                .configure(&self.wgpu_handle.device, &self.surface_config);
        }
    }
}
