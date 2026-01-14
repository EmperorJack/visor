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
        let (texture, renderer) =
            Self::create_graphics(&wgpu_handle.device, width, height, sample_count);

        Self {
            id,
            texture,
            renderer,
            wgpu_handle,
        }
    }

    fn create_graphics(
        device: &nannou::wgpu::Device,
        width: u32,
        height: u32,
        sample_count: u32,
    ) -> (nannou::wgpu::Texture, nannou::draw::Renderer) {
        let format = nannou::wgpu::TextureFormat::Rgba16Float;

        let texture = nannou::wgpu::TextureBuilder::new()
            .size([width, height])
            .usage(
                nannou::wgpu::TextureUsages::RENDER_ATTACHMENT
                    | nannou::wgpu::TextureUsages::TEXTURE_BINDING,
            )
            .sample_count(sample_count)
            .format(format)
            .build(device);

        let renderer = nannou::draw::RendererBuilder::new()
            .build_from_texture_descriptor(device, texture.descriptor());

        (texture, renderer)
    }

    pub fn id(&self) -> &RenderTextureId {
        &self.id
    }

    pub fn texture(&self) -> &nannou::wgpu::Texture {
        &self.texture
    }

    pub fn texture_view(&self) -> nannou::wgpu::TextureView {
        self.texture.view().build()
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        (self.texture, self.renderer) = Self::create_graphics(
            &self.wgpu_handle.device,
            width,
            height,
            self.texture().sample_count(),
        );
    }

    pub fn render(&mut self, draw: &nannou::Draw, encoder: &mut nannou::wgpu::CommandEncoder) {
        self.renderer
            .render_to_texture(&self.wgpu_handle.device, encoder, draw, &self.texture);
    }
}
