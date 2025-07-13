use std::collections::HashMap;

pub(crate) type MidiMappingConfig = HashMap<String, MidiVariableConfig>;

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub(crate) enum MidiVariableConfig {
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
pub(crate) enum MidiEncoderMode {
    #[serde(rename = "7Fh/01h")]
    Mode7fh01h,
    #[serde(rename = "3Fh/41h")]
    Mode3fh41h,
}
