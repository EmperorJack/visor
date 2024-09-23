export function frameCount() {
  const { op_time_frame_count } = Deno.core.ops;

  return op_time_frame_count();
}

export function time() {
  const { op_time_time } = Deno.core.ops;

  return op_time_time();
}
