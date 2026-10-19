const texture = loadTexture("../assets/readme/visuals-screenshot.png");

const draw = createDraw();

console.log(`Texture size: [${texture.width()}, ${texture.height()}]`);

export function update() {
  draw.clear();

  draw.texture(texture).wh(300, 300).tint(1, 1, 0, 1);
}
