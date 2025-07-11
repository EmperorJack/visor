import { Draw } from "./draw.ts";

import "./color.ts";

function createDraw() {
  return new Draw(0);
}

globalThis.createDraw = createDraw;
