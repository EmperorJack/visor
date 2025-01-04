const { op_counter_count, op_counter_increment } = Deno.core.ops;

export function count() {
  return op_counter_count();
}

export function increment() {
  op_counter_increment();
}
