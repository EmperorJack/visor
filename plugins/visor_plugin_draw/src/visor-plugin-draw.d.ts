declare function createDraw(): Draw;

interface Draw {
  clear(): void;
  background(color: Color): void;

  ellipse(): Ellipse;
  rect(): Rect;
  quad(): Quad;
  polyline(): Polyline;
  spline(): Spline;

  translate(x: number, y: number): Draw;
  rotate(radians: number): Draw;
  scale(s: number): Draw;

  copy(): Draw;
}

interface Ellipse
  extends ShapePosition<Ellipse>,
    ShapeWidthHeight<Ellipse>,
    ShapeFill<Ellipse>,
    ShapeStroke<Ellipse> {}

interface Rect
  extends ShapePosition<Rect>,
    ShapeWidthHeight<Rect>,
    ShapeFill<Rect>,
    ShapeStroke<Rect> {}

interface Quad extends ShapePosition<Quad>, ShapeFill<Quad>, ShapeStroke<Quad> {
  points(
    x1: number,
    y1: number,
    x2: number,
    y2: number,
    x3: number,
    y3: number,
    x4: number,
    y4: number
  ): Quad;
}

interface Polyline
  extends ShapePosition<Polyline>,
    ShapePoint<Polyline>,
    ShapeStroke<Polyline> {}

interface Spline
  extends ShapePosition<Spline>,
    ShapePoint<Spline>,
    ShapeStroke<Spline> {
  tension(t: number): Spline;
  resolution(s: number): Spline;
}

interface ShapePosition<S> {
  xy(x: number, y: number): S;
  xyz(x: number, y: number, z: number): S;
}

interface ShapeWidthHeight<S> {
  wh(w: number, h: number): S;
}

interface ShapePoint<S> {
  point(x: number, y: number): S;
}

interface ShapeFill<S> {
  fill(color: Color): S;
  noFill(): S;
}

interface ShapeStroke<S> {
  stroke(color: Color): S;
  strokeWeight(w: number): S;
}

type Color =
  | {
      type: "rgba";
      r: number;
      g: number;
      b: number;
      a: number;
    }
  | {
      type: "hsva";
      h: number;
      s: number;
      v: number;
      a: number;
    };

declare function rgb(r: number, g: number, b: number): Color;
declare function rgba(r: number, g: number, b: number, a: number): Color;
declare function hsv(h: number, s: number, v: number): Color;
declare function hsva(h: number, s: number, v: number, a: number): Color;

declare function random(amount: number): number;
declare function radians(degrees: number): number;
declare function degrees(radians: number): number;
declare function lerp(start: number, stop: number, amount: number): number;
declare function norm(value: number, start: number, stop: number): number;
declare function map(
  value: number,
  start1: number,
  stop1: number,
  start2: number,
  stop2: number
): number;
declare function constrain(value: number, min: number, max: number): number;
declare function noise(x: number, y?: number, z?: number): number;
