const {
  op_draw_background,
  op_draw_ellipse,
  op_draw_ellipse_xy,
  op_draw_ellipse_xyz,
  op_draw_ellipse_wh,
  op_draw_ellipse_rgb,
  op_draw_ellipse_rgba,
  op_draw_ellipse_hsv,
  op_draw_ellipse_hsva,
  op_draw_ellipse_stroke_rgb,
  op_draw_ellipse_stroke_rgba,
  op_draw_ellipse_stroke_hsv,
  op_draw_ellipse_stroke_hsva,
  op_draw_ellipse_stroke_weight,
  op_draw_rect,
  op_draw_rect_xy,
  op_draw_rect_xyz,
  op_draw_rect_wh,
  op_draw_rect_rgb,
  op_draw_rect_rgba,
  op_draw_rect_hsv,
  op_draw_rect_hsva,
  op_draw_rect_stroke_rgb,
  op_draw_rect_stroke_rgba,
  op_draw_rect_stroke_hsv,
  op_draw_rect_stroke_hsva,
  op_draw_rect_stroke_weight,
  op_draw_translate,
  op_draw_rotate,
  op_draw_scale,
} = Deno.core.ops;

class Draw {
  #id;

  constructor(id) {
    this.#id = id;
  }

  copy() {
    return new Draw(this.#id);
  }

  // Background
  clear() {
    op_draw_background(this.#id, 0, 0, 0);
  }
  background(r, g, b) {
    op_draw_background(this.#id, r, g, b);
  }

  // Shapes
  ellipse() {
    const shapeId = op_draw_ellipse(this.#id);
    return new Ellipse(shapeId);
  }
  rect() {
    const shapeId = op_draw_rect(this.#id);
    return new Rect(shapeId);
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
  scale(s) {
    const nextId = op_draw_scale(this.#id, s);
    return new Draw(nextId);
  }
}

class Ellipse {
  #id;

  constructor(id) {
    this.#id = id;
  }

  xy(x, y) {
    op_draw_ellipse_xy(this.#id, x, y);
    return this;
  }

  xyz(x, y, z) {
    op_draw_ellipse_xyz(this.#id, x, y, z);
    return this;
  }

  wh(w, h) {
    op_draw_ellipse_wh(this.#id, w, h);
    return this;
  }

  rgb(r, g, b) {
    op_draw_ellipse_rgb(this.#id, r, g, b);
    return this;
  }

  rgba(r, g, b, a) {
    op_draw_ellipse_rgba(this.#id, r, g, b, a);
    return this;
  }

  hsv(h, s, v) {
    op_draw_ellipse_hsv(this.#id, h, s, v);
    return this;
  }

  hsva(h, s, v, a) {
    op_draw_ellipse_hsva(this.#id, h, s, v, a);
    return this;
  }

  strokeRgb(r, g, b) {
    op_draw_ellipse_stroke_rgb(this.#id, r, g, b);
    return this;
  }

  strokeRgba(r, g, b, a) {
    op_draw_ellipse_stroke_rgba(this.#id, r, g, b, a);
    return this;
  }

  strokeHsv(h, s, v) {
    op_draw_ellipse_stroke_hsv(this.#id, h, s, v);
    return this;
  }

  strokeHsva(h, s, v, a) {
    op_draw_ellipse_stroke_hsva(this.#id, h, s, v, a);
    return this;
  }

  strokeWeight(w) {
    op_draw_ellipse_stroke_weight(this.#id, w);
    return this;
  }
}

class Rect {
  #id;

  constructor(id) {
    this.#id = id;
  }

  xy(x, y) {
    op_draw_rect_xy(this.#id, x, y);
    return this;
  }

  xyz(x, y, z) {
    op_draw_rect_xyz(this.#id, x, y, z);
    return this;
  }

  wh(w, h) {
    op_draw_rect_wh(this.#id, w, h);
    return this;
  }

  rgb(r, g, b) {
    op_draw_rect_rgb(this.#id, r, g, b);
    return this;
  }

  rgba(r, g, b, a) {
    op_draw_rect_rgba(this.#id, r, g, b, a);
    return this;
  }

  hsv(h, s, v) {
    op_draw_rect_hsv(this.#id, h, s, v);
    return this;
  }

  hsva(h, s, v, a) {
    op_draw_rect_hsva(this.#id, h, s, v, a);
    return this;
  }

  strokeRgb(r, g, b) {
    op_draw_rect_stroke_rgb(this.#id, r, g, b);
    return this;
  }

  strokeRgba(r, g, b, a) {
    op_draw_rect_stroke_rgba(this.#id, r, g, b, a);
    return this;
  }

  strokeHsv(h, s, v) {
    op_draw_rect_stroke_hsv(this.#id, h, s, v);
    return this;
  }

  strokeHsva(h, s, v, a) {
    op_draw_rect_stroke_hsva(this.#id, h, s, v, a);
    return this;
  }

  strokeWeight(w) {
    op_draw_rect_stroke_weight(this.#id, w);
    return this;
  }
}

function createDraw() {
  return new Draw(0);
}

globalThis.createDraw = createDraw;
