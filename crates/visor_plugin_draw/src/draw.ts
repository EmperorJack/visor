import { Ellipse } from "./ellipse.ts";
import ops from "./ops.ts";
import { Polygon } from "./polygon.ts";
import { Polyline } from "./polyline.ts";
import { Quad } from "./quad.ts";
import { Rect } from "./rect.ts";
import { Spline } from "./spline.ts";

const {
  op_draw_background_rgb,
  op_draw_background_hsv,
  op_draw_ellipse,
  op_draw_rect,
  op_draw_quad,
  op_draw_polygon,
  op_draw_polyline,
  op_draw_spline,
  op_draw_translate,
  op_draw_rotate,
  op_draw_scale,
} = ops;

export class Draw {
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
  polygon() {
    const shapeId = op_draw_polygon(this.#id);
    return new Polygon(shapeId);
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
