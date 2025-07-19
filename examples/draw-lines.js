const draw = createDraw();

export function update() {
  draw.clear();

  const n = 5;

  const polyline = draw
    .polyline()
    .xy(-250, 100)
    .stroke(rgb(1, 1, 1))
    .strokeWeight(1);

  const spline = draw
    .spline()
    .xy(-250, 0)
    .stroke(rgb(1, 1, 1))
    .strokeWeight(1)
    .tension(0.25)
    .resolution(100);

  const spline2 = draw
    .spline()
    .xy(-250, -100)
    .stroke(rgb(1, 1, 1))
    .strokeWeight(1)
    .tension(0.5)
    .resolution(100);

  for (let i = 0; i <= n; i++) {
    const x = (i / n) * 500;
    const y = Math.sin((i / n) * Math.PI * 2) * 75;
    polyline.point(x, y);
    spline.point(x, y);
    spline2.point(x, y);
  }
}
