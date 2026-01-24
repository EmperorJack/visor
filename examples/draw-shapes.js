const draw = createDraw();

export function update() {
  draw.clear();

  draw.rect().xy(-225, 75).wh(75, 75);
  draw
    .rect()
    .xy(-225, -75)
    .wh(75, 75)
    .noFill()
    .stroke(rgb(1, 1, 1))
    .strokeWeight(1);

  draw.ellipse().xy(-75, 75).wh(75, 75);
  draw
    .ellipse()
    .xy(-75, -75)
    .wh(75, 75)
    .noFill()
    .stroke(rgb(1, 1, 1))
    .strokeWeight(1);

  draw.quad().xy(75, 75).points(-50, 0, 0, 50, 50, 0, 0, -50);
  draw
    .quad()
    .xy(75, -75)
    .points(-50, 0, 0, 50, 50, 0, 0, -50)
    .noFill()
    .stroke(rgb(1, 1, 1))
    .strokeWeight(1);

  draw
    .polygon()
    .xy(225, 75)
    .point(0, 35)
    .point(35, 0)
    .point(35, -35)
    .point(-35, -35)
    .point(-35, 0)
    .point(0, 35);

  draw
    .polygon()
    .xy(225, -75)
    .point(0, 35)
    .point(35, 0)
    .point(35, -35)
    .point(-35, -35)
    .point(-35, 0)
    .point(0, 35)
    .noFill()
    .stroke(rgb(1, 1, 1))
    .strokeWeight(1);
}
