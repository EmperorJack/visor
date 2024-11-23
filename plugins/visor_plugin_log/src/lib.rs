use std::{collections::HashMap, sync::RwLock};

use deno_core::{extension, op2, Extension, OpState};
use visor_engine::{engine::Engine, plugin::Plugin, sketch::SketchId, store::Store, Runtime};

pub struct LogPlugin;

pub type State = HashMap<SketchId, Vec<LogEntry>>;

pub struct LogEntry {
    pub message: String,
    pub message_type: LogEntryType,
}

pub enum LogEntryType {
    Stdout,
    Stderr,
}

extension!(
    extension,
    ops = [op_log_console_log, op_log_console_error],
    esm_entry_point = "visor:log",
    esm = [
        dir "src",
        "visor:log" = "visor-log.js",
    ]
);

impl LogPlugin {
    pub fn get_state(store: &Store) -> &RwLock<State> {
        store.get::<RwLock<State>>()
    }
}

impl Plugin for LogPlugin {
    fn extension(&self) -> Extension {
        extension::init_ops_and_esm()
    }

    fn build(&self, _engine: &mut Engine, store: &Store) {
        store.set(RwLock::new(State::default()));
    }

    fn before_sketch_update(&self, _sketch_id: &SketchId, runtime: &mut Runtime, _store: &Store) {
        runtime.put_state(Vec::<LogEntry>::default());
    }

    fn after_sketch_update(&self, sketch_id: &SketchId, runtime: &mut Runtime, store: &Store) {
        let logs: Vec<LogEntry> = runtime.take_state();

        // TODO: consider supporting a persistent per-sketch store to reduce possible lock contention
        let mut state = store
            .get::<RwLock<State>>()
            .write()
            .expect("Unexpected: could not acquire write lock for state");

        state.insert(*sketch_id, logs);
    }
}

#[op2(fast)]
fn op_log_console_log(state: &mut OpState, #[string] message: String) {
    let logs = state.borrow_mut::<Vec<LogEntry>>();

    logs.push(LogEntry {
        message,
        message_type: LogEntryType::Stdout,
    });
}

#[op2(fast)]
fn op_log_console_error(state: &mut OpState, #[string] message: String) {
    let logs = state.borrow_mut::<Vec<LogEntry>>();

    logs.push(LogEntry {
        message,
        message_type: LogEntryType::Stderr,
    });
}
