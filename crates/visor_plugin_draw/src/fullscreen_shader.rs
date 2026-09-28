use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
};

use anyhow::{Result, anyhow};
use deno_core::{OpState, op2, v8};
use deno_error::JsErrorBox;
use visor_engine::{AccessSketchStore, WgpuHandle};

use crate::draw_plugin::{ShapeId, ShapeType, SketchState};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct FullscreenShaderId(pub(crate) u32);

impl FullscreenShaderId {
    fn new(path: &str) -> Self {
        let mut hasher = std::hash::DefaultHasher::new();

        path.hash(&mut hasher);

        FullscreenShaderId(hasher.finish() as u32)
    }
}

pub(crate) struct FullscreenShader {
    texture: nannou::wgpu::Texture,
    pub(crate) texture_view: nannou::wgpu::TextureView,
    render_pipeline: nannou::wgpu::RenderPipeline,
    uniforms_meta: HashMap<String, UniformMeta>,
    uniforms_buffer: Vec<u8>,
    uniform_buffer: nannou::wgpu::Buffer,
    uniform_bind_group: nannou::wgpu::BindGroup,
    pub(crate) is_being_drawn: bool,
    has_uniforms_changed: bool,
    wgpu_handle: Arc<WgpuHandle>,
}

struct UniformMeta {
    offset: usize,
    wgsl_type: UniformType,
}

enum ScalarType {
    F32,
    U32,
    I32,
}

enum UniformType {
    Scalar(ScalarType),
    Vector {
        scalar_type: ScalarType,
        size: naga::VectorSize,
    },
    MatrixF32 {
        columns: naga::VectorSize,
        rows: naga::VectorSize,
    },
    ArrayOfVector4 {
        scalar_type: ScalarType,
        length: u32,
    },
}

const VERTEX_SHADER_SOURCE: &str = include_str!("fullscreen_vertex_shader.wgsl");

impl FullscreenShader {
    pub(crate) fn new(
        path: String,
        width: u32,
        height: u32,
        wgpu_handle: &Arc<WgpuHandle>,
    ) -> Result<Self> {
        let device = &wgpu_handle.device;

        let (texture, texture_view) = Self::create_graphics(device, width, height);

        let fragment_shader_source = std::fs::read_to_string(&path)
            .map_err(|_| anyhow!("Could not load fullscreen shader at path {}", path))?;

        let source = format!("{}\n{}", VERTEX_SHADER_SOURCE, fragment_shader_source);

        let module = naga::front::wgsl::parse_str(&source)
            .map_err(|error| anyhow!("Could not parse WGSL shader: {}", error.message()))?;

        let mut validator = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::default(),
        );

        validator.validate(&module).map_err(|err| {
            anyhow!(
                "WGSL shader validation error: {}",
                err.emit_to_string(&source)
            )
        })?;

        let shader = device.create_shader_module(nannou::wgpu::ShaderModuleDescriptor {
            label: Some("Fullscreen Shader"),
            source: nannou::wgpu::ShaderSource::Wgsl(source.into()),
        });

        let (uniforms_meta, total_struct_size) = Self::parse_uniforms_meta(module)?;

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

