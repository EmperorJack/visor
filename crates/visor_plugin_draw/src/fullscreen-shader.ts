import ops from "./ops.ts";

const { op_draw_fullscreen_shader_set_uniform } = ops;

export class FullscreenShader {
  #id: number;

  constructor(id: number) {
    this.#id = id;
  }

  id() {
    return this.#id;
  }

  setUniform(key: string, value: number) {
    op_draw_fullscreen_shader_set_uniform(this.#id, key, value);
  }
}
