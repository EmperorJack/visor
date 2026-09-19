import "./color.ts";
import { Draw } from "./draw.ts";
import { FullscreenShader } from "./fullscreen-shader.ts";
import ops from "./ops.ts";

const { op_draw_width, op_draw_height, op_draw_fullscreen_shader_load } = ops;

function createDraw() {
  return new Draw(0);
}

function loadFullscreenShader(path: string): FullscreenShader {
  const shaderId = op_draw_fullscreen_shader_load(path);
  return new FullscreenShader(shaderId);
}

globalThis.createDraw = createDraw;
globalThis.loadFullscreenShader = loadFullscreenShader;

// TODO: move to another plugin where it makes sense
globalThis.width = op_draw_width;
globalThis.height = op_draw_height;
