import ops from "./ops.ts";

const {
  op_draw_path_xy,
  op_draw_path_xyz,
  op_draw_path_point,
  op_draw_path_fill_rgba,
  op_draw_path_fill_hsva,
  op_draw_path_tension,
  op_draw_path_resolution,
} = ops;

export class Path {
  #id: number;

  constructor(id: number) {
    this.#id = id;
  }

  xy(x: number, y: number) {
    op_draw_path_xy(this.#id, x, y);
    return this;
  }

  xyz(x: number, y: number, z: number) {
    op_draw_path_xyz(this.#id, x, y, z);
    return this;
  }

  point(x: number, y: number) {
    op_draw_path_point(this.#id, x, y);
    return this;
  }

  fill(color: Color) {
    switch (color.type) {
      case "rgba": {
        const { r, g, b, a } = color;
        op_draw_path_fill_rgba(this.#id, r, g, b, a);
        break;
      }

      case "hsva": {
        const { h, s, v, a } = color;
        op_draw_path_fill_hsva(this.#id, h, s, v, a);
        break;
      }
    }

    return this;
  }

  tension(t: number) {
    op_draw_path_tension(this.#id, t);
    return this;
  }

  resolution(n: number) {
    if (!Number.isInteger(n)) {
      throw new Error("Resolution must be an integer");
    }

    if (n <= 0) {
      throw new Error("Resolution must be greater than 0");
    }

    op_draw_path_resolution(this.#id, n);
    return this;
  }
}
