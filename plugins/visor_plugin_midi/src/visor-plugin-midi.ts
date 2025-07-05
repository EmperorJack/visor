declare namespace Deno {
  const core: {
    ops: {
      op_midi_input_devices: () => Array<string>;
      op_midi_connect_input_device: (name: string) => void;
      op_midi_disconnect_input_device: (name: string) => void;
      op_midi_load_mapping: (path: string) => void;
      op_midi_clear_mapping: () => void;
      op_midi_control_value: (id: string) => number;
      op_midi_encoder_increment: (id: string) => boolean;
      op_midi_encoder_decrement: (id: string) => boolean;
      op_midi_note_on: (id: string) => boolean;
      op_midi_note_off: (id: string) => boolean;
      op_midi_note_down: (id: string) => boolean;
      op_midi_note_velocity: (id: string) => number;
    };
  };
}

const {
  op_midi_input_devices,
  op_midi_connect_input_device,
  op_midi_disconnect_input_device,
  op_midi_load_mapping,
  op_midi_clear_mapping,
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

  connectInputDevice(name: string) {
    op_midi_connect_input_device(name);
  },

  disconnectInputDevice(name: string) {
    op_midi_disconnect_input_device(name);
  },

  loadMapping(path: string) {
    op_midi_load_mapping(path);
  },

  clearMapping() {
    op_midi_clear_mapping();
  },

  note(id: string) {
    return new Note(id);
  },

  encoder(id: string) {
    return new Encoder(id);
  },

  control(id: string) {
    return op_midi_control_value(id);
  },
};

class Encoder {
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

class Note {
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

globalThis.midi = midi;
