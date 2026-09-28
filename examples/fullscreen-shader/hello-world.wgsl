struct FragmentInput {
  @location(0) uv: vec2f,
}

struct FragmentOutput {
  @location(0) color: vec4f,
}

struct Uniforms {
  time: f32,
  width: u32,
  height: u32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@fragment
fn fs_main(in: FragmentInput) -> FragmentOutput {
  let time = uniforms.time;

  let blue = sin(in.uv.x + time * 1.25) + cos(in.uv.y + time * 0.75) + time * 0.5;

  return FragmentOutput(vec4f(0.0, 0.0, blue % 1.0, 1.0));
}
