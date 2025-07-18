const draw = createDraw();

export function update() {
  draw.clear();

  draw.rect().xy(0, 0).wh(100, 100).fill(rgb(1, 0, 0));
}
