use deno_core::anyhow::{anyhow, Result};
use std::collections::HashMap;

use crate::{
    config::{MidiMappingConfig, MidiVariableConfig},
    control::MidiControl,
    encoder::MidiEncoder,
    note::MidiNote,
};

pub(crate) struct MidiMapping {
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
                self.variables
                    .controls
                    .get_mut(name)
                    .expect("Unexpected: could not find midi variable")
                    .changed(value);
            }
        }

        if let Some(names) = self
            .variable_name_mapping
            .encoder_mapping
            .get(&(channel, number))
        {
            for name in names {
                self.variables
                    .encoders
                    .get_mut(name)
                    .expect("Unexpected: could not find midi variable")
                    .changed(value);
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
                self.variables
                    .notes
                    .get_mut(name)
                    .expect("Unexpected: could not find midi variable")
                    .on(velocity);
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
                self.variables
                    .notes
                    .get_mut(name)
                    .expect("Unexpected: could not find midi variable")
                    .off(velocity);
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
            variable_name_mapping,
            variables,
        }
    }
}
