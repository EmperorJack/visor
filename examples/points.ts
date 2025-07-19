const draw = createDraw();

type Point = {
  x: number;
  y: number;
};

const points: Array<Point> = [];

export function setup() {
  for (let i = 0; i < 100; i++) {
    points.push({ x: Math.random() * 500 - 250, y: Math.random() * 500 - 250 });
  }
}

export function update() {
  draw.clear();

  for (let i = 0; i < points.length; i++) {
    const opacity = i / points.length;

    const point = points[i];

    draw
      .ellipse()
      .xy(point.x, point.y)
      .wh(10, 10)
      .fill(rgba(0.5, 0, 1, opacity));
  }
}
