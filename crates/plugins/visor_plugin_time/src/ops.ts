declare namespace Deno {
  const core: {
    ops: {
      op_time_frame_count: () => number;
      op_time_time: () => number;
      op_time_delta: () => number;
    };
  };
}

export default Deno.core.ops;
