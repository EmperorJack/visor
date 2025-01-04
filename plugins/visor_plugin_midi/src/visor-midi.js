const { op_midi_input_devices } = Deno.core.ops;

const midi = {
  listInputDevices() {
    return op_midi_input_devices();
  },
};

globalThis.midi = midi;
