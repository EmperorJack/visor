use crate::instance::Instance;

pub struct RenderTexture {
    device: nannou::wgpu::Device,
    queue: nannou::wgpu::Queue,
    texture: nannou::wgpu::Texture,
    renderer: nannou::draw::Renderer,
}

impl RenderTexture {
    pub async fn new(instance: &Instance, width: u32, height: u32) -> Self {
        let adapter = instance
            .request_adapter(&nannou::wgpu::RequestAdapterOptions {
                power_preference: nannou::wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .expect("Unexpected: could not request wgpu adapter");

        let (device, queue) = adapter
            .request_device(
                &nannou::wgpu::DeviceDescriptor {
                    features: nannou::wgpu::Features::empty(),
                    limits: nannou::wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .expect("Unexpected: could not connect to wgpu device");

        let format = nannou::wgpu::TextureFormat::Rgba16Float;

        let texture = nannou::wgpu::TextureBuilder::new()
            .size([width, height])
            .usage(
                nannou::wgpu::TextureUsages::RENDER_ATTACHMENT
                    | nannou::wgpu::TextureUsages::TEXTURE_BINDING,
            )
            .sample_count(1)
            .format(format)
            .build(&device);

        let renderer = nannou::draw::RendererBuilder::new()
            .build_from_texture_descriptor(&device, texture.descriptor());

        Self {
            device,
            queue,
            texture,
            renderer,
        }
    }

    pub fn texture_view(&self) -> nannou::wgpu::TextureView {
        self.texture.view().build()
    }

    pub fn render(&mut self, draw: &nannou::Draw, mut encoder: nannou::wgpu::CommandEncoder) {
        self.renderer
            .render_to_texture(&self.device, &mut encoder, draw, &self.texture);

        self.queue.submit(Some(encoder.finish()));
    }
}
