use crate::stats::Stats;

pub struct Engine {
    stats: Stats,
}

impl Engine {
    pub fn run() {
        let mut engine = Self::new();

        println!("Engine running!");

        loop {
            engine.update();

            // TODO: adjust sleep duration dynamically or find a way to integrate into an event loop e.g: tao, winit
            std::thread::sleep(std::time::Duration::from_millis(2));
        }
    }

    pub fn new() -> Self {
        Self {
            stats: Stats::new(),
        }
    }

    pub fn update(&mut self) {
        self.stats.before_update();

        // TODO: implement update body

        self.stats.after_update();
    }
}
