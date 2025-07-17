use uuid::Uuid;

use crate::{RenderTextureId, engine::Engine, wgpu::render_texture::RenderTexture};

pub struct RenderTextureBuilder {
    id: Option<RenderTextureId>,
    width: u32,
    height: u32,
}

impl RenderTextureBuilder {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            id: None,
            width,
            height,
        }
    }

    pub fn with_id(mut self, id: RenderTextureId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn build(self, engine: &mut Engine) -> &RenderTexture {
        let id = self.id.unwrap_or(RenderTextureId(Uuid::new_v4()));

        let render_texture = engine.runtime_handle.block_on(async {
            RenderTexture::new(engine.wgpu_handle().clone(), id, self.width, self.height).await
        });

        engine.manage_render_texture(render_texture)
    }
}
