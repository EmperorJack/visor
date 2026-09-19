const shader = loadFullscreenShader("./examples/fullscreen-shader.wgsl");

const draw = createDraw();

export function update() {
  draw.clear();

  draw.fullscreenShader(shader);
}
