const shader = loadFullscreenShader(
  "./examples/fullscreen-shader/hello-world.wgsl",
);

const draw = createDraw();

export function update() {
  draw.clear();

  shader.setUniform("time", time());
  shader.setUniform("width", width());
  shader.setUniform("height", height());

  fullscreenShader(shader);
}
