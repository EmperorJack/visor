declare function random(amount: number): number;
declare function radians(degrees: number): number;
declare function degrees(radians: number): number;
declare function lerp(start: number, stop: number, amount: number): number;
declare function norm(value: number, start: number, stop: number): number;
declare function map(
  value: number,
  start1: number,
  stop1: number,
  start2: number,
  stop2: number
): number;
declare function constrain(value: number, min: number, max: number): number;
declare function noise(x: number, y?: number, z?: number): number;
