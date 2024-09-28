use std::{collections::HashMap, sync::RwLock};

use deno_core::{extension, op2, Extension, OpState};
use visor_engine::{engine::Engine, plugin::Plugin, store::Store, Runtime};

pub struct LogPlugin;

pub type State = HashMap<String, Logs>;
pub type Logs = Vec<LogEntry>;

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

impl Plugin for LogPlugin {
    fn extension(&self) -> Extension {
        extension::init_ops_and_esm()
    }

    fn build(&self, _engine: &mut Engine, store: &Store) {
        store.set(RwLock::new(State::default()));
    }

    fn before_sketch_update(&self, _sketch_id: &str, runtime: &mut Runtime, _store: &Store) {
        runtime.put_state(Logs::default());
    }

    fn after_sketch_update(&self, sketch_id: &str, runtime: &mut Runtime, store: &Store) {
        let logs: Logs = runtime.take_state();

        if !logs.is_empty() {
            let mut state = store
                .get::<RwLock<State>>()
                .write()
                .expect("Unexpected: could not acquire read lock for state");

            let logs_entry = state.entry(sketch_id.into()).or_default();
            logs_entry.extend(logs);
        }
    }
}

#[op2(fast)]
fn op_log_console_log(state: &mut OpState, #[string] message: String) {
    let logs = state.borrow_mut::<Logs>();

    logs.push(LogEntry {
        message,
        message_type: LogEntryType::Stdout,
    });
}

#[op2(fast)]
fn op_log_console_error(state: &mut OpState, #[string] message: String) {
    let logs = state.borrow_mut::<Logs>();

    logs.push(LogEntry {
        message,
        message_type: LogEntryType::Stderr,
    });
}
