use std::sync::RwLock;

use deno_core::{Extension, OpState, extension, op2};
use visor_engine::{
    engine::Engine,
    plugin::{AccessSketchStore, Plugin, plugin_implementation},
    sketch::SketchId,
    sketch_store::SketchStore,
    store::Store,
};

pub struct CounterPlugin;

// Note: this line is only required for linked plugins
plugin_implementation!(Plugin, CounterPlugin);

#[derive(Clone)]
struct State {
    count: u32,
}

extension!(
    visor_plugin_counter,
    ops = [op_counter_count, op_counter_increment],
    esm_entry_point = "ext:visor_plugin_counter/src/visor-plugin-counter.js",
    esm = ["src/visor-plugin-counter.js"]
);

impl Plugin for CounterPlugin {
    fn extension(&self) -> Extension {
        visor_plugin_counter::init()
    }

    fn build(&self, _engine: &mut Engine, store: &Store) {
        let state = State { count: 0 };

        store.set(RwLock::new(state));
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

        // TODO: this is not a global counter as intended, need to fix
        sketch_store.set(state.clone());
    }
}

#[op2(fast)]
fn op_counter_count(state: &mut OpState) -> u32 {
    let state = state.sketch_store().get::<State>();

    state.count
}

#[op2(fast)]
fn op_counter_increment(state: &mut OpState) {
    let state = state.sketch_store_mut().get_mut::<State>();

    state.count += 1;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use visor_core::{engine::Engine, engine_builder::EngineBuilder};

    use crate::CounterPlugin;

    fn verify_plugin(mut engine: Engine) {
        let sketch_path =
            PathBuf::from(format!("{}/example-sketch.js", env!("CARGO_MANIFEST_DIR")));

        let sketch_id = *engine.create_sketch(sketch_path).id();

        engine.update();

        let sketch = engine
            .sketches()
            .get(&sketch_id)
            .expect("Unexpected: could not find sketch");

        assert_eq!(*sketch.compile_error(), None);
        assert_eq!(*sketch.runtime_error(), None);

        let sketch_store = engine
            .sketch_stores()
            .get(&sketch_id)
            .expect("Unexpected: could not find sketch store");

        let sketch_logs = visor_plugin_log::LogPlugin::get_state(sketch_store);

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
