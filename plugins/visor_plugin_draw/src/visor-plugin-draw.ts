type ShapeCommand = (id: number) => number;
type ShapeXYCommand = (id: number, x: number, y: number) => void;
type ShapeXYZCommand = (id: number, x: number, y: number, z: number) => void;
type ShapeWHCommand = (id: number, w: number, h: number) => void;
type ShapeRGBACommand = (
  id: number,
  r: number,
  g: number,
  b: number,
  a: number
) => void;
type ShapeHSVACommand = (
  id: number,
  h: number,
  s: number,
  v: number,
  a: number
) => void;
type ShapeNoFillCommand = (id: number) => void;
type ShapeStrokeWeightCommand = (id: number, w: number) => void;
type ShapeTensionCommand = (id: number, t: number) => void;
type ShapePointCommand = (id: number, x: number, y: number) => void;

declare namespace Deno {
  const core: {
    ops: {
      op_draw_background_rgb: (
        id: number,
        r: number,
        g: number,
        b: number
      ) => void;
      op_draw_background_hsv: (
        id: number,
        h: number,
        s: number,
        v: number
      ) => void;
      op_draw_ellipse: ShapeCommand;
      op_draw_ellipse_xy: ShapeXYCommand;
      op_draw_ellipse_xyz: ShapeXYZCommand;
      op_draw_ellipse_wh: ShapeWHCommand;
      op_draw_ellipse_fill_rgba: ShapeRGBACommand;
      op_draw_ellipse_fill_hsva: ShapeHSVACommand;
      op_draw_ellipse_no_fill: ShapeNoFillCommand;
      op_draw_ellipse_stroke_rgba: ShapeRGBACommand;
      op_draw_ellipse_stroke_hsva: ShapeHSVACommand;
      op_draw_ellipse_stroke_weight: ShapeStrokeWeightCommand;
      op_draw_rect: ShapeCommand;
      op_draw_rect_xy: ShapeXYCommand;
      op_draw_rect_xyz: ShapeXYZCommand;
      op_draw_rect_wh: ShapeWHCommand;
      op_draw_rect_fill_rgba: ShapeRGBACommand;
      op_draw_rect_fill_hsva: ShapeHSVACommand;
      op_draw_rect_no_fill: ShapeNoFillCommand;
      op_draw_rect_stroke_rgba: ShapeRGBACommand;
      op_draw_rect_stroke_hsva: ShapeHSVACommand;
      op_draw_rect_stroke_weight: ShapeStrokeWeightCommand;
      op_draw_quad: ShapeCommand;
      op_draw_quad_xy: ShapeXYCommand;
      op_draw_quad_xyz: ShapeXYZCommand;
      op_draw_quad_points: (
        id: number,
        x1: number,
        y1: number,
        x2: number,
        y2: number,
        x3: number,
        y3: number,
        x4: number,
        y4: number
      ) => void;
      op_draw_quad_fill_rgba: ShapeRGBACommand;
      op_draw_quad_fill_hsva: ShapeHSVACommand;
      op_draw_quad_no_fill: ShapeNoFillCommand;
      op_draw_quad_stroke_rgba: ShapeRGBACommand;
      op_draw_quad_stroke_hsva: ShapeHSVACommand;
      op_draw_quad_stroke_weight: ShapeStrokeWeightCommand;
      op_draw_polyline: ShapeCommand;
      op_draw_polyline_xyz: ShapeXYZCommand;
      op_draw_polyline_point: ShapePointCommand;
      op_draw_polyline_stroke_rgba: ShapeRGBACommand;
      op_draw_polyline_stroke_hsva: ShapeHSVACommand;
      op_draw_polyline_stroke_weight: ShapeStrokeWeightCommand;
      op_draw_spline: ShapeCommand;
      op_draw_spline_xyz: ShapeXYZCommand;
      op_draw_spline_point: ShapePointCommand;
      op_draw_spline_stroke_rgba: ShapeRGBACommand;
      op_draw_spline_stroke_hsva: ShapeHSVACommand;
      op_draw_spline_stroke_weight: ShapeStrokeWeightCommand;
      op_draw_spline_tension: ShapeTensionCommand;
      op_draw_spline_resolution: (id: number, n: number) => void;
      op_draw_translate: (id: number, x: number, y: number) => number;
      op_draw_rotate: (id: number, radians: number) => number;
      op_draw_scale: (id: number, s: number) => number;
      op_draw_noise: (x: number, y: number, z: number) => number;
    };
  };
}

