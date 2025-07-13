import ops from "./ops.ts";

const { op_log_console_log, op_log_console_error } = ops;

const console: Console = {
  log,
  error,
};

function log(...args: Args) {
  const message = argsToMessage(...args);
  return op_log_console_log(message);
}

function error(...args: Args) {
  const message = argsToMessage(...args);
  return op_log_console_error(message);
}

// deno-lint-ignore no-explicit-any
type Args = Array<any>;

function argsToMessage(...args: Args): string {
  return args.map((arg) => JSON.stringify(arg)).join(" ");
}

globalThis.console = console;
