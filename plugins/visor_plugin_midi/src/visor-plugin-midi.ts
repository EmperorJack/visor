import { Encoder } from "./encoder.ts";
import { Note } from "./note.ts";
import ops from "./ops.ts";

const {
  op_midi_input_devices,
  op_midi_connect_input_device,
  op_midi_disconnect_input_device,
  op_midi_load_mapping,
  op_midi_clear_mapping,
  op_midi_control_value,
} = ops;

const midi: Midi = {
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

globalThis.midi = midi;
