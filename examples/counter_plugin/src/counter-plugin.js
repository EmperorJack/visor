const { op_counter_count, op_counter_increment } = Deno.core.ops;

function count() {
  return op_counter_count();
}

function increment() {
  op_counter_increment();
}

// TODO: make an example that exports the functions instead of using globalThis
globalThis.count = count;
globalThis.increment = increment;
