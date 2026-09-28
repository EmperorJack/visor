struct FragmentInput {
  @location(0) uv: vec2f,
}

struct FragmentOutput {
  @location(0) color: vec4f,
}

struct Uniforms {
  // Scalars
  scalar_f32: f32,
  scalar_i32: i32,
  scalar_u32: f32,

  // Vectors
  vector_2_f32: vec2<f32>,
  vector_2_i32: vec2<i32>,
  vector_2_u32: vec2<u32>,
  vector_3_f32: vec3<f32>,
  vector_3_i32: vec3<i32>,
  vector_3_u32: vec3<u32>,
  vector_4_f32: vec4<f32>,
  vector_4_i32: vec4<i32>,
  vector_4_u32: vec4<u32>,

  // Matrices (f32 only)
  matrix_2x2: mat2x2<f32>,
  matrix_3x3: mat3x3<f32>,
  matrix_4x4: mat4x4<f32>,
  // Note that non-square matrices are also supported e.g: mat2x3

  // Arrays (vector4 only)
  array_f32: array<vec4<f32>, 10>,
  array_i32: array<vec4<i32>, 10>,
  array_u32: array<vec4<i32>, 10>,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@fragment
fn fs_main(in: FragmentInput) -> FragmentOutput {
  let color = vec4f(uniforms.scalar_f32, 0, 0, 1);

  return FragmentOutput(color);
}
