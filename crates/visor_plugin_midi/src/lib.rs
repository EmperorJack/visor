use std::{collections::HashMap, path::PathBuf, sync::RwLock};

use config::MidiMappingConfig;
use deno_core::{
    Extension, OpState,
    anyhow::{Result, anyhow},
    extension, op2,
};
use deno_error::JsErrorBox;
pub use mapping::MidiMappingEvent;
use mapping::{MidiMapping, MidiVariables};
use midir::{MidiInput, MidiInputConnection};
use midly::{MidiMessage, live::LiveEvent};
use tokio::sync::{broadcast, mpsc};
use visor_engine::{AccessSketchStore, Engine, Plugin, SketchId, SketchStore, Store};

mod config;
mod control;
mod encoder;
mod mapping;
mod note;

pub struct MidiPlugin;

struct State {
    event_sender: mpsc::Sender<Event>,
    event_receiver: mpsc::Receiver<Event>,
    input_connections: HashMap<String, InputConnection>,
    // TODO: allow multiple labelled mappings e.g: per input device
    midi_mapping: Option<MidiMapping>,
}

impl State {
    fn process_midi_messages(&mut self) {
        for input_connection in self.input_connections.values_mut() {
            while let Ok((channel, message)) = input_connection.message_receiver.try_recv() {
                if let Some(ref mut midi_mapping) = self.midi_mapping {
                    match message {
                        MidiMessage::NoteOff { key, vel } => {
                            midi_mapping.note_off(channel, key.into(), vel.into());
                        }
                        MidiMessage::NoteOn { key, vel } => {
                            midi_mapping.note_on(channel, key.into(), vel.into());
                        }
                        MidiMessage::Controller { controller, value } => {
                            midi_mapping.controller_changed(
                                channel,
                                controller.into(),
                                value.into(),
                            );
                        }
                        _ => {}
                    };
                }
            }
        }
    }
}

struct InputConnection {
    connection: MidiInputConnection<mpsc::Sender<(u8, MidiMessage)>>,
    message_receiver: mpsc::Receiver<(u8, MidiMessage)>,
}

struct SketchState {
    input_connections: Vec<String>,
    variables: Option<MidiVariables>,
}

enum Event {
    AddInputConnection(String, InputConnection),
    RemoveInputConnection(String),
    LoadMapping(Box<MidiMapping>),
    ClearMapping,
}

extension!(
    visor_plugin_midi,
    ops = [
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
    ],
    esm_entry_point = "ext:visor_plugin_midi/src/visor-plugin-midi.ts",
    esm = [
        "src/encoder.ts",
        "src/note.ts",
        "src/ops.ts",
        "src/visor-plugin-midi.ts"
    ]
);

impl MidiPlugin {
    pub fn list_input_devices() -> Result<Vec<String>> {
        list_input_devices()
    }

    pub fn connect_input_device(store: &Store, name: String) -> Result<()> {
        let mut state = store
            .get::<RwLock<State>>()
            .write()
            .expect("Unexpected: could not acquire write lock for state");

        if state.input_connections.contains_key(&name) {
            return Err(anyhow!("MIDI input device {} already connected", name));
        }

        let input_connection = create_input_connection(name.clone())?;

        state.input_connections.insert(name, input_connection);

        Ok(())
    }

    pub fn disconnect_input_device(store: &Store, name: String) {
        let mut state = store
            .get::<RwLock<State>>()
            .write()
            .expect("Unexpected: could not acquire write lock for state");

        if let Some(input_connection) = state.input_connections.remove(&name) {
            input_connection.connection.close();
        }
    }

    pub fn load_midi_mapping(store: &Store, path: PathBuf) -> Result<()> {
        let mut state = store
            .get::<RwLock<State>>()
            .write()
            .expect("Unexpected: could not acquire write lock for state");

        let midi_mapping = load_midi_mapping(path)?;

        state.midi_mapping = Some(midi_mapping);

        Ok(())
    }

    pub fn clear_midi_mapping(store: &Store) {
        let mut state = store
            .get::<RwLock<State>>()
            .write()
            .expect("Unexpected: could not acquire write lock for state");

        state.midi_mapping = None;
    }

