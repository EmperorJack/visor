use std::{collections::HashMap, sync::RwLock};

use deno_core::{extension, op2, Extension, OpState};
use visor_engine::{engine::Engine, plugin::Plugin, sketch::SketchId, store::Store, Runtime};

pub struct StatePlugin;

type State = HashMap<SketchId, SketchState>;

type SketchState = HashMap<String, String>;

extension!(
    extension,
    ops = [op_state_create, op_state_get, op_state_set, op_state_remove_unused],
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

    fn build(&self, _engine: &mut Engine, store: &Store) {
        store.set(RwLock::new(State::default()));
    }

    fn before_sketch_update(&self, sketch_id: &SketchId, runtime: &mut Runtime, store: &Store) {
        let mut state = store
            .get::<RwLock<State>>()
            .write()
            .expect("Unexpected: could not acquire write lock for state");

        let sketch_state = state.remove(sketch_id).unwrap_or_default();

        runtime.put_state(sketch_state);
    }

    fn after_sketch_update(&self, sketch_id: &SketchId, runtime: &mut Runtime, store: &Store) {
        let mut state = store
            .get::<RwLock<State>>()
            .write()
            .expect("Unexpected: could not acquire write lock for state");

        let sketch_state: SketchState = runtime.take_state();

        state.insert(*sketch_id, sketch_state);
    }
}

#[op2(fast)]
fn op_state_create(state: &mut OpState, #[string] id: String, #[string] value: String) {
    let state = state.borrow_mut::<SketchState>();

    if !state.contains_key(&id) {
        state.insert(id, value);
    }
}

#[op2]
#[string]
fn op_state_get<'a>(state: &mut OpState, #[string] id: String) -> String {
    let state = state.borrow_mut::<SketchState>();

    state
        .get(&id)
        .expect(&format!(
            "Unexpected: could not find sketch state variable with id {:?}",
            id
        ))
        .to_string()
}

#[op2(fast)]
fn op_state_set(state: &mut OpState, #[string] id: String, #[string] value: String) {
    let state = state.borrow_mut::<SketchState>();

    state.insert(id, value);
}

#[op2]
fn op_state_remove_unused(state: &mut OpState, #[serde] ids: Vec<String>) {
    let state = state.borrow_mut::<SketchState>();

    state.retain(|id, _| ids.contains(id));
}
