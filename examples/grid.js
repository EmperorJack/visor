let draw = createDraw();

export function update() {
  draw.clear();

  const n = 10;
  const xSpacing = width() / n;
  const ySpacing = height() / n;

  draw = draw.translate(-width() * 0.5, -height() * 0.5);
  draw = draw.translate(xSpacing * 0.5, ySpacing * 0.5);

  for (let xi = 0; xi < n; xi++) {
    for (let yi = 0; yi < n; yi++) {
      const x = xi * xSpacing;
      const y = yi * ySpacing;

      draw.rect().xy(x, y).wh(15, 15);
    }
  }
}
