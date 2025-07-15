use std::sync::RwLock;

use deno_core::{Extension, OpState, extension, op2};
use visor_engine::{
    AccessSketchStore, Engine, Plugin, SketchId, SketchStore, Store, linked_plugin,
};

pub struct CounterPlugin;

// Note: this line is only required for linked plugins
linked_plugin!(CounterPlugin);

#[derive(Clone)]
struct State {
    count: u32,
}

extension!(
    visor_plugin_counter,
    ops = [op_counter_count, op_counter_increment],
    esm_entry_point = "ext:visor_plugin_counter/src/counter-plugin.js",
    esm = ["src/counter-plugin.js"]
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

        // TODO: this is not a global counter as intended, would be good to demonstrate both
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
