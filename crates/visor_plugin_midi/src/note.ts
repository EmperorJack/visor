import ops from "./ops.ts";

const {
  op_midi_note_on,
  op_midi_note_off,
  op_midi_note_down,
  op_midi_note_velocity,
} = ops;

export class Note {
  #id: string;

  constructor(id: string) {
    this.#id = id;
  }

  on() {
    return op_midi_note_on(this.#id);
  }

  off() {
    return op_midi_note_off(this.#id);
  }

  down() {
    return op_midi_note_down(this.#id);
  }

  velocity() {
    return op_midi_note_velocity(this.#id);
  }
}
