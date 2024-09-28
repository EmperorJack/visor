function argsToMessage(...args) {
  return args.map((arg) => JSON.stringify(arg)).join(" ");
}

export function log(...args) {
  const { op_log_console_log } = Deno.core.ops;

  const message = argsToMessage(...args);
  return op_log_console_log(message);
}

export function error(...args) {
  const { op_log_console_error } = Deno.core.ops;

  const message = argsToMessage(...args);
  return op_log_console_error(message);
}
