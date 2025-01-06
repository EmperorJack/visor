const {
  op_midi_input_devices,
  op_midi_connect_input_device,
  op_midi_disconnect_input_device,
  op_midi_load_mapping,
  op_midi_control_value,
  op_midi_encoder_increment,
  op_midi_encoder_decrement,
  op_midi_note_on,
  op_midi_note_off,
  op_midi_note_down,
  op_midi_note_velocity,
} = Deno.core.ops;

const midi = {
  listInputDevices() {
    return op_midi_input_devices();
  },

  connectInputDevice(name) {
    op_midi_connect_input_device(name);
  },

  disconnectInputDevice(name) {
    op_midi_disconnect_input_device(name);
  },

  loadMapping(path) {
    op_midi_load_mapping(path);
  },

  note(id) {
    return new Note(id);
  },

  encoder(id) {
    return new Encoder(id);
  },

  control(id) {
    return op_midi_control_value(id);
  },
};

class Encoder {
  #id;

  constructor(id) {
    this.#id = id;
  }

  increment() {
    return op_midi_encoder_increment(this.#id);
  }

  decrement() {
    return op_midi_encoder_decrement(this.#id);
  }
}

class Note {
  #id;

  constructor(id) {
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

globalThis.midi = midi;
