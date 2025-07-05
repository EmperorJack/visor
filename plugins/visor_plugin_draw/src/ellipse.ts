import ops from "./ops.ts";

const {
  op_draw_ellipse_xy,
  op_draw_ellipse_xyz,
  op_draw_ellipse_wh,
  op_draw_ellipse_fill_rgba,
  op_draw_ellipse_fill_hsva,
  op_draw_ellipse_no_fill,
  op_draw_ellipse_stroke_rgba,
  op_draw_ellipse_stroke_hsva,
  op_draw_ellipse_stroke_weight,
} = ops;

export class Ellipse {
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
