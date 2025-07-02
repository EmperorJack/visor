use std::{
    sync::RwLock,
    time::{Duration, Instant},
};

use deno_core::{Extension, OpState, extension, op2};
use visor_engine::{
    engine::Engine,
    plugin::{AccessSketchStore, Plugin},
    sketch::SketchId,
    sketch_store::SketchStore,
    store::Store,
};

pub struct TimePlugin;

pub struct State {
    pub frame_count: u32,
    pub frame_rate: f32,
    pub seconds_elapsed: f32,
    time_started: Instant,
    time_last_updated: Instant,
    since_last_update: Duration,
    target_frame_duration: Duration,
    delta: f32,
}

struct SketchState {
    time: f32,
    frame_count: u32,
    delta: f32,
}

const TARGET_FRAME_RATE: u32 = 60;

extension!(
    visor_plugin_time,
    ops = [op_time_frame_count, op_time_time, op_time_delta],
    esm_entry_point = "ext:visor_plugin_time/src/visor-plugin-time.js",
    esm = ["src/visor-plugin-time.js"]
);

impl TimePlugin {
    pub fn get_state(store: &Store) -> &RwLock<State> {
        store.get::<RwLock<State>>()
    }
}

impl Plugin for TimePlugin {
    fn extension(&self) -> Extension {
        visor_plugin_time::init()
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
            target_frame_duration: Duration::from_secs_f64(1.0 / TARGET_FRAME_RATE as f64),
            delta: 0.0,
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

        state.delta =
            state.since_last_update.as_secs_f32() / state.target_frame_duration.as_secs_f32();
    }

    fn before_sketch_update(
        &self,
        _sketch_id: &SketchId,
        store: &Store,
        sketch_store: &mut SketchStore,
    ) {
        let state = store
            .get::<RwLock<State>>()
            .write()
            .expect("Unexpected: could not acquire read lock for state");

        let sketch_state = SketchState {
            time: state.seconds_elapsed,
            frame_count: state.frame_count,
            delta: state.delta,
        };

        sketch_store.set(sketch_state);
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
    let sketch_state = state.sketch_store().get::<SketchState>();

    sketch_state.frame_count
}

#[op2(fast)]
fn op_time_time(state: &mut OpState) -> f32 {
    let sketch_state = state.sketch_store().get::<SketchState>();

    sketch_state.time
}

#[op2(fast)]
fn op_time_delta(state: &mut OpState) -> f32 {
    let sketch_state = state.sketch_store().get::<SketchState>();

    sketch_state.delta
}
