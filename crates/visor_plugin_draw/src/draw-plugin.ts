import "./color.ts";
import { Draw } from "./draw.ts";
import { FullscreenShader as FullscreenShaderClass } from "./fullscreen-shader.ts";
import ops from "./ops.ts";
import { Texture as TextureClass } from "./texture.ts";

const {
  op_draw_width,
  op_draw_height,
  op_draw_texture_load,
  op_draw_fullscreen_shader,
  op_draw_fullscreen_shader_load,
} = ops;

function createDraw() {
  return new Draw(0);
}

function loadTexture(path: string): Texture {
  const textureId = op_draw_texture_load(path);
  return new TextureClass(textureId);
}

function loadFullscreenShader(path: string): FullscreenShader {
  const shaderId = op_draw_fullscreen_shader_load(path);
  return new FullscreenShaderClass(shaderId);
}

function fullscreenShader(shader: FullscreenShader) {
  op_draw_fullscreen_shader(shader.id());
}

globalThis.createDraw = createDraw;
globalThis.loadTexture = loadTexture;
globalThis.loadFullscreenShader = loadFullscreenShader;
globalThis.fullscreenShader = fullscreenShader;

// TODO: move to another plugin where it makes sense
globalThis.width = op_draw_width;
globalThis.height = op_draw_height;
