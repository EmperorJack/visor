use std::collections::HashMap;

pub type MidiMappingConfig = HashMap<String, MidiVariableConfig>;

#[derive(Clone, serde::Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum MidiVariableConfig {
    Control {
        channel: u8,
        number: u8,
        default_value: Option<u8>,
    },
    Encoder {
        channel: u8,
        number: u8,
        mode: MidiEncoderMode,
    },
    Note {
        channel: u8,
        number: u8,
    },
}

#[derive(Clone, serde::Deserialize)]
pub enum MidiEncoderMode {
    #[serde(rename = "7Fh/01h")]
    Mode7fh01h,
    #[serde(rename = "3Fh/41h")]
    Mode3fh41h,
}
