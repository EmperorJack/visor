import { Draw } from "./draw.ts";

import "./color.ts";
import "./math.ts";

function createDraw() {
  return new Draw(0);
}

globalThis.createDraw = createDraw;