        Ok(Self {
            texture,
            texture_view,
            render_pipeline,
            uniforms_meta,
            uniforms_buffer,
            uniform_buffer,
            uniform_bind_group,
            is_being_drawn: true,
            has_uniforms_changed: false,
            wgpu_handle: wgpu_handle.clone(),
        })
    }

    fn parse_uniforms_meta(module: naga::Module) -> Result<(HashMap<String, UniformMeta>, usize)> {
        let mut uniforms_meta = HashMap::new();
        let mut total_struct_size = 0;

        for (_, ty) in module.types.iter() {
            if let naga::TypeInner::Struct { members, span } = &ty.inner {
                if ty.name.as_deref() == Some("Uniforms") {
                    total_struct_size = *span as usize;

                    for member in members {
                        if let Some(ref name) = member.name {
                            let offset = member.offset as usize;
                            let wgsl_type = match &module.types[member.ty].inner {
                                naga::TypeInner::Scalar(scalar) => match scalar.kind {
                                    naga::ScalarKind::Sint => UniformType::Scalar(ScalarType::I32),
                                    naga::ScalarKind::Uint => UniformType::Scalar(ScalarType::U32),
                                    naga::ScalarKind::Float => UniformType::Scalar(ScalarType::F32),
                                    _ => Err(anyhow!("Invalid uniform type"))?,
                                },
                                naga::TypeInner::Vector { size, scalar } => match scalar.kind {
                                    naga::ScalarKind::Sint => UniformType::Vector {
                                        scalar_type: ScalarType::I32,
                                        size: *size,
                                    },
                                    naga::ScalarKind::Uint => UniformType::Vector {
                                        scalar_type: ScalarType::U32,
                                        size: *size,
                                    },
                                    naga::ScalarKind::Float => UniformType::Vector {
                                        scalar_type: ScalarType::F32,
                                        size: *size,
                                    },
                                    _ => Err(anyhow!("Invalid uniform type"))?,
                                },
                                naga::TypeInner::Matrix { columns, rows, .. } => {
                                    UniformType::MatrixF32 {
                                        columns: *columns,
                                        rows: *rows,
                                    }
                                }
                                naga::TypeInner::Array { base, size, .. } => {
                                    let length = match size {
                                        naga::ArraySize::Constant(size) => size.get(),
                                        naga::ArraySize::Dynamic => {
                                            Err(anyhow!("Invalid uniform type"))?
                                        }
                                    };

                                    let scalar_type = match &module.types[*base].inner {
                                        naga::TypeInner::Vector { size, scalar } => {
                                            match size {
                                                naga::VectorSize::Quad => {}
                                                _ => Err(anyhow!("Invalid uniform type"))?,
                                            };

                                            match scalar.kind {
                                                naga::ScalarKind::Sint => ScalarType::I32,
                                                naga::ScalarKind::Uint => ScalarType::U32,
                                                naga::ScalarKind::Float => ScalarType::F32,
                                                _ => Err(anyhow!("Invalid uniform type"))?,
                                            }
                                        }
                                        _ => Err(anyhow!("Invalid uniform type"))?,
                                    };

                                    UniformType::ArrayOfVector4 {
                                        scalar_type,
                                        length,
                                    }
                                }
                                _ => Err(anyhow!("Invalid uniform type"))?,
                            };

                            uniforms_meta.insert(name.clone(), UniformMeta { offset, wgsl_type });
                        }
                    }
                }
            }
        }

        Ok((uniforms_meta, total_struct_size))
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

    pub(crate) fn set_uniform(
        &mut self,
        key: &str,
        value: v8::Local<v8::Value>,
        scope: &mut v8::HandleScope,
    ) -> Result<()> {
        let uniform_meta = self
            .uniforms_meta
            .get(key)
            .ok_or_else(|| anyhow!("Shader uniform {} not found", key))?;

        let start = uniform_meta.offset;

        match &uniform_meta.wgsl_type {
            UniformType::Scalar(scalar_type) => {
                if !value.is_number() {
                    return Err(anyhow!(
                        "Invalid value provided to set scalar uniform {}, expected number but got {}",
                        key,
                        value.type_repr()
                    ));
                }

                let value = value
                    .to_number(scope)
                    .expect("Unexpected: could not parse v8 value into number");

                Self::write_uniform_scalar(
                    &mut self.uniforms_buffer,
                    value,
                    scope,
                    scalar_type,
                    start,
                );
            }
            UniformType::Vector { scalar_type, size } => {
                if !value.is_array() {
                    return Err(anyhow!(
                        "Invalid value provided to set vector uniform {}, expected array but got {}",
                        key,
                        value.type_repr()
                    ));
                }

                let vector = v8::Local::<v8::Array>::try_from(value)
                    .expect("Unexpected: could not parse v8 value into array");

                let size = *size as u32;

                if size != vector.length() {
                    return Err(anyhow!(
                        "Invalid array provided to set vector uniform {}, expected length of {} but got {}",
                        key,
                        size,
                        vector.length(),
                    ));
                }

                for i in 0..vector.length() {
                    let value = vector
                        .get_index(scope, i)
                        .expect("Unexpected: could not get v8 item value from array")
                        .to_number(scope)
                        .expect("Unexpected: could not parse v8 value into number");
                    let offset = start + (i as usize * 4);

                    Self::write_uniform_scalar(
                        &mut self.uniforms_buffer,
                        value,
                        scope,
                        scalar_type,
                        offset,
                    );
                }
            }
            UniformType::MatrixF32 { columns, rows } => {
                if !value.is_array() {
                    return Err(anyhow!(
                        "Invalid value provided to set matrix uniform {}, expected array but got {}",
                        key,
                        value.type_repr()
                    ));
                }

                let matrix = v8::Local::<v8::Array>::try_from(value)
                    .expect("Unexpected: could not parse v8 value into array");

                let columns = *columns as u32;
                let rows = *rows as u32;

                if columns * rows != matrix.length() {
                    return Err(anyhow!(
                        "Invalid array provided to set matrix uniform {}, expected length of {} but got {}",
                        key,
                        columns * rows,
                        matrix.length(),
                    ));
                }

                for i in 0..matrix.length() {
                    let value = matrix
                        .get_index(scope, i)
                        .expect("Unexpected: could not get v8 item value from array")
                        .to_number(scope)
                        .expect("Unexpected: could not parse v8 value into number");
                    let offset = start + (i as usize * 4);

                    let value = value.value() as f32;
                    self.uniforms_buffer[offset..offset + 4].copy_from_slice(&value.to_ne_bytes());
                }
            }
            UniformType::ArrayOfVector4 {
                scalar_type,
                length,
            } => {
                if !value.is_array() {
                    return Err(anyhow!(
                        "Invalid value provided to set array uniform {}, expected array but got {}",
                        key,
                        value.type_repr()
                    ));
                }

                let array = v8::Local::<v8::Array>::try_from(value)
                    .expect("Unexpected: could not parse v8 value into array");

                if *length != array.length() {
                    return Err(anyhow!(
                        "Invalid array provided to set array uniform {}, expected length of {} but got {}",
                        key,
                        length,
                        array.length(),
                    ));
                }

                for i in 0..array.length() {
                    let value = array
                        .get_index(scope, i)
                        .expect("Unexpected: could not get v8 item value from array");
                    let element_offset = start + (i as usize * 16);

                    if !value.is_array() {
                        return Err(anyhow!(
                            "Invalid item provided to set vector uniform {}, expected array but got {}",
                            key,
                            value.type_repr()
                        ));
                    }

                    let vector = v8::Local::<v8::Array>::try_from(value)
                        .expect("Unexpected: could not parse v8 value into array");

                    if 4 != vector.length() {
                        return Err(anyhow!(
                            "Invalid array item provided to set element of vector uniform {}, expected length of 4 but got {}",
                            key,
                            vector.length(),
                        ));
                    }

                    for j in 0..vector.length() {
                        let value = vector
                            .get_index(scope, j)
                            .expect("Unexpected: could not get v8 item value from array")
                            .to_number(scope)
                            .expect("Unexpected: could not parse v8 value into number");
                        let offset = element_offset + (j as usize * 4);

                        Self::write_uniform_scalar(
                            &mut self.uniforms_buffer,
                            value,
                            scope,
                            scalar_type,
                            offset,
                        );
                    }
                }
            }
        };

        self.has_uniforms_changed = true;

        Ok(())
    }

    fn write_uniform_scalar(
        buffer: &mut [u8],
        value: v8::Local<v8::Number>,
        scope: &mut v8::HandleScope,
        scalar_type: &ScalarType,
        offset: usize,
    ) {
        match scalar_type {
            ScalarType::F32 => {
                let value = value.value() as f32;
                buffer[offset..offset + 4].copy_from_slice(&value.to_ne_bytes());
            }
            ScalarType::I32 => {
                let value = value
                    .int32_value(scope)
                    .expect("Unexpected: could not parse v8 number into i32");
                buffer[offset..offset + 4].copy_from_slice(&value.to_ne_bytes());
            }
            ScalarType::U32 => {
                let value = value.value().max(0.0) as u32;
                buffer[offset..offset + 4].copy_from_slice(&value.to_ne_bytes());
            }
        };
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

pub(crate) type FullscreenShaderCommandMap = HashMap<ShapeId, FullscreenShaderId>;

impl SketchState {
    pub(crate) fn start_drawing_fullscreen_shader(&mut self, shader_id: FullscreenShaderId) {
        self.fullscreen_shader_command_map
            .insert(self.next_shape_id, shader_id);

        self.shape_order
            .push((self.next_shape_id, ShapeType::FullscreenShader));

        self.fullscreen_shader_map
            .get_mut(&shader_id)
            .expect("Unexpected: could not find fullscreen shader for given id")
            .is_being_drawn = true;
    }

    pub(crate) fn load_fullscreen_shader(&mut self, path: String) -> Result<FullscreenShaderId> {
        let shader_id = FullscreenShaderId::new(&path);

        let shader = FullscreenShader::new(path, self.width, self.height, &self.wgpu_handle)?;

        self.fullscreen_shader_map.insert(shader_id, shader);

        Ok(shader_id)
    }

    pub(crate) fn set_fullscreen_shader_uniform(
        &mut self,
        id: FullscreenShaderId,
        key: String,
        value: v8::Local<v8::Value>,
        scope: &mut v8::HandleScope,
    ) -> Result<()> {
        self.fullscreen_shader_map
            .get_mut(&id)
            .expect("Unexpected: could not find shader for given id")
            .set_uniform(&key, value, scope)?;

        Ok(())
    }
}

#[op2(fast)]
pub(crate) fn op_draw_fullscreen_shader(state: &mut OpState, shader_id: u32) {
    let state = state.sketch_store_mut().get_mut::<SketchState>();

    state.start_drawing_fullscreen_shader(FullscreenShaderId(shader_id));
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
    value: v8::Local<v8::Value>,
    scope: &mut v8::HandleScope,
) -> Result<(), JsErrorBox> {
    let state = state.sketch_store_mut().get_mut::<SketchState>();

    state
        .set_fullscreen_shader_uniform(FullscreenShaderId(id), key, value, scope)
        .map_err(|error| JsErrorBox::generic(error.to_string()))
}
