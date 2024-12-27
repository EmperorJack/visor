use std::collections::HashMap;

use deno_core::{extension, op2, Extension, OpState};
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
    extension,
    ops = [op_state_create, op_state_set, op_state_remove_unused],
    esm_entry_point = "visor:state",
    esm = [
        dir "src",
        "visor:state" = "visor-state.js",
    ]
);

impl Plugin for StatePlugin {
    fn extension(&self) -> Extension {
        extension::init_ops_and_esm()
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
