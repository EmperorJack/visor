const draw = createDraw();

export function setup() {
  console.log("Hello, world!");
}

export function update() {
  draw.clear();

  for (let i = -5; i <= 5; i++) {
    const x = i * 50;
    const y = Math.sin(time() + i) * 50;

    const hue = map(x, -250, 250, 0, 1);
    const color = hsv(hue, 0.8, 1);

    draw.ellipse().xy(x, y).wh(25, 25).fill(color);
  }
}
