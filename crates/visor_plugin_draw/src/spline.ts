import ops from "./ops.ts";

const {
  op_draw_spline_xyz,
  op_draw_spline_point,
  op_draw_spline_stroke_rgba,
  op_draw_spline_stroke_hsva,
  op_draw_spline_stroke_weight,
  op_draw_spline_tension,
  op_draw_spline_resolution,
} = ops;

export class Spline {
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
    if (!Number.isInteger(n)) {
      throw new Error("Resolution must be an integer");
    }

    if (n <= 0) {
      throw new Error("Resolution must be greater than 0");
    }

    op_draw_spline_resolution(this.#id, n);
    return this;
  }
}
