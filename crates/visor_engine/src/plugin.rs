use visor_runtime::{runtime::Runtime, Extension};

use crate::{engine::Engine, store::Store};

pub trait Plugin: Send + Sync {
    fn extension(&self) -> Extension;

    fn build(&self, _engine: &mut Engine, _state: &Store) {}

    fn engine_update(&self, _engine: &mut Engine, _state: &Store) {}

    fn sketch_update(&self, _runtime: &mut Runtime, _state: &Store) {}
}
