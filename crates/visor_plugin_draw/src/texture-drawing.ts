import ops from "./ops.ts";

const {
  op_draw_texture_xy,
  op_draw_texture_xyz,
  op_draw_texture_wh,
  op_draw_texture_tint,
} = ops;

export class TextureDrawing {
  #id: number;

  constructor(id: number) {
    this.#id = id;
  }

  xy(x: number, y: number) {
    op_draw_texture_xy(this.#id, x, y);
    return this;
  }

  xyz(x: number, y: number, z: number) {
    op_draw_texture_xyz(this.#id, x, y, z);
    return this;
  }

  wh(w: number, h: number) {
    op_draw_texture_wh(this.#id, w, h);
    return this;
  }

  tint(r: number, g: number, b: number, a: number) {
    op_draw_texture_tint(this.#id, r, g, b, a);
    return this;
  }
}
