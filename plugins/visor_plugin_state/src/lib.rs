use std::collections::HashMap;

use deno_core::{Extension, OpState, extension, op2};
use visor_engine::{
    engine::Engine,
    plugin::{AccessSketchStore, Plugin},
    sketch::SketchId,
    sketch_store::SketchStore,
    store::Store,
};

pub struct StatePlugin;

type SketchState = HashMap<String, String>;

extension!(
    visor_plugin_state,
    ops = [op_state_create, op_state_set, op_state_remove_unused],
    esm_entry_point = "ext:visor_plugin_state/src/visor-plugin-state.js",
    esm = ["src/visor-plugin-state.js"]
);

impl Plugin for StatePlugin {
    fn extension(&self) -> Extension {
        visor_plugin_state::init()
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
}

#[op2]
#[string]
fn op_state_create(state: &mut OpState, #[string] id: String, #[string] value: String) -> String {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.entry(id).or_insert(value).to_string()
}

#[op2(fast)]
fn op_state_set(state: &mut OpState, #[string] id: String, #[string] value: String) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.insert(id, value);
}

#[op2]
fn op_state_remove_unused(state: &mut OpState, #[serde] ids: Vec<String>) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.retain(|id, _| ids.contains(id));
}
