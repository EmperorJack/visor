use std::{collections::HashMap, sync::Arc};

use deno_core::{OpState, op2};
use deno_error::JsErrorBox;
use visor_engine::{AccessSketchStore, Engine, WgpuHandle};

use crate::draw_plugin::{DrawId, FullscreenShaderId, ShapeId, SketchState};

pub(crate) struct FullscreenShader {
    texture: nannou::wgpu::Texture,
    pub(crate) texture_view: nannou::wgpu::TextureView,
    render_pipeline: nannou::wgpu::RenderPipeline,
    pub(crate) is_being_drawn: bool,
    wgpu_handle: Arc<WgpuHandle>,
}

const VERTEX_SHADER_SOURCE: &str = include_str!("fullscreen_vertex_shader.wgsl");

impl FullscreenShader {
    pub(crate) fn new(
        fragment_shader_source: String,
        engine: &mut Engine,
        width: u32,
        height: u32,
    ) -> Self {
        let device = &engine.wgpu_handle().device;

        let (texture, texture_view) = Self::create_graphics(device, width, height);

        let source = format!("{}\n{}", VERTEX_SHADER_SOURCE, fragment_shader_source);

        let shader = device.create_shader_module(nannou::wgpu::ShaderModuleDescriptor {
            label: Some("Fullscreen Shader"),
            source: nannou::wgpu::ShaderSource::Wgsl(source.into()),
        });

        let render_pipeline =
            device.create_render_pipeline(&nannou::wgpu::RenderPipelineDescriptor {
                label: Some("Fullscreen Shader Render Pipeline"),
                layout: None,
                vertex: nannou::wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[],
                    compilation_options: Default::default(),
                },
                fragment: Some(nannou::wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(nannou::wgpu::ColorTargetState {
                        format: texture_view.format(),
                        blend: Some(nannou::wgpu::BlendState::REPLACE),
                        write_mask: nannou::wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                primitive: nannou::wgpu::PrimitiveState {
                    cull_mode: None,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: Default::default(),
                multiview: None,
            });

        Self {
            texture,
            texture_view,
            render_pipeline,
            is_being_drawn: true,
            wgpu_handle: engine.wgpu_handle().clone(),
        }
    }

    fn create_graphics(
        device: &nannou::wgpu::Device,
        width: u32,
        height: u32,
    ) -> (nannou::wgpu::Texture, nannou::wgpu::TextureView) {
        let texture = nannou::wgpu::TextureBuilder::new()
            .size([width, height])
            .usage(
                nannou::wgpu::TextureUsages::RENDER_ATTACHMENT
                    | nannou::wgpu::TextureUsages::TEXTURE_BINDING,
            )
            .sample_count(1)
            .format(nannou::wgpu::TextureFormat::Rgba16Float)
            .build(device);

        let texture_view = texture.view().build();

        (texture, texture_view)
    }

    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        (self.texture, self.texture_view) =
            Self::create_graphics(&self.wgpu_handle.device, width, height);
    }

    pub(crate) fn render(&self, encoder: &mut nannou::wgpu::CommandEncoder) {
        let mut render_pass = encoder.begin_render_pass(&nannou::wgpu::RenderPassDescriptor {
            label: Some("Fullscreen Shader Render Pass"),
            color_attachments: &[Some(nannou::wgpu::RenderPassColorAttachment {
                view: &self.texture_view,
                resolve_target: None,
                ops: nannou::wgpu::Operations {
                    load: nannou::wgpu::LoadOp::Clear(nannou::wgpu::Color::TRANSPARENT),
                    store: nannou::wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.draw(0..3, 0..1)
    }
}

pub(crate) enum FullscreenShaderEvent {
    Load {
        id: FullscreenShaderId,
        source: String,
        width: u32,
        height: u32,
    },
}

pub(crate) type FullscreenShaderCommandMap = HashMap<ShapeId, (DrawId, FullscreenShaderId)>;

#[op2(fast)]
pub(crate) fn op_draw_fullscreen_shader(state: &mut OpState, id: u32, shader_id: u32) {
    let state = state.sketch_store_mut().get_mut::<SketchState>();

    state.start_drawing_shader(DrawId(id), FullscreenShaderId(shader_id));
}

#[op2(fast)]
pub(crate) fn op_draw_fullscreen_shader_load(
    state: &mut OpState,
    #[string] path: String,
) -> Result<u32, JsErrorBox> {
    let state = state.sketch_store_mut().get_mut::<SketchState>();

    state
        .load_fullscreen_shader(path)
        .map(|shader_id| shader_id.0)
        .map_err(|error| JsErrorBox::generic(error.to_string()))
}
