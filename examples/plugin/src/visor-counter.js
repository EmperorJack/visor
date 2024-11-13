export function count() {
  const { op_counter_count } = Deno.core.ops;

  return op_counter_count();
}

export function increment() {
  const { op_counter_increment } = Deno.core.ops;

  op_counter_increment();
}
