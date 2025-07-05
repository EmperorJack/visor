// TODO: move the following to a math plugin?

import ops from "./ops.ts";

const { op_draw_noise } = ops;

function random(amount: number) {
  return Math.random() * amount;
}

function radians(degrees: number) {
  return degrees * (Math.PI / 180);
}

function degrees(radians: number) {
  return radians * (180 / Math.PI);
}

function lerp(start: number, stop: number, amount: number) {
  return start + (stop - start) * amount;
}

function norm(value: number, start: number, stop: number) {
  return (value - start) / (stop - start);
}

function map(
  value: number,
  start1: number,
  stop1: number,
  start2: number,
  stop2: number
) {
  return start2 + (stop2 - start2) * ((value - start1) / (stop1 - start1));
}

function constrain(value: number, min: number, max: number) {
  if (value < min) {
    return min;
  }

  if (value > max) {
    return max;
  }

  return value;
}

function noise(x: number, y = 0, z = 0) {
  return op_draw_noise(x, y, z);
}

globalThis.random = random;
globalThis.radians = radians;
globalThis.degrees = degrees;
globalThis.lerp = lerp;
globalThis.norm = norm;
globalThis.map = map;
globalThis.constrain = constrain;
globalThis.noise = noise;
