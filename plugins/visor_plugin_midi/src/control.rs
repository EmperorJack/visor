pub(crate) struct MidiControl {
    value: f32,
}

impl MidiControl {
    pub fn new(default_value: Option<u8>) -> Self {
        Self {
            value: default_value
                .map(|value| value as f32 / 127.0)
                .unwrap_or(0.0),
        }
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn changed(&mut self, value: u8) {
        self.value = value as f32 / 127.0
    }
}
