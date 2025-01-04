const {
  op_midi_input_devices,
  op_midi_connect_input_device,
  op_midi_disconnect_input_device,
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
};

globalThis.midi = midi;
