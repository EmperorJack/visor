use std::ops::Deref;

use visor_runtime::{runtime::Runtime, Extension};

use crate::{engine::Engine, sketch::SketchId, store::Store};

pub trait Plugin: Send + Sync {
    fn extension(&self) -> Extension;

    fn build(&self, _engine: &mut Engine, _state: &Store) {}

    fn engine_update(&self, _engine: &mut Engine, _state: &Store) {}

    fn before_sketch_update(&self, _sketch_id: &SketchId, _runtime: &mut Runtime, _state: &Store) {}
    fn after_sketch_update(&self, _sketch_id: &SketchId, _runtime: &mut Runtime, _state: &Store) {}
}

// TODO: re-export pluginator so easier for consuming crates
pluginator::plugin_trait!(Plugin);

pub(crate) enum LoadedPlugin {
    Compiled(Box<dyn Plugin>),
    Linked(pluginator::LoadedPlugin<dyn Plugin>),
}

impl Deref for LoadedPlugin {
    type Target = dyn Plugin;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Compiled(plugin) => plugin.as_ref(),
            Self::Linked(loaded_plugin) => loaded_plugin.deref(),
        }
    }
}