    pub fn control_value(store: &Store, name: &str) -> Result<f32> {
        let state = store
            .get::<RwLock<State>>()
            .read()
            .expect("Unexpected: could not acquire read lock for state");

        let Some(ref midi_mapping) = state.midi_mapping else {
            return Err(anyhow!("No MIDI variable mapping loaded"));
        };

        midi_mapping.variables().control_value(name)
    }

    pub fn is_encoder_increment(store: &Store, name: &str) -> Result<bool> {
        let state = store
            .get::<RwLock<State>>()
            .read()
            .expect("Unexpected: could not acquire read lock for state");

        let Some(ref midi_mapping) = state.midi_mapping else {
            return Err(anyhow!("No MIDI variable mapping loaded"));
        };

        midi_mapping.variables().is_encoder_increment(name)
    }

    pub fn is_encoder_decrement(store: &Store, name: &str) -> Result<bool> {
        let state = store
            .get::<RwLock<State>>()
            .read()
            .expect("Unexpected: could not acquire read lock for state");

        let Some(ref midi_mapping) = state.midi_mapping else {
            return Err(anyhow!("No MIDI variable mapping loaded"));
        };

        midi_mapping.variables().is_encoder_decrement(name)
    }

    pub fn is_note_on(store: &Store, name: &str) -> Result<bool> {
        let state = store
            .get::<RwLock<State>>()
            .read()
            .expect("Unexpected: could not acquire read lock for state");

        let Some(ref midi_mapping) = state.midi_mapping else {
            return Err(anyhow!("No MIDI variable mapping loaded"));
        };

        midi_mapping.variables().is_note_on(name)
    }

    pub fn is_note_off(store: &Store, name: &str) -> Result<bool> {
        let state = store
            .get::<RwLock<State>>()
            .read()
            .expect("Unexpected: could not acquire read lock for state");

        let Some(ref midi_mapping) = state.midi_mapping else {
            return Err(anyhow!("No MIDI variable mapping loaded"));
        };

        midi_mapping.variables().is_note_off(name)
    }

    pub fn is_note_down(store: &Store, name: &str) -> Result<bool> {
        let state = store
            .get::<RwLock<State>>()
            .read()
            .expect("Unexpected: could not acquire read lock for state");

        let Some(ref midi_mapping) = state.midi_mapping else {
            return Err(anyhow!("No MIDI variable mapping loaded"));
        };

        midi_mapping.variables().is_note_down(name)
    }

    pub fn note_velocity(store: &Store, name: &str) -> Result<f32> {
        let state = store
            .get::<RwLock<State>>()
            .read()
            .expect("Unexpected: could not acquire read lock for state");

        let Some(ref midi_mapping) = state.midi_mapping else {
            return Err(anyhow!("No MIDI variable mapping loaded"));
        };

        midi_mapping.variables().note_velocity(name)
    }

    pub fn subscribe_mapping_event(
        store: &Store,
    ) -> Result<broadcast::Receiver<(String, MidiMappingEvent)>> {
        let mut state = store
            .get::<RwLock<State>>()
            .write()
            .expect("Unexpected: could not acquire write lock for state");

        let Some(ref mut midi_mapping) = state.midi_mapping else {
            return Err(anyhow!("No MIDI variable mapping loaded"));
        };

        Ok(midi_mapping.subscribe())
    }
}

impl Plugin for MidiPlugin {
    fn extension(&self) -> Extension {
        visor_plugin_midi::init()
    }

    fn typescript_declaration(&self) -> Option<String> {
        Some(include_str!("visor-plugin-midi.d.ts").into())
    }

    fn build(&self, _engine: &mut Engine, store: &Store) {
        let (event_sender, event_receiver) = mpsc::channel::<Event>(64);

        store.set(RwLock::new(State {
            event_sender,
            event_receiver,
            input_connections: Default::default(),
            midi_mapping: None,
        }));
    }

    fn build_sketch(
        &self,
        _sketch_id: &SketchId,
        _engine: &mut Engine,
        store: &Store,
        sketch_store: &mut SketchStore,
    ) {
        let state = store
            .get::<RwLock<State>>()
            .read()
            .expect("Unexpected: could not acquire read lock for state");

        sketch_store.set(state.event_sender.clone());
    }

