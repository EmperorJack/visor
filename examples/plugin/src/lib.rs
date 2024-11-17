use std::sync::RwLock;

use deno_core::{extension, op2, Extension, OpState};
use visor_engine::{
    engine::Engine,
    plugin::{plugin_implementation, Plugin},
    sketch::SketchId,
    store::Store,
    Runtime,
};

pub struct CounterPlugin;

// Note: this line is only required for linked plugins
plugin_implementation!(Plugin, CounterPlugin);

struct State {
    count: u32,
}

extension!(
    extension,
    ops = [op_counter_count, op_counter_increment],
    esm_entry_point = "visor:counter",
    esm = [
        dir "src",
        "visor:counter" = "visor-counter.js",
    ]
);

impl Plugin for CounterPlugin {
    fn extension(&self) -> Extension {
        extension::init_ops_and_esm()
    }

    fn build(&self, _engine: &mut Engine, store: &Store) {
        let state = State { count: 0 };

        store.set(RwLock::new(state));
    }

    fn before_sketch_update(&self, _sketch_id: &SketchId, runtime: &mut Runtime, store: &Store) {
        let state = store
            .get::<RwLock<State>>()
            .read()
            .expect("Unexpected: could not acquire read lock for state");

        runtime.put_state(state.count);
    }

    fn after_sketch_update(&self, _sketch_id: &SketchId, runtime: &mut Runtime, store: &Store) {
        let count: u32 = runtime.take_state();

        let mut state = store
            .get::<RwLock<State>>()
            .write()
            .expect("Unexpected: could not acquire write lock for state");

        state.count = count;
    }
}

#[op2(fast)]
fn op_counter_count(state: &mut OpState) -> u32 {
    let count = state.borrow::<u32>();

    *count
}

#[op2(fast)]
fn op_counter_increment(state: &mut OpState) {
    let count = state.borrow_mut::<u32>();

    *count += 1;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use visor::{engine::Engine, engine_builder::EngineBuilder};

    use crate::CounterPlugin;

    fn verify_plugin(mut engine: Engine) {
        let sketch_path =
            PathBuf::from(format!("{}/example-sketch.js", env!("CARGO_MANIFEST_DIR")));

        let sketch_id = engine.create_sketch(sketch_path);

        engine.update();

        let log_state = engine
            .store()
            .get::<visor_plugin_log::State>()
            .read()
            .expect("Unexpected: could not acquire read lock for log plugin state");

        let sketch_logs = log_state
            .get(&sketch_id)
            .expect("Unexpected: sketch logs should be defined");

        assert_eq!(sketch_logs[0].message, "0");
        assert_eq!(sketch_logs[1].message, "1");
        assert_eq!(sketch_logs[2].message, "2");
    }

    #[test]
    fn as_compiled_plugin() {
        let engine = EngineBuilder::default()
            .extend_plugins(vec![Box::new(CounterPlugin)])
            .build();

        verify_plugin(engine);
    }

    #[test]
    fn as_linked_plugin() {
        let plugin_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("target/debug/libplugin.dylib");

        let engine = EngineBuilder::default()
            .with_linked_plugins(vec![plugin_path])
            .build();

        verify_plugin(engine);
    }
}
