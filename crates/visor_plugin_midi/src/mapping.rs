use deno_core::anyhow::{Result, anyhow};
use std::collections::HashMap;
use tokio::sync::broadcast;

use crate::{
    config::{MidiMappingConfig, MidiVariableConfig},
    control::MidiControl,
    encoder::MidiEncoder,
    note::MidiNote,
};

pub(crate) struct MidiMapping {
    event_sender: Option<broadcast::Sender<(String, MidiMappingEvent)>>,
    variable_name_mapping: MidiVariableNameMapping,
    variables: MidiVariables,
}

impl MidiMapping {
    pub fn controller_changed(&mut self, channel: u8, number: u8, value: u8) {
        if let Some(names) = self
            .variable_name_mapping
            .control_mapping
            .get(&(channel, number))
        {
            for name in names {
                let variable = self
                    .variables
                    .controls
                    .get_mut(name)
                    .expect("Unexpected: could not find midi variable");

                variable.changed(value);

                if let Some(event_sender) = self.event_sender.as_ref() {
                    event_sender
                        .send((
                            name.into(),
                            MidiMappingEvent::ControllerChanged(variable.value()),
                        ))
                        .ok();
                }
            }
        }

        if let Some(names) = self
            .variable_name_mapping
            .encoder_mapping
            .get(&(channel, number))
        {
            for name in names {
                let variable = self
                    .variables
                    .encoders
                    .get_mut(name)
                    .expect("Unexpected: could not find midi variable");

                variable.changed(value);

                if let Some(event_sender) = self.event_sender.as_ref() {
                    if variable.is_increment() {
                        event_sender
                            .send((name.into(), MidiMappingEvent::EncoderIncrement))
                            .ok();
                    } else if variable.is_decrement() {
                        event_sender
                            .send((name.into(), MidiMappingEvent::EncoderDecrement))
                            .ok();
                    }
                }
            }
        }
    }

    pub fn note_on(&mut self, channel: u8, number: u8, velocity: u8) {
        if let Some(names) = self
            .variable_name_mapping
            .note_mapping
            .get(&(channel, number))
        {
            for name in names {
                let variable = self
                    .variables
                    .notes
                    .get_mut(name)
                    .expect("Unexpected: could not find midi variable");

                variable.on(velocity);

                if let Some(event_sender) = self.event_sender.as_ref() {
                    event_sender
                        .send((name.into(), MidiMappingEvent::NoteOn(variable.velocity())))
                        .ok();
                }
            }
        }
    }

    pub fn note_off(&mut self, channel: u8, number: u8, velocity: u8) {
        if let Some(names) = self
            .variable_name_mapping
            .note_mapping
            .get(&(channel, number))
        {
            for name in names {
                let variable = self
                    .variables
                    .notes
                    .get_mut(name)
                    .expect("Unexpected: could not find midi variable");

                variable.off(velocity);

                if let Some(event_sender) = self.event_sender.as_ref() {
                    event_sender
                        .send((name.into(), MidiMappingEvent::NoteOff(variable.velocity())))
                        .ok();
                }
            }
        }
    }

    pub fn variables(&self) -> &MidiVariables {
        &self.variables
    }

    pub fn after_sketch_update(&mut self) {
        for encoder in self.variables.encoders.values_mut() {
            encoder.after_sketch_update();
        }

        for note in self.variables.notes.values_mut() {
            note.after_sketch_update();
        }
    }

    pub fn subscribe(&mut self) -> broadcast::Receiver<(String, MidiMappingEvent)> {
        if let Some(event_sender) = &self.event_sender {
            return event_sender.subscribe();
        }

        let (event_sender, event_receiver) = broadcast::channel(1024);

        self.event_sender = Some(event_sender);

        event_receiver
    }
}

pub(crate) struct MidiVariableNameMapping {
    control_mapping: HashMap<(u8, u8), Vec<String>>,
    encoder_mapping: HashMap<(u8, u8), Vec<String>>,
    note_mapping: HashMap<(u8, u8), Vec<String>>,
}

#[derive(Clone)]
pub(crate) struct MidiVariables {
    controls: HashMap<String, MidiControl>,
    encoders: HashMap<String, MidiEncoder>,
    notes: HashMap<String, MidiNote>,
}

#[derive(Clone)]
pub enum MidiMappingEvent {
    ControllerChanged(f32),
    EncoderIncrement,
    EncoderDecrement,
    NoteOn(f32),
    NoteOff(f32),
}

impl MidiVariables {
    fn get_control(&self, name: &str) -> Result<&MidiControl> {
        self.controls
            .get(name)
            .ok_or_else(|| anyhow!("MIDI control variable {} could not be found", name))
    }

    fn get_encoder(&self, name: &str) -> Result<&MidiEncoder> {
        self.encoders
            .get(name)
            .ok_or_else(|| anyhow!("MIDI encoder variable {} could not be found", name))
    }

    fn get_note(&self, name: &str) -> Result<&MidiNote> {
        self.notes
            .get(name)
            .ok_or_else(|| anyhow!("MIDI note variable {} could not be found", name))
    }

    pub fn control_value(&self, name: &str) -> Result<f32> {
        self.get_control(name).map(|control| control.value())
    }

    pub fn is_encoder_increment(&self, name: &str) -> Result<bool> {
        self.get_encoder(name).map(|encoder| encoder.is_increment())
    }

    pub fn is_encoder_decrement(&self, name: &str) -> Result<bool> {
        self.get_encoder(name).map(|encoder| encoder.is_decrement())
    }

    pub fn is_note_on(&self, name: &str) -> Result<bool> {
        self.get_note(name).map(|note| note.is_on())
    }

    pub fn is_note_off(&self, name: &str) -> Result<bool> {
        self.get_note(name).map(|note| note.is_off())
    }

    pub fn is_note_down(&self, name: &str) -> Result<bool> {
        self.get_note(name).map(|note| note.is_down())
    }

    pub fn note_velocity(&self, name: &str) -> Result<f32> {
        self.get_note(name).map(|note| note.velocity())
    }
}

impl From<MidiMappingConfig> for MidiMapping {
    fn from(mapping_config: MidiMappingConfig) -> Self {
        let mut variable_name_mapping = MidiVariableNameMapping {
            control_mapping: Default::default(),
            encoder_mapping: Default::default(),
            note_mapping: Default::default(),
        };

        let mut variables = MidiVariables {
            controls: Default::default(),
            encoders: Default::default(),
            notes: Default::default(),
        };

        for (name, variable_config) in mapping_config {
            match variable_config {
                MidiVariableConfig::Control {
                    channel,
                    number,
                    default_value,
                } => {
                    variable_name_mapping
                        .control_mapping
                        .entry((channel, number))
                        .or_default()
                        .push(name.clone());

                    variables
                        .controls
                        .insert(name, MidiControl::new(default_value));
                }

                MidiVariableConfig::Encoder {
                    channel,
                    number,
                    mode,
                } => {
                    variable_name_mapping
                        .encoder_mapping
                        .entry((channel, number))
                        .or_default()
                        .push(name.clone());

                    variables.encoders.insert(name, MidiEncoder::new(mode));
                }

                MidiVariableConfig::Note { channel, number } => {
                    variable_name_mapping
                        .note_mapping
                        .entry((channel, number))
                        .or_default()
                        .push(name.clone());

                    variables.notes.insert(name, MidiNote::new());
                }
            }
        }

        Self {
            event_sender: None,
            variable_name_mapping,
            variables,
        }
    }
}
