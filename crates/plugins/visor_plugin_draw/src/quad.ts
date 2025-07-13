import ops from "./ops.ts";

const {
  op_draw_quad_xy,
  op_draw_quad_xyz,
  op_draw_quad_points,
  op_draw_quad_fill_rgba,
  op_draw_quad_fill_hsva,
  op_draw_quad_no_fill,
  op_draw_quad_stroke_rgba,
  op_draw_quad_stroke_hsva,
  op_draw_quad_stroke_weight,
} = ops;

export class Quad {
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
