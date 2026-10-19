import ops from "./ops.ts";

const { op_draw_texture_width, op_draw_texture_height } = ops;

export class Texture {
  #id: number;

  constructor(id: number) {
    this.#id = id;
  }

  id() {
    return this.#id;
  }

  width() {
    return op_draw_texture_width(this.#id);
  }

  height() {
    return op_draw_texture_height(this.#id);
  }
}
