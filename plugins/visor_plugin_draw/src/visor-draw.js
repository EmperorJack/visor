const {
  op_draw_background_rgb,
  op_draw_background_hsv,
  op_draw_ellipse,
  op_draw_ellipse_xy,
  op_draw_ellipse_xyz,
  op_draw_ellipse_wh,
  op_draw_ellipse_fill_rgba,
  op_draw_ellipse_fill_hsva,
  op_draw_ellipse_no_fill,
  op_draw_ellipse_stroke_rgba,
  op_draw_ellipse_stroke_hsva,
  op_draw_ellipse_stroke_weight,
  op_draw_rect,
  op_draw_rect_xy,
  op_draw_rect_xyz,
  op_draw_rect_wh,
  op_draw_rect_fill_rgba,
  op_draw_rect_fill_hsva,
  op_draw_rect_no_fill,
  op_draw_rect_stroke_rgba,
  op_draw_rect_stroke_hsva,
  op_draw_rect_stroke_weight,
  op_draw_polyline,
  op_draw_polyline_xyz,
  op_draw_polyline_point,
  op_draw_polyline_stroke_rgba,
  op_draw_polyline_stroke_hsva,
  op_draw_polyline_stroke_weight,
  op_draw_polyline_tension,
  op_draw_spline,
  op_draw_spline_xyz,
  op_draw_spline_point,
  op_draw_spline_stroke_rgba,
  op_draw_spline_stroke_hsva,
  op_draw_spline_stroke_weight,
  op_draw_spline_tension,
  op_draw_spline_resolution,
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
    op_draw_background_rgb(this.#id, 0, 0, 0);
  }
  background(color) {
    switch (color.type) {
      case "rgba": {
        const { r, g, b } = color;
        op_draw_background_rgb(this.#id, r, g, b);
        break;
      }

      case "hsva": {
        const { h, s, v } = color;
        op_draw_background_hsv(this.#id, h, s, v);
        break;
      }
    }
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
  polyline() {
    const shapeId = op_draw_polyline(this.#id);
    return new Polyline(shapeId);
  }
  spline() {
    const shapeId = op_draw_spline(this.#id);
    return new Spline(shapeId);
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

  fill(color) {
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

  stroke(color) {
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

  fill(color) {
    switch (color.type) {
      case "rgba": {
        const { r, g, b, a } = color;
        op_draw_rect_fill_rgba(this.#id, r, g, b, a);
        break;
      }

      case "hsva": {
        const { h, s, v, a } = color;
        op_draw_rect_fill_hsva(this.#id, h, s, v, a);
        break;
      }
    }

    return this;
  }

  noFill() {
    op_draw_rect_no_fill(this.#id);
    return this;
  }

  stroke(color) {
    switch (color.type) {
      case "rgba": {
        const { r, g, b, a } = color;
        op_draw_rect_stroke_rgba(this.#id, r, g, b, a);
        break;
      }

      case "hsva": {
        const { h, s, v, a } = color;
        op_draw_rect_stroke_hsva(this.#id, h, s, v, a);
        break;
      }
    }

    return this;
  }

  strokeWeight(w) {
    op_draw_rect_stroke_weight(this.#id, w);
    return this;
  }
}

class Polyline {
  #id;

  constructor(id) {
    this.#id = id;
  }

  xy(x, y) {
    op_draw_polyline_xyz(this.#id, x, y, 0);
    return this;
  }

  xyz(x, y, z) {
    op_draw_polyline_xyz(this.#id, x, y, z);
    return this;
  }

  point(x, y) {
    op_draw_polyline_point(this.#id, x, y);
    return this;
  }

  stroke(color) {
    switch (color.type) {
      case "rgba": {
        const { r, g, b, a } = color;
        op_draw_polyline_stroke_rgba(this.#id, r, g, b, a);
        break;
      }

      case "hsva": {
        const { h, s, v, a } = color;
        op_draw_polyline_stroke_hsva(this.#id, h, s, v, a);
        break;
      }
    }

    return this;
  }

  strokeWeight(w) {
    op_draw_polyline_stroke_weight(this.#id, w);
    return this;
  }
}

class Spline {
  #id;

  constructor(id) {
    this.#id = id;
  }

  xy(x, y) {
    op_draw_spline_xyz(this.#id, x, y, 0);
    return this;
  }

  xyz(x, y, z) {
    op_draw_spline_xyz(this.#id, x, y, z);
    return this;
  }

  point(x, y) {
    op_draw_spline_point(this.#id, x, y);
    return this;
  }

  stroke(color) {
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

  strokeWeight(w) {
    op_draw_spline_stroke_weight(this.#id, w);
    return this;
  }

  tension(t) {
    op_draw_spline_tension(this.#id, t);
    return this;
  }

  resolution(n) {
    op_draw_spline_resolution(this.#id, n);
    return this;
  }
}

function rgb(r, g, b) {
  return { type: "rgba", r, g, b, a: 1.0 };
}

function rgba(r, g, b, a) {
  return { type: "rgba", r, g, b, a };
}

function hsv(h, s, v) {
  return { type: "hsva", h, s, v, a: 1.0 };
}

function hsva(h, s, v, a) {
  return { type: "hsva", h, s, v, a };
}

function createDraw() {
  return new Draw(0);
}

globalThis.createDraw = createDraw;
globalThis.rgb = rgb;
globalThis.rgba = rgba;
globalThis.hsv = hsv;
globalThis.hsva = hsva;
