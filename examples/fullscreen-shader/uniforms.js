const shader = loadFullscreenShader(
  "./examples/fullscreen-shader/uniforms.wgsl",
);

const draw = createDraw();

shader.setUniform("scalar_f32", 1.5);
shader.setUniform("scalar_i32", -1);
shader.setUniform("scalar_u32", 1);

shader.setUniform("vector_2_f32", [0, 0]);
shader.setUniform("vector_2_i32", [0, 0]);
shader.setUniform("vector_2_u32", [0, 0]);
shader.setUniform("vector_3_f32", [0, 0, 0]);
shader.setUniform("vector_3_i32", [0, 0, 0]);
shader.setUniform("vector_3_u32", [0, 0, 0]);
shader.setUniform("vector_4_f32", [0, 0, 0, 0]);
shader.setUniform("vector_4_i32", [0, 0, 0, 0]);
shader.setUniform("vector_4_u32", [0, 0, 0, 0]);

shader.setUniform(
  "matrix_2x2",
  [
    //
    0, 0,
    //
    0, 0,
  ],
);
shader.setUniform(
  "matrix_3x3",
  [
    0, 0, 0,
    //
    0, 0, 0,
    //
    0, 0, 0,
  ],
);
shader.setUniform(
  "matrix_4x4",
  [
    0, 0, 0, 0,
    //
    0, 0, 0, 0,
    //
    0, 0, 0, 0,
    //
    0, 0, 0, 0,
  ],
);

shader.setUniform("array_f32", [
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
]);
shader.setUniform("array_i32", [
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
]);
shader.setUniform("array_u32", [
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
  [0, 0, 0, 0],
]);

export function update() {
  draw.clear();

  shader.setUniform("scalar_f32", time() % 1.0);

  fullscreenShader(shader);
}
