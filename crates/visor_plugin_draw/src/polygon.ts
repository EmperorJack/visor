import ops from "./ops.ts";

const {
  op_draw_polygon_xy,
  op_draw_polygon_xyz,
  op_draw_polygon_point,
  op_draw_polygon_fill_rgba,
  op_draw_polygon_fill_hsva,
  op_draw_polygon_no_fill,
  op_draw_polygon_stroke_rgba,
  op_draw_polygon_stroke_hsva,
  op_draw_polygon_stroke_weight,
} = ops;

export class Polygon {
  #id: number;

  constructor(id: number) {
    this.#id = id;
  }

  xy(x: number, y: number) {
    op_draw_polygon_xy(this.#id, x, y);
    return this;
  }

  xyz(x: number, y: number, z: number) {
    op_draw_polygon_xyz(this.#id, x, y, z);
    return this;
  }

  point(x: number, y: number) {
    op_draw_polygon_point(this.#id, x, y);
    return this;
  }

  fill(color: Color) {
    switch (color.type) {
      case "rgba": {
        const { r, g, b, a } = color;
        op_draw_polygon_fill_rgba(this.#id, r, g, b, a);
        break;
      }

      case "hsva": {
        const { h, s, v, a } = color;
        op_draw_polygon_fill_hsva(this.#id, h, s, v, a);
        break;
      }
    }

    return this;
  }

  noFill() {
    op_draw_polygon_no_fill(this.#id);
    return this;
  }

  stroke(color: Color) {
    switch (color.type) {
      case "rgba": {
        const { r, g, b, a } = color;
        op_draw_polygon_stroke_rgba(this.#id, r, g, b, a);
        break;
      }

      case "hsva": {
        const { h, s, v, a } = color;
        op_draw_polygon_stroke_hsva(this.#id, h, s, v, a);
        break;
      }
    }

    return this;
  }

  strokeWeight(w: number) {
    op_draw_polygon_stroke_weight(this.#id, w);
    return this;
  }
}
