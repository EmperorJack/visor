declare function createState<S extends State>(state: S): S;

// deno-lint-ignore no-explicit-any
type State = Record<string | number | symbol, any>;
