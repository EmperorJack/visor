const draw = createDraw();

export function update() {
  draw.background(rgb(0, 0, 1));

  draw
    .rect()
    .xy(0, 0)
    .wh(100, 100)
    .fill(rgb(1, 0, 0));
}
