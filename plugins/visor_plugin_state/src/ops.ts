declare namespace Deno {
  const core: {
    ops: {
      op_state_create: (id: string, value: string) => string;
      op_state_set: (id: string, value: string) => void;
      op_state_remove_unused: (ids: Array<string>) => void;
    };
  };
}

export default Deno.core.ops;
