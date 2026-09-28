const shader = loadFullscreenShader("./examples/fullscreen-shader/uv.wgsl");

const draw = createDraw();

export function update() {
  draw.clear();

  fullscreenShader(shader);
}
