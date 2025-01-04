use deno_core::{extension, op2, Extension};
use midir::MidiInput;
use visor_engine::plugin::Plugin;

pub struct MidiPlugin;

extension!(
    extension,
    ops = [op_midi_input_devices],
    esm_entry_point = "visor:midi",
    esm = [
        dir "src",
        "visor:midi" = "visor-midi.js",
    ]
);

impl Plugin for MidiPlugin {
    fn extension(&self) -> Extension {
        extension::init_ops_and_esm()
    }
}

fn create_midi_input() -> MidiInput {
    MidiInput::new("Visor plugin MIDI input").expect("Error: could not create MIDI input")
}

#[op2]
#[serde]
fn op_midi_input_devices() -> Vec<String> {
    let midi_input = create_midi_input();

    midi_input
        .ports()
        .iter()
        .filter_map(|port| midi_input.port_name(port).ok())
        .collect()
}
