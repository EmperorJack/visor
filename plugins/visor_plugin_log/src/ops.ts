declare namespace Deno {
  const core: {
    ops: {
      op_log_console_log: (message: string) => void;
      op_log_console_error: (message: string) => void;
    };
  };
}

export default Deno.core.ops;
