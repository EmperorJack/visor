use std::{collections::HashMap, sync::Arc};

use anyhow::{Result, anyhow};
use deno_core::{OpState, op2};
use deno_error::JsErrorBox;
use visor_engine::{AccessSketchStore, Engine, WgpuHandle};

use crate::draw_plugin::{DrawId, FullscreenShaderId, ShapeId, SketchState};

pub(crate) enum FullscreenShader {
    Unloaded,
    Loaded(FullscreenShaderState),
}

pub(crate) struct FullscreenShaderState {
    texture: nannou::wgpu::Texture,
    pub(crate) texture_view: nannou::wgpu::TextureView,
    render_pipeline: nannou::wgpu::RenderPipeline,
    uniforms: HashMap<String, UniformVariable>,
    uniforms_buffer: Vec<u8>,
    uniform_buffer: nannou::wgpu::Buffer,
    uniform_bind_group: nannou::wgpu::BindGroup,
    pub(crate) is_being_drawn: bool,
    has_uniforms_changed: bool,
    wgpu_handle: Arc<WgpuHandle>,
}

struct UniformVariable {
    offset: usize,
    size: usize,
}

const VERTEX_SHADER_SOURCE: &str = include_str!("fullscreen_vertex_shader.wgsl");

impl FullscreenShaderState {
    pub(crate) fn new(
        fragment_shader_source: String,
        engine: &mut Engine,
        width: u32,
        height: u32,
    ) -> Self {
        let device = &engine.wgpu_handle().device;

        let (texture, texture_view) = Self::create_graphics(device, width, height);

        let source = format!("{}\n{}", VERTEX_SHADER_SOURCE, fragment_shader_source);

        let module =
            naga::front::wgsl::parse_str(&source).expect("Unexpected: WGSL must be valid here");

        // TODO: this can fail, need to handle the error
        // Could we have access to the device from within the shader code? Should we?
        let shader = device.create_shader_module(nannou::wgpu::ShaderModuleDescriptor {
            label: Some("Fullscreen Shader"),
            source: nannou::wgpu::ShaderSource::Wgsl(source.into()),
        });

        let mut uniforms = HashMap::new();
        let mut total_struct_size = 0;

        for (_, ty) in module.types.iter() {
            if let naga::TypeInner::Struct { members, span } = &ty.inner {
                if ty.name.as_deref() == Some("Uniforms") {
                    total_struct_size = *span as usize;

                    for member in members {
                        if let Some(ref name) = member.name {
                            uniforms.insert(
                                name.clone(),
                                UniformVariable {
                                    offset: member.offset as usize,
                                    // TODO: test the types other than f32 work
                                    size: match module.types[member.ty].inner {
                                        naga::TypeInner::Scalar { .. } => 4, // f32, u32, i32
                                        naga::TypeInner::Vector { size, .. } => (size as usize) * 4, // vec2, vec3, vec4
                                        _ => 4,
                                    },
                                },
                            );
                        }
                    }
                }
            }
        }

        let padded_size = (total_struct_size + 15) & !15;
        let uniforms_buffer = vec![0; padded_size];

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&nannou::wgpu::BindGroupLayoutDescriptor {
                label: Some("Fullscreen Shader Uniform Bind Group Layout"),
                entries: &[nannou::wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: nannou::wgpu::ShaderStages::FRAGMENT,
                    ty: nannou::wgpu::BindingType::Buffer {
                        ty: nannou::wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let pipeline_layout =
            device.create_pipeline_layout(&nannou::wgpu::PipelineLayoutDescriptor {
                label: Some("Fullscreen Shader Pipeline Layout"),
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let uniform_buffer = device.create_buffer(&nannou::wgpu::BufferDescriptor {
            label: Some("Fullscreen Shader Uniform Buffer"),
            size: padded_size as u64,
            usage: nannou::wgpu::BufferUsages::UNIFORM | nannou::wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_bind_group = device.create_bind_group(&nannou::wgpu::BindGroupDescriptor {
            label: Some("Fullscreen Shader Uniform Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[nannou::wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let render_pipeline =
            device.create_render_pipeline(&nannou::wgpu::RenderPipelineDescriptor {
                label: Some("Fullscreen Shader Render Pipeline"),
                layout: Some(&pipeline_layout),
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
            uniforms,
            uniforms_buffer,
            uniform_buffer,
            uniform_bind_group,
            is_being_drawn: true,
            has_uniforms_changed: false,
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

    // TODO: support uniform value types other than f32
    pub(crate) fn set_uniform(&mut self, key: &str, value: f32) -> Result<()> {
        let uniform = self
            .uniforms
            .get(key)
            .ok_or_else(|| anyhow!("Shader uniform {} not found", key))?;

        let bytes = &value.to_ne_bytes();

        let start = uniform.offset;
        let end = start + uniform.size;
        self.uniforms_buffer[start..end].copy_from_slice(bytes);

        self.has_uniforms_changed = true;

        Ok(())
    }

    pub(crate) fn render(&mut self, encoder: &mut nannou::wgpu::CommandEncoder) {
        if self.has_uniforms_changed {
            self.wgpu_handle
                .queue
                .write_buffer(&self.uniform_buffer, 0, &self.uniforms_buffer);

            self.has_uniforms_changed = false;
        }

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
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
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

#[op2(fast)]
pub(crate) fn op_draw_fullscreen_shader_set_uniform(
    state: &mut OpState,
    id: u32,
    #[string] key: String,
    value: f32,
) -> Result<(), JsErrorBox> {
    let state = state.sketch_store_mut().get_mut::<SketchState>();

    state
        .set_fullscreen_shader_uniform(FullscreenShaderId(id), key, value)
        .map_err(|error| JsErrorBox::generic(error.to_string()))
}
