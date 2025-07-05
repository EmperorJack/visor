use deno_core::{Extension, OpState, extension, op2};
use visor_engine::{
    engine::Engine,
    plugin::{AccessSketchStore, Plugin},
    sketch::SketchId,
    sketch_store::SketchStore,
    store::Store,
};

pub struct LogPlugin;

type SketchState = Vec<LogEntry>;

#[derive(Clone)]
pub struct LogEntry {
    pub message: String,
    pub message_type: LogEntryType,
}

#[derive(Clone)]
pub enum LogEntryType {
    Stdout,
    Stderr,
}

extension!(
    visor_plugin_log,
    ops = [op_log_console_log, op_log_console_error],
    esm_entry_point = "ext:visor_plugin_log/src/visor-plugin-log.ts",
    esm = ["src/visor-plugin-log.ts"]
);

impl LogPlugin {
    pub fn get_state(sketch_store: &SketchStore) -> &SketchState {
        sketch_store.get()
    }
}

impl Plugin for LogPlugin {
    fn extension(&self) -> Extension {
        visor_plugin_log::init()
    }

    fn typescript_declaration(&self) -> Option<String> {
        Some(include_str!("visor-plugin-log.d.ts").into())
    }

    fn build_sketch(
        &self,
        _sketch_id: &SketchId,
        _engine: &mut Engine,
        _store: &Store,
        sketch_store: &mut SketchStore,
    ) {
        sketch_store.set(SketchState::default());
    }

    fn before_sketch_update(
        &self,
        _sketch_id: &SketchId,
        _store: &Store,
        sketch_store: &mut SketchStore,
    ) {
        sketch_store.get_mut::<SketchState>().clear();
    }
}

#[op2(fast)]
fn op_log_console_log(state: &mut OpState, #[string] message: String) {
    let logs = state.sketch_store_mut().get_mut::<SketchState>();

    logs.push(LogEntry {
        message,
        message_type: LogEntryType::Stdout,
    });
}

#[op2(fast)]
fn op_log_console_error(state: &mut OpState, #[string] message: String) {
    let logs = state.sketch_store_mut().get_mut::<SketchState>();

    logs.push(LogEntry {
        message,
        message_type: LogEntryType::Stderr,
    });
}
