use uuid::Uuid;

use crate::{RenderTextureId, engine::Engine, wgpu::render_texture::RenderTexture};

pub struct RenderTextureBuilder {
    id: Option<RenderTextureId>,
    width: u32,
    height: u32,
    sample_count: Option<u32>,
}

impl RenderTextureBuilder {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            id: None,
            width,
            height,
            sample_count: None,
        }
    }

    pub fn with_id(mut self, id: RenderTextureId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_sample_count(mut self, sample_count: u32) -> Self {
        self.sample_count = Some(sample_count);
        self
    }

    pub fn build(self, engine: &mut Engine) -> &RenderTexture {
        let id = self.id.unwrap_or(RenderTextureId(Uuid::new_v4()));

        let sample_count = self.sample_count.unwrap_or(1);

        let render_texture = engine.runtime_handle.block_on(async {
            RenderTexture::new(
                engine.wgpu_handle().clone(),
                id,
                self.width,
                self.height,
                sample_count,
            )
            .await
        });

        engine.manage_render_texture(render_texture)
    }
}
