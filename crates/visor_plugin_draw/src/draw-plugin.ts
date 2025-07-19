import "./color.ts";
import { Draw } from "./draw.ts";
import ops from "./ops.ts";

const { op_draw_width, op_draw_height } = ops;

function createDraw() {
  return new Draw(0);
}

globalThis.createDraw = createDraw;

// TODO: move to another plugin where it makes sense
globalThis.width = op_draw_width;
globalThis.height = op_draw_height;
