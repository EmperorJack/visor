declare function createState<S extends State>(variable: S): S;

// deno-lint-ignore no-explicit-any
type State = Record<string | number | symbol, any>;
