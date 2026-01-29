const draw = createDraw();

export function update() {
  draw.clear();

  const n = 10;

  const path = draw
    .path()
    .xy(-250, 100)
    .fill(rgb(1, 1, 1));

  const pathCurved = draw
    .path()
    .xy(-250, 0)
    .fill(rgb(1, 1, 1))
    .tension(0.25);

  const pathCurved2 = draw
    .path()
    .xy(-250, -100)
    .fill(rgb(1, 1, 1))
    .tension(0.5);

  path.point(0, -25);
  pathCurved.point(0, -25);
  pathCurved2.point(0, -25);

  for (let i = 0; i <= n; i++) {
    const x = (i / n) * 500;
    const y = noise(i * 10, time()) * 25;
    path.point(x, y);
    pathCurved.point(x, y);
    pathCurved2.point(x, y);
  }

  path.point(500, -25);
  pathCurved.point(500, -25);
  pathCurved2.point(500, -25);
}
