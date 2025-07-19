const draw = createDraw();

export function update() {
  draw.clear();

  if (frameCount() % 60 == 0) {
    console.log(frameCount());
  }

  const size = 75 + Math.sin(time()) * 25;

  draw
    .rotate(time() * 0.5)
    .rect()
    .wh(size, size)
    .fill(rgb(0, 1, 1));
}
