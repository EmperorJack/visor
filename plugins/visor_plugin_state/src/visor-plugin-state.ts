// deno-lint-ignore-file no-explicit-any

import ops from "./ops.ts";

const { op_state_create, op_state_set, op_state_remove_unused } = ops;

function createState<S extends State>(variables: S) {
  const stateVariables = Object.entries(variables).reduce<
    Record<string, StateVariable>
  >((sofar, [key, value]) => {
    sofar[key] = new StateVariable(key, value);
    return sofar;
  }, {});

  op_state_remove_unused(Object.keys(stateVariables));

  const handler: ProxyHandler<S> = {
    get(target, prop, receiver) {
      if (Object.keys(target).includes(prop.toString())) {
        return target[prop].get();
      }

      return Reflect.get(target, prop, receiver);
    },

    set(target, prop, newValue, receiver) {
      if (Object.keys(target).includes(prop.toString())) {
        target[prop].set(newValue);
        return true;
      }

      return Reflect.set(target, prop, newValue, receiver);
    },
  };

  return new Proxy(stateVariables, handler) as S;
}

class StateVariable {
  id: string;
  value: any;

  constructor(id: string, defaultValue: any) {
    this.id = id;

    this.value = JSON.parse(op_state_create(id, JSON.stringify(defaultValue)));
  }

  get() {
    return this.value;
  }

  set(value: any) {
    this.value = value;

    // TODO: can we persist state between runtime compiles without saving a JSON string every set()?
    op_state_set(this.id, JSON.stringify(value));
  }
}

globalThis.createState = createState;
