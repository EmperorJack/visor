export function rect(x, y, d) {
  const { op_draw_rect } = Deno.core.ops;

  op_draw_rect(x, y, d);
}
