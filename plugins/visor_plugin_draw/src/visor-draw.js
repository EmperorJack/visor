const { op_draw_ellipse, op_draw_rect, op_draw_translate, op_draw_rotate } =
  Deno.core.ops;

class Draw {
  #id;

  constructor(id) {
    this.#id = id;
  }

  copy() {
    return new Draw(this.#id);
  }

  // Shapes
  circle(x, y, size) {
    op_draw_ellipse(this.#id, x, y, size, size);
  }
  ellipse(x, y, width, height) {
    op_draw_ellipse(this.#id, x, y, width, height);
  }
  square(x, y, size) {
    op_draw_rect(this.#id, x, y, size, size);
  }
  rect(x, y, width, height) {
    op_draw_rect(this.#id, x, y, width, height);
  }

  // Transforms
  translate(x, y) {
    const nextId = op_draw_translate(this.#id, x, y);
    return new Draw(nextId);
  }
  rotate(radians) {
    const nextId = op_draw_rotate(this.#id, radians);
    return new Draw(nextId);
  }
}

function createDraw() {
  return new Draw(0);
}

globalThis.createDraw = createDraw;
