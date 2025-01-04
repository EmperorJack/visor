use std::collections::HashMap;

use deno_core::{
    anyhow::{anyhow, Result},
    extension, op2, Extension, OpState,
};
use midir::{MidiInput, MidiInputConnection};
use midly::live::LiveEvent;
use visor_engine::{
    engine::Engine,
    plugin::{AccessSketchStore, Plugin},
    sketch::SketchId,
    sketch_store::SketchStore,
    store::Store,
};

pub struct MidiPlugin;

struct SketchState {
    input_connections: HashMap<String, MidiInputConnection<()>>,
}

extension!(
    extension,
    ops = [op_midi_input_devices, op_midi_connect_input_device, op_midi_disconnect_input_device],
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

    fn build_sketch(
        &self,
        _sketch_id: &SketchId,
        _engine: &mut Engine,
        _store: &Store,
        sketch_store: &mut SketchStore,
    ) {
        sketch_store.set(SketchState {
            input_connections: Default::default(),
        });
    }
}

fn create_midi_input() -> Result<MidiInput> {
    Ok(MidiInput::new("Visor plugin MIDI input")?)
}

#[op2]
#[serde]
fn op_midi_input_devices() -> Result<Vec<String>> {
    let midi_input = create_midi_input()?;

    let input_devices = midi_input
        .ports()
        .iter()
        .filter_map(|port| midi_input.port_name(port).ok())
        .collect();

    Ok(input_devices)
}

#[op2(fast)]
fn op_midi_connect_input_device(state: &mut OpState, #[string] name: String) -> Result<()> {
    let state = state.sketch_store_mut().get_mut::<SketchState>();

    let midi_input = create_midi_input()?;

    let input_ports = midi_input.ports();

    let port = input_ports.iter().find(|port| {
        if let Ok(port_name) = midi_input.port_name(port) {
            port_name == name
        } else {
            false
        }
    });

    let Some(port) = port else {
        return Err(anyhow!("MIDI input port for {} could not be found", name));
    };

    if state.input_connections.contains_key(&name) {
        return Err(anyhow!("MIDI input device {} already connected", name));
    }

    let input_connection = midi_input.connect(
        port,
        &format!("Visor plugin MIDI input connection to {}", name),
        |_timestamp, event, _| {
            let event = LiveEvent::parse(event).ok();

            if let Some(event) = event {
                println!("{:?}", event);
            }
        },
        (),
    )?;

    state.input_connections.insert(name, input_connection);

    Ok(())
}

#[op2(fast)]
fn op_midi_disconnect_input_device(state: &mut OpState, #[string] name: String) -> Result<()> {
    let state = state.sketch_store_mut().get_mut::<SketchState>();

    let Some(input_connection) = state.input_connections.remove(&name) else {
        return Err(anyhow!("MIDI input device {} is not connected", name));
    };

    input_connection.close();

    Ok(())
}
