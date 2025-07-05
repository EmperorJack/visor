import ops from "./ops.ts";

const {
  op_draw_polyline_xyz,
  op_draw_polyline_point,
  op_draw_polyline_stroke_rgba,
  op_draw_polyline_stroke_hsva,
  op_draw_polyline_stroke_weight,
} = ops;

export class Polyline {
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
