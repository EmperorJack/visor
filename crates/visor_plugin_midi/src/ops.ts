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

export default Deno.core.ops;
