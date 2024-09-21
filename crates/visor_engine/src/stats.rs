use std::time::{Duration, Instant};

pub(crate) struct Stats {
    pub frame_count: u32,
    pub frame_rate: f32,
    pub seconds_elapsed: f32,
    time_started: Instant,
    time_last_updated: Instant,
    since_last_update: Duration,
}

impl Stats {
    pub fn new() -> Self {
        let time_started = Instant::now();

        Self {
            frame_count: 0,
            frame_rate: 0.0,
            seconds_elapsed: 0.0,
            time_started,
            time_last_updated: time_started,
            since_last_update: Duration::default(),
        }
    }

    pub fn before_update(&mut self) {
        let time_now = Instant::now();

        self.since_last_update = time_now.duration_since(self.time_last_updated);
        self.time_last_updated = time_now;

        self.seconds_elapsed = Self::calculate_seconds_elapsed(self.time_started, time_now);

        self.frame_rate = Self::calculate_frame_rate(self.since_last_update);
    }

    pub fn after_update(&mut self) {
        self.frame_count += 1;
    }

    fn calculate_seconds_elapsed(time_started: Instant, time_now: Instant) -> f32 {
        let since_start = time_now.duration_since(time_started);

        (since_start.as_secs() as f64 + since_start.subsec_nanos() as f64 * 1e-9) as f32
    }

    fn calculate_frame_rate(since_last_update: Duration) -> f32 {
        if since_last_update.as_secs() > 0 {
            return 0.0;
        }

        let millis = since_last_update.subsec_millis() as f32;
        if millis == 0.0 {
            return std::f32::MAX;
        }

        1000.0 / millis
    }
}
