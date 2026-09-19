struct VertexOutput {
  @builtin(position) position: vec4f,
  @location(0) uv: vec2f,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
  var uvs = array<vec2f, 3>(
    vec2f(0.0, 2.0),
    vec2f(0.0, 0.0),
    vec2f(2.0, 0.0)
  );
  let uv = uvs[vertex_index];

  let position: vec4f = vec4f(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0, 0.0, 1.0);

  return VertexOutput(position, uv);
}
