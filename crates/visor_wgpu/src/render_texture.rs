use std::sync::Arc;

use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RenderTextureId(pub Uuid);

pub struct RenderTexture {
    id: RenderTextureId,
    texture: nannou::wgpu::Texture,
    renderer: nannou::draw::Renderer,
    device: Arc<nannou::wgpu::Device>,
}

impl RenderTexture {
    pub async fn new(
        id: RenderTextureId,
        device: Arc<nannou::wgpu::Device>,
        width: u32,
        height: u32,
    ) -> Self {
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
            id,
            texture,
            renderer,
            device,
        }
    }

    pub fn id(&self) -> &RenderTextureId {
        &self.id
    }

    pub fn texture_view(&self) -> nannou::wgpu::TextureView {
        self.texture.view().build()
    }

    pub fn render(&mut self, draw: &nannou::Draw, encoder: &mut nannou::wgpu::CommandEncoder) {
        self.renderer
            .render_to_texture(&self.device, encoder, draw, &self.texture);
    }
}
