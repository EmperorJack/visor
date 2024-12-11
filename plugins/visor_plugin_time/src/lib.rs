use std::{
    sync::RwLock,
    time::{Duration, Instant},
};

use deno_core::{extension, op2, Extension, OpState};
use visor_engine::{engine::Engine, plugin::Plugin, sketch::SketchId, store::Store, Runtime};

pub struct TimePlugin;

pub struct State {
    pub frame_count: u32,
    pub frame_rate: f32,
    pub seconds_elapsed: f32,
    time_started: Instant,
    time_last_updated: Instant,
    pub since_last_update: Duration,
}

struct SketchState {
    time: f32,
    frame_count: u32,
}

extension!(
    extension,
    ops = [op_time_frame_count, op_time_time],
    esm_entry_point = "visor:time",
    esm = [
        dir "src",
        "visor:time" = "visor-time.js",
    ]
);

impl TimePlugin {
    pub fn get_state(store: &Store) -> &RwLock<State> {
        store.get::<RwLock<State>>()
    }
}

impl Plugin for TimePlugin {
    fn extension(&self) -> Extension {
        extension::init_ops_and_esm()
    }

    fn build(&self, _engine: &mut Engine, store: &Store) {
        let time_started = Instant::now();

        let state = State {
            frame_count: 0,
            frame_rate: 0.0,
            seconds_elapsed: 0.0,
            time_started,
            time_last_updated: time_started,
            since_last_update: Duration::default(),
        };

        store.set(RwLock::new(state));
    }

    fn before_engine_update(&self, _engine: &mut Engine, store: &Store) {
        let mut state = store
            .get::<RwLock<State>>()
            .write()
            .expect("Unexpected: could not acquire write lock for state");

        let time_now = Instant::now();

        state.since_last_update = time_now.duration_since(state.time_last_updated);
        state.time_last_updated = time_now;

        state.seconds_elapsed = calculate_seconds_elapsed(state.time_started, time_now);

        state.frame_rate = calculate_frame_rate(state.since_last_update);
    }

    fn before_sketch_update(&self, _sketch_id: &SketchId, runtime: &mut Runtime, store: &Store) {
        let state = store
            .get::<RwLock<State>>()
            .read()
            .expect("Unexpected: could not acquire read lock for state");

        let sketch_state = SketchState {
            time: state.seconds_elapsed,
            frame_count: state.frame_count,
        };

        runtime.put_state(sketch_state);
    }

    fn after_engine_update(&self, _engine: &mut Engine, store: &Store) {
        let mut state = store
            .get::<RwLock<State>>()
            .write()
            .expect("Unexpected: could not acquire write lock for state");

        state.frame_count += 1;
    }
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
        return f32::MAX;
    }

    1000.0 / millis
}

#[op2(fast)]
fn op_time_frame_count(state: &mut OpState) -> u32 {
    let state = state.borrow::<SketchState>();

    state.frame_count
}

#[op2(fast)]
fn op_time_time(state: &mut OpState) -> f32 {
    let state = state.borrow::<SketchState>();

    state.time
}
