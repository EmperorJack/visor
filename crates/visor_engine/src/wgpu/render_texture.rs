use std::sync::Arc;

use uuid::Uuid;

use crate::wgpu::handle::WgpuHandle;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RenderTextureId(pub Uuid);

pub struct RenderTexture {
    id: RenderTextureId,
    texture: nannou::wgpu::Texture,
    renderer: nannou::draw::Renderer,
    wgpu_handle: Arc<WgpuHandle>,
}

impl RenderTexture {
    pub(crate) async fn new(
        wgpu_handle: Arc<WgpuHandle>,
        id: RenderTextureId,
        width: u32,
        height: u32,
        sample_count: u32,
    ) -> Self {
        let format = nannou::wgpu::TextureFormat::Rgba16Float;

        let texture = nannou::wgpu::TextureBuilder::new()
            .size([width, height])
            .usage(
                nannou::wgpu::TextureUsages::RENDER_ATTACHMENT
                    | nannou::wgpu::TextureUsages::TEXTURE_BINDING,
            )
            .sample_count(sample_count)
            .format(format)
            .build(&wgpu_handle.device);

        let renderer = nannou::draw::RendererBuilder::new()
            .build_from_texture_descriptor(&wgpu_handle.device, texture.descriptor());

        Self {
            id,
            texture,
            renderer,
            wgpu_handle,
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
            .render_to_texture(&self.wgpu_handle.device, encoder, draw, &self.texture);
    }
}
