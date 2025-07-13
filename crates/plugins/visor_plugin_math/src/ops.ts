declare namespace Deno {
  const core: {
    ops: {
      op_draw_noise: (x: number, y: number, z: number) => number;
    };
  };
}

export default Deno.core.ops;
