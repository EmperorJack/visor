declare namespace Deno {
  export const core: {
    ops: {
      op_log_console_log: (message: string) => void;
      op_log_console_error: (message: string) => void;
    };
  };
}

const { op_log_console_log, op_log_console_error } = Deno.core.ops;

// deno-lint-ignore no-explicit-any
type Args = Array<any>;

function argsToMessage(...args: Args): string {
  return args.map((arg) => JSON.stringify(arg)).join(" ");
}

function log(...args: Args) {
  const message = argsToMessage(...args);
  return op_log_console_log(message);
}

function error(...args: Args) {
  const message = argsToMessage(...args);
  return op_log_console_error(message);
}

declare const globalThis: {
  console: {
    log: typeof log;
    error: typeof error;
  };
};

globalThis.console = {
  log,
  error,
};
