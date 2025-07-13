function rgb(r: number, g: number, b: number): Color {
  return { type: "rgba", r, g, b, a: 1.0 };
}

function rgba(r: number, g: number, b: number, a: number): Color {
  return { type: "rgba", r, g, b, a };
}

function hsv(h: number, s: number, v: number): Color {
  return { type: "hsva", h, s, v, a: 1.0 };
}

function hsva(h: number, s: number, v: number, a: number): Color {
  return { type: "hsva", h, s, v, a };
}

globalThis.rgb = rgb;
globalThis.rgba = rgba;
globalThis.hsv = hsv;
globalThis.hsva = hsva;