const {
  op_draw_background_rgb,
  op_draw_background_hsv,
  op_draw_ellipse,
  op_draw_ellipse_xy,
  op_draw_ellipse_xyz,
  op_draw_ellipse_wh,
  op_draw_ellipse_fill_rgba,
  op_draw_ellipse_fill_hsva,
  op_draw_ellipse_no_fill,
  op_draw_ellipse_stroke_rgba,
  op_draw_ellipse_stroke_hsva,
  op_draw_ellipse_stroke_weight,
  op_draw_rect,
  op_draw_rect_xy,
  op_draw_rect_xyz,
  op_draw_rect_wh,
  op_draw_rect_fill_rgba,
  op_draw_rect_fill_hsva,
  op_draw_rect_no_fill,
  op_draw_rect_stroke_rgba,
  op_draw_rect_stroke_hsva,
  op_draw_rect_stroke_weight,
  op_draw_quad,
  op_draw_quad_xy,
  op_draw_quad_xyz,
  op_draw_quad_points,
  op_draw_quad_fill_rgba,
  op_draw_quad_fill_hsva,
  op_draw_quad_no_fill,
  op_draw_quad_stroke_rgba,
  op_draw_quad_stroke_hsva,
  op_draw_quad_stroke_weight,
  op_draw_polyline,
  op_draw_polyline_xyz,
  op_draw_polyline_point,
  op_draw_polyline_stroke_rgba,
  op_draw_polyline_stroke_hsva,
  op_draw_polyline_stroke_weight,
  op_draw_spline,
  op_draw_spline_xyz,
  op_draw_spline_point,
  op_draw_spline_stroke_rgba,
  op_draw_spline_stroke_hsva,
  op_draw_spline_stroke_weight,
  op_draw_spline_tension,
  op_draw_spline_resolution,
  op_draw_translate,
  op_draw_rotate,
  op_draw_scale,
  op_draw_noise,
} = Deno.core.ops;

class Draw {
  #id: number;

  constructor(id: number) {
    this.#id = id;
  }

