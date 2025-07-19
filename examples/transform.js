const draw = createDraw();

export function update() {
  draw.clear();

  const drawTranslated = draw.translate(-150, 0);
  drawTranslated.rect().wh(75, 75);

  const drawScaled = draw.scale(1.5);
  drawScaled.rect().wh(75, 75);

  const drawRotated = draw.translate(150, 0).rotate(Math.PI * 0.25);
  drawRotated.rect().wh(75, 75);
}
