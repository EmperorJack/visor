import ops from "./ops.ts";

const { op_midi_encoder_increment, op_midi_encoder_decrement } = ops;

export class Encoder {
  #id: string;

  constructor(id: string) {
    this.#id = id;
  }

  increment() {
    return op_midi_encoder_increment(this.#id);
  }

  decrement() {
    return op_midi_encoder_decrement(this.#id);
  }
}