    fn before_engine_update(&self, _engine: &mut Engine, store: &Store) {
        let mut state = store
            .get::<RwLock<State>>()
            .write()
            .expect("Unexpected: could not acquire write lock for state");

        state.process_midi_messages();

        while let Ok(event) = state.event_receiver.try_recv() {
            match event {
                Event::AddInputConnection(name, input_connection) => {
                    state.input_connections.insert(name, input_connection);
                }
                Event::RemoveInputConnection(name) => {
                    if let Some(input_connection) = state.input_connections.remove(&name) {
                        input_connection.connection.close();
                    }
                }
                Event::LoadMapping(midi_mapping) => state.midi_mapping = Some(*midi_mapping),
                Event::ClearMapping => state.midi_mapping = None,
            }
        }
    }

    fn before_sketch_update(
        &self,
        _sketch_id: &SketchId,
        store: &Store,
        sketch_store: &mut SketchStore,
    ) {
        let state = store
            .get::<RwLock<State>>()
            .read()
            .expect("Unexpected: could not acquire read lock for state");

        let sketch_state = SketchState {
            input_connections: state.input_connections.keys().cloned().collect(),
            variables: state
                .midi_mapping
                .as_ref()
                .map(|midi_mapping| midi_mapping.variables().clone()),
        };

        sketch_store.set(sketch_state);
    }

    fn after_engine_update(&self, _engine: &mut Engine, store: &Store) {
        let mut state = store
            .get::<RwLock<State>>()
            .write()
            .expect("Unexpected: could not acquire write lock for state");

        if let Some(ref mut midi_mapping) = state.midi_mapping {
            midi_mapping.after_sketch_update();
        }
    }
}

fn create_midi_input() -> Result<MidiInput> {
    Ok(MidiInput::new("Visor plugin MIDI input")?)
}

fn list_input_devices() -> Result<Vec<String>> {
    let midi_input = create_midi_input()?;

    let input_devices = midi_input
        .ports()
        .iter()
        .filter_map(|port| midi_input.port_name(port).ok())
        .collect();

    Ok(input_devices)
}

fn create_input_connection(name: String) -> Result<InputConnection> {
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

    let (message_sender, message_receiver) = mpsc::channel::<(u8, MidiMessage)>(1024);

    let connection = midi_input.connect(
        port,
        &format!("Visor plugin MIDI input connection to {}", name),
        |_timestamp, event, message_sender| {
            let event = LiveEvent::parse(event).ok();

            if let Some(LiveEvent::Midi { channel, message }) = event {
                message_sender
                    .try_send((channel.into(), message))
                    .expect("Unexpected: could not send to midi message channel");
            }
        },
        message_sender,
    )?;

    Ok(InputConnection {
        connection,
        message_receiver,
    })
}

fn load_midi_mapping(path: PathBuf) -> Result<MidiMapping> {
    let contents = std::fs::read_to_string(path)?;

    let mapping_config: MidiMappingConfig = serde_json::from_str(&contents)?;

    Ok(mapping_config.into())
}

#[op2]
#[serde]
fn op_midi_input_devices() -> Result<Vec<String>, JsErrorBox> {
    list_input_devices().map_err(|error| JsErrorBox::generic(error.to_string()))
}

#[op2(fast)]
fn op_midi_connect_input_device(
    state: &mut OpState,
    #[string] name: String,
) -> Result<(), JsErrorBox> {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    if sketch_state.input_connections.contains(&name) {
        return Err(JsErrorBox::generic(format!(
            "MIDI input device {} already connected",
            name
        )));
    }

    let input_connection = create_input_connection(name.clone())
        .map_err(|error| JsErrorBox::generic(error.to_string()))?;

    let event_sender = state.sketch_store().get::<mpsc::Sender<Event>>();
    event_sender
        .try_send(Event::AddInputConnection(name, input_connection))
        .expect("Unexpected: could not send midi plugin event");

    Ok(())
}

#[op2(fast)]
fn op_midi_disconnect_input_device(
    state: &mut OpState,
    #[string] name: String,
) -> Result<(), JsErrorBox> {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    if !sketch_state.input_connections.contains(&name) {
        return Err(JsErrorBox::generic(format!(
            "MIDI input device {} is not connected",
            name
        )));
    }

    let event_sender = state.sketch_store().get::<mpsc::Sender<Event>>();
    event_sender
        .try_send(Event::RemoveInputConnection(name))
        .expect("Unexpected: could not send midi plugin event");

    Ok(())
}