  // Background
  clear() {
    op_draw_background_rgb(this.#id, 0, 0, 0);
  }
  background(color: Color) {
    switch (color.type) {
      case "rgba": {
        const { r, g, b } = color;
        op_draw_background_rgb(this.#id, r, g, b);
        break;
      }

      case "hsva": {
        const { h, s, v } = color;
        op_draw_background_hsv(this.#id, h, s, v);
        break;
      }
    }
  }

  // Shapes
  ellipse() {
    const shapeId = op_draw_ellipse(this.#id);
    return new Ellipse(shapeId);
  }
  rect() {
    const shapeId = op_draw_rect(this.#id);
    return new Rect(shapeId);
  }
  quad() {
    const shapeId = op_draw_quad(this.#id);
    return new Quad(shapeId);
  }
  polyline() {
    const shapeId = op_draw_polyline(this.#id);
    return new Polyline(shapeId);
  }
  spline() {
    const shapeId = op_draw_spline(this.#id);
    return new Spline(shapeId);
  }

  // Transforms
  translate(x: number, y: number) {
    const nextId = op_draw_translate(this.#id, x, y);
    return new Draw(nextId);
  }
  rotate(radians: number) {
    const nextId = op_draw_rotate(this.#id, radians);
    return new Draw(nextId);
  }
  scale(s: number) {
    const nextId = op_draw_scale(this.#id, s);
    return new Draw(nextId);
  }

  copy() {
    return new Draw(this.#id);
  }
}

class Ellipse {
  #id: number;

  constructor(id: number) {
    this.#id = id;
  }

  xy(x: number, y: number) {
    op_draw_ellipse_xy(this.#id, x, y);
    return this;
  }

  xyz(x: number, y: number, z: number) {
    op_draw_ellipse_xyz(this.#id, x, y, z);
    return this;
  }

  wh(w: number, h: number) {
    op_draw_ellipse_wh(this.#id, w, h);
    return this;
  }

  fill(color: Color) {
    switch (color.type) {
      case "rgba": {
        const { r, g, b, a } = color;
        op_draw_ellipse_fill_rgba(this.#id, r, g, b, a);
        break;
      }

      case "hsva": {
        const { h, s, v, a } = color;
        op_draw_ellipse_fill_hsva(this.#id, h, s, v, a);
        break;
      }
    }

    return this;
  }

  noFill() {
    op_draw_ellipse_no_fill(this.#id);
    return this;
  }

  stroke(color: Color) {
    switch (color.type) {
      case "rgba": {
        const { r, g, b, a } = color;
        op_draw_ellipse_stroke_rgba(this.#id, r, g, b, a);
        break;
      }

      case "hsva": {
        const { h, s, v, a } = color;
        op_draw_ellipse_stroke_hsva(this.#id, h, s, v, a);
        break;
      }
    }

    return this;
  }

  strokeWeight(w: number) {
    op_draw_ellipse_stroke_weight(this.#id, w);
    return this;
  }
}

class Rect {
  #id: number;

  constructor(id: number) {
    this.#id = id;
  }

  xy(x: number, y: number) {
    op_draw_rect_xy(this.#id, x, y);
    return this;
  }

  xyz(x: number, y: number, z: number) {
    op_draw_rect_xyz(this.#id, x, y, z);
    return this;
  }

  wh(w: number, h: number) {
    op_draw_rect_wh(this.#id, w, h);
    return this;
  }

  fill(color: Color) {
    switch (color.type) {
      case "rgba": {
        const { r, g, b, a } = color;
        op_draw_rect_fill_rgba(this.#id, r, g, b, a);
        break;
      }

      case "hsva": {
        const { h, s, v, a } = color;
        op_draw_rect_fill_hsva(this.#id, h, s, v, a);
        break;
      }
    }

    return this;
  }

  noFill() {
    op_draw_rect_no_fill(this.#id);
    return this;
  }

  stroke(color: Color) {
    switch (color.type) {
      case "rgba": {
        const { r, g, b, a } = color;
        op_draw_rect_stroke_rgba(this.#id, r, g, b, a);
        break;
      }

      case "hsva": {
        const { h, s, v, a } = color;
        op_draw_rect_stroke_hsva(this.#id, h, s, v, a);
        break;
      }
    }

    return this;
  }

  strokeWeight(w: number) {
    op_draw_rect_stroke_weight(this.#id, w);
    return this;
  }
}

class Quad {
  #id: number;

  constructor(id: number) {
    this.#id = id;
  }

  xy(x: number, y: number) {
    op_draw_quad_xy(this.#id, x, y);
    return this;
  }

  xyz(x: number, y: number, z: number) {
    op_draw_quad_xyz(this.#id, x, y, z);
    return this;
  }

  points(
    x1: number,
    y1: number,
    x2: number,
    y2: number,
    x3: number,
    y3: number,
    x4: number,
    y4: number
  ) {
    op_draw_quad_points(this.#id, x1, y1, x2, y2, x3, y3, x4, y4);
    return this;
  }

  fill(color: Color) {
    switch (color.type) {
      case "rgba": {
        const { r, g, b, a } = color;
        op_draw_quad_fill_rgba(this.#id, r, g, b, a);
        break;
      }

      case "hsva": {
        const { h, s, v, a } = color;
        op_draw_quad_fill_hsva(this.#id, h, s, v, a);
        break;
      }
    }

    return this;
  }

  noFill() {
    op_draw_quad_no_fill(this.#id);
    return this;
  }

  stroke(color: Color) {
    switch (color.type) {
      case "rgba": {
        const { r, g, b, a } = color;
        op_draw_quad_stroke_rgba(this.#id, r, g, b, a);
        break;
      }

      case "hsva": {
        const { h, s, v, a } = color;
        op_draw_quad_stroke_hsva(this.#id, h, s, v, a);
        break;
      }
    }

    return this;
  }

  strokeWeight(w: number) {
    op_draw_quad_stroke_weight(this.#id, w);
    return this;
  }
}

class Polyline {
  #id: number;

  constructor(id: number) {
    this.#id = id;
  }

  xy(x: number, y: number) {
    op_draw_polyline_xyz(this.#id, x, y, 0);
    return this;
  }

  xyz(x: number, y: number, z: number) {
    op_draw_polyline_xyz(this.#id, x, y, z);
    return this;
  }

  point(x: number, y: number) {
    op_draw_polyline_point(this.#id, x, y);
    return this;
  }

  stroke(color: Color) {
    switch (color.type) {
      case "rgba": {
        const { r, g, b, a } = color;
        op_draw_polyline_stroke_rgba(this.#id, r, g, b, a);
        break;
      }

      case "hsva": {
        const { h, s, v, a } = color;
        op_draw_polyline_stroke_hsva(this.#id, h, s, v, a);
        break;
      }
    }

    return this;
  }

  strokeWeight(w: number) {
    op_draw_polyline_stroke_weight(this.#id, w);
    return this;
  }
}

class Spline {
  #id: number;

  constructor(id: number) {
    this.#id = id;
  }

  xy(x: number, y: number) {
    op_draw_spline_xyz(this.#id, x, y, 0);
    return this;
  }

  xyz(x: number, y: number, z: number) {
    op_draw_spline_xyz(this.#id, x, y, z);
    return this;
  }

  point(x: number, y: number) {
    op_draw_spline_point(this.#id, x, y);
    return this;
  }

  stroke(color: Color) {
    switch (color.type) {
      case "rgba": {
        const { r, g, b, a } = color;
        op_draw_spline_stroke_rgba(this.#id, r, g, b, a);
        break;
      }

      case "hsva": {
        const { h, s, v, a } = color;
        op_draw_spline_stroke_hsva(this.#id, h, s, v, a);
        break;
      }
    }

    return this;
  }

  strokeWeight(w: number) {
    op_draw_spline_stroke_weight(this.#id, w);
    return this;
  }

  tension(t: number) {
    op_draw_spline_tension(this.#id, t);
    return this;
  }

  resolution(n: number) {
    op_draw_spline_resolution(this.#id, n);
    return this;
  }
}

function rgb(r: number, g: number, b: number): Color {
  return { type: "rgba", r, g, b, a: 1.0 };
}

function rgba(r: number, g: number, b: number, a: number): Color {
  return { type: "rgba", r, g, b, a };
}

function hsv(h: number, s: number, v: number): Color {
  return { type: "hsva", h, s, v, a: 1.0 };
}

function hsva(h: number, s: number, v: number, a: number): Color {
  return { type: "hsva", h, s, v, a };
}

function createDraw() {
  return new Draw(0);
}

function random(amount: number) {
  return Math.random() * amount;
}

function radians(degrees: number) {
  return degrees * (Math.PI / 180);
}

function degrees(radians: number) {
  return radians * (180 / Math.PI);
}

function lerp(start: number, stop: number, amount: number) {
  return start + (stop - start) * amount;
}

function norm(value: number, start: number, stop: number) {
  return (value - start) / (stop - start);
}

function map(
  value: number,
  start1: number,
  stop1: number,
  start2: number,
  stop2: number
) {
  return start2 + (stop2 - start2) * ((value - start1) / (stop1 - start1));
}

function constrain(value: number, min: number, max: number) {
  if (value < min) {
    return min;
  }

  if (value > max) {
    return max;
  }

  return value;
}

function noise(x: number, y = 0, z = 0) {
  return op_draw_noise(x, y, z);
}

globalThis.createDraw = createDraw;
globalThis.rgb = rgb;
globalThis.rgba = rgba;
globalThis.hsv = hsv;
globalThis.hsva = hsva;
// TODO: move the following to a math plugin?
globalThis.random = random;
globalThis.radians = radians;
globalThis.degrees = degrees;
globalThis.lerp = lerp;
globalThis.norm = norm;
globalThis.map = map;
globalThis.constrain = constrain;
globalThis.noise = noise;
