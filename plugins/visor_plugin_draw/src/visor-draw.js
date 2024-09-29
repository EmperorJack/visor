export function circle(x, y, size) {
  const { op_draw_ellipse } = Deno.core.ops;

  op_draw_ellipse(x, y, size, size);
}

export function ellipse(x, y, width, height) {
  const { op_draw_ellipse } = Deno.core.ops;

  op_draw_ellipse(x, y, width, height);
}

export function square(x, y, size) {
  const { op_draw_rect } = Deno.core.ops;

  op_draw_rect(x, y, size, size);
}

export function rect(x, y, width, height) {
  const { op_draw_rect } = Deno.core.ops;

  op_draw_rect(x, y, width, height);
}
