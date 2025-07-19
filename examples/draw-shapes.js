const draw = createDraw();

export function update() {
  draw.clear();

  draw.rect().xy(-150, 75).wh(75, 75);
  draw.ellipse().xy(0, 75).wh(75, 75);
  draw.quad().xy(150, 75).points(-50, 0, 0, 50, 50, 0, 0, -50);

  draw
    .rect()
    .xy(-150, -75)
    .wh(75, 75)
    .noFill()
    .stroke(rgb(1, 1, 1))
    .strokeWeight(1);
  draw
    .ellipse()
    .xy(0, -75)
    .wh(75, 75)
    .noFill()
    .stroke(rgb(1, 1, 1))
    .strokeWeight(1);
  draw
    .quad()
    .xy(150, -75)
    .points(-50, 0, 0, 50, 50, 0, 0, -50)
    .noFill()
    .stroke(rgb(1, 1, 1))
    .strokeWeight(1);
}
