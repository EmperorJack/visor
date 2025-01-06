pub(crate) struct MidiNote {
    is_on: bool,
    is_off: bool,
    is_down: bool,
    velocity: f32,
}

impl MidiNote {
    pub fn new() -> Self {
        Self {
            is_on: false,
            is_off: false,
            is_down: false,
            velocity: 0.0,
        }
    }

    pub fn is_on(&self) -> bool {
        self.is_on
    }

    pub fn is_off(&self) -> bool {
        self.is_off
    }

    pub fn is_down(&self) -> bool {
        self.is_down
    }

    pub fn velocity(&self) -> f32 {
        self.velocity
    }

    pub fn on(&mut self, velocity: u8) {
        self.is_on = true;
        self.is_down = true;
        self.velocity = velocity as f32 / 127.0;
    }

    pub fn off(&mut self, velocity: u8) {
        self.is_off = true;
        self.is_down = false;
        self.velocity = velocity as f32 / 127.0;
    }

    pub fn after_sketch_update(&mut self) {
        self.is_on = false;
        self.is_off = false;
    }
}
