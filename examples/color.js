const draw = createDraw();

export function update() {
  draw.clear();

  const red = rgb(1, 0, 0);
  const green = rgb(0, 1, 0);
  const blue = rgb(0, 0, 1);

  draw.ellipse().xy(-100, 75).wh(50, 50).fill(red);
  draw.ellipse().xy(0, 75).wh(50, 50).fill(green);
  draw.ellipse().xy(100, 75).wh(50, 50).fill(blue);

  for (let i = 0; i <= 9; i++) {
    const x = (i / 9) * 400 - 200;

    const hue = i / 10;
    const color = hsv(hue, 1, 1);

    draw.rect().xy(x, -75).wh(25, 25).fill(color);
  }
}
