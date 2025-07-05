declare namespace Deno {
  const core: {
    ops: {
      op_time_frame_count: () => number;
      op_time_time: () => number;
      op_time_delta: () => number;
    };
  };
}

const { op_time_frame_count, op_time_time, op_time_delta } = Deno.core.ops;

function frameCount() {
  return op_time_frame_count();
}

function time() {
  return op_time_time();
}

function delta() {
  return op_time_delta();
}

globalThis.frameCount = frameCount;
globalThis.time = time;
globalThis.delta = delta;
