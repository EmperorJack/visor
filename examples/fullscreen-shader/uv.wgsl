struct FragmentInput {
  @location(0) uv: vec2f,
}

struct FragmentOutput {
  @location(0) color: vec4f,
}

@fragment
fn fs_main(in: FragmentInput) -> FragmentOutput {
  var color = vec4f(in.uv.x, in.uv.y, 0.0, 1.0);

  return FragmentOutput(color);
}