#[op2(fast)]
fn op_midi_load_mapping(state: &mut OpState, #[string] path: String) -> Result<(), JsErrorBox> {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    let path: PathBuf = path
        .parse()
        .map_err(|_| JsErrorBox::generic("Could not parse path"))?;

    let midi_mapping =
        load_midi_mapping(path).map_err(|error| JsErrorBox::generic(error.to_string()))?;

    sketch_state.variables = Some(midi_mapping.variables().clone());

    let event_sender = state.sketch_store().get::<mpsc::Sender<Event>>();
    event_sender
        .try_send(Event::LoadMapping(Box::new(midi_mapping)))
        .expect("Unexpected: could not send midi plugin event");

    Ok(())
}

#[op2(fast)]
fn op_midi_clear_mapping(state: &mut OpState) -> Result<(), JsErrorBox> {
    let event_sender = state.sketch_store().get::<mpsc::Sender<Event>>();
    event_sender
        .try_send(Event::ClearMapping)
        .expect("Unexpected: could not send midi plugin event");

    Ok(())
}

#[op2(fast)]
fn op_midi_control_value(state: &mut OpState, #[string] name: String) -> Result<f32, JsErrorBox> {
    let sketch_state = state.sketch_store().get::<SketchState>();

    let Some(ref variables) = sketch_state.variables else {
        return Err(JsErrorBox::generic("No MIDI variable mapping loaded"));
    };

    variables
        .control_value(&name)
        .map_err(|error| JsErrorBox::generic(error.to_string()))
}

#[op2(fast)]
fn op_midi_encoder_increment(
    state: &mut OpState,
    #[string] name: String,
) -> Result<bool, JsErrorBox> {
    let sketch_state = state.sketch_store().get::<SketchState>();

    let Some(ref variables) = sketch_state.variables else {
        return Err(JsErrorBox::generic("No MIDI variable mapping loaded"));
    };

    variables
        .is_encoder_increment(&name)
        .map_err(|error| JsErrorBox::generic(error.to_string()))
}

#[op2(fast)]
fn op_midi_encoder_decrement(
    state: &mut OpState,
    #[string] name: String,
) -> Result<bool, JsErrorBox> {
    let sketch_state = state.sketch_store().get::<SketchState>();

    let Some(ref variables) = sketch_state.variables else {
        return Err(JsErrorBox::generic("No MIDI variable mapping loaded"));
    };

    variables
        .is_encoder_decrement(&name)
        .map_err(|error| JsErrorBox::generic(error.to_string()))
}

#[op2(fast)]
fn op_midi_note_on(state: &mut OpState, #[string] name: String) -> Result<bool, JsErrorBox> {
    let sketch_state = state.sketch_store().get::<SketchState>();

    let Some(ref variables) = sketch_state.variables else {
        return Err(JsErrorBox::generic("No MIDI variable mapping loaded"));
    };

    variables
        .is_note_on(&name)
        .map_err(|error| JsErrorBox::generic(error.to_string()))
}

#[op2(fast)]
fn op_midi_note_off(state: &mut OpState, #[string] name: String) -> Result<bool, JsErrorBox> {
    let sketch_state = state.sketch_store().get::<SketchState>();

    let Some(ref variables) = sketch_state.variables else {
        return Err(JsErrorBox::generic("No MIDI variable mapping loaded"));
    };

    variables
        .is_note_off(&name)
        .map_err(|error| JsErrorBox::generic(error.to_string()))
}

#[op2(fast)]
fn op_midi_note_down(state: &mut OpState, #[string] name: String) -> Result<bool, JsErrorBox> {
    let sketch_state = state.sketch_store().get::<SketchState>();

    let Some(ref variables) = sketch_state.variables else {
        return Err(JsErrorBox::generic("No MIDI variable mapping loaded"));
    };

    variables
        .is_note_down(&name)
        .map_err(|error| JsErrorBox::generic(error.to_string()))
}

#[op2(fast)]
fn op_midi_note_velocity(state: &mut OpState, #[string] name: String) -> Result<f32, JsErrorBox> {
    let sketch_state = state.sketch_store().get::<SketchState>();

    let Some(ref variables) = sketch_state.variables else {
        return Err(JsErrorBox::generic("No MIDI variable mapping loaded"));
    };

    variables
        .note_velocity(&name)
        .map_err(|error| JsErrorBox::generic(error.to_string()))
}
