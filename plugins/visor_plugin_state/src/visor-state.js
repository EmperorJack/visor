const { op_state_create, op_state_get, op_state_set, op_state_remove_unused } =
  Deno.core.ops;

class _StateVariable {
  id;

  constructor(id, defaultValue) {
    this.id = id;

    op_state_create(id, JSON.stringify(defaultValue));
  }

  // TODO: better way to store state than JSON strings
  // e.g: cache in JS and export/import into new runtime on compile?
  get() {
    return JSON.parse(op_state_get(this.id));
  }

  set(value) {
    op_state_set(this.id, JSON.stringify(value));
  }
}

function createState(variables) {
  const stateVariables = Object.entries(variables).reduce(
    (sofar, [key, value]) => {
      sofar[key] = new _StateVariable(key, value);
      return sofar;
    },
    {}
  );

  op_state_remove_unused(Object.keys(stateVariables));

  const handler = {
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

  return new Proxy(stateVariables, handler);
}

globalThis.createState = createState;
