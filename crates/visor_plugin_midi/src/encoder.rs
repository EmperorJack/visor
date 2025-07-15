use crate::config::MidiEncoderMode;

#[derive(Clone)]
pub(crate) struct MidiEncoder {
    mode: MidiEncoderMode,
    is_increment: bool,
    is_decrement: bool,
}

impl MidiEncoder {
    pub fn new(mode: MidiEncoderMode) -> Self {
        Self {
            mode,
            is_increment: false,
            is_decrement: false,
        }
    }

    pub fn is_increment(&self) -> bool {
        self.is_increment
    }

    pub fn is_decrement(&self) -> bool {
        self.is_decrement
    }

    pub fn changed(&mut self, value: u8) {
        match self.mode {
            MidiEncoderMode::Mode7fh01h => {
                if value <= 64 {
                    self.is_increment = true;
                } else if (65..=127).contains(&value) {
                    self.is_decrement = true;
                }
            }

            MidiEncoderMode::Mode3fh41h => {
                if (65..=127).contains(&value) {
                    self.is_increment = true;
                } else if value <= 63 {
                    self.is_decrement = true
                }
            }
        }
    }

    pub fn after_sketch_update(&mut self) {
        self.is_increment = false;
        self.is_decrement = false;
    }
}
