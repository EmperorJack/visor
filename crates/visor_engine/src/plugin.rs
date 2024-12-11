use std::ops::Deref;

use visor_runtime::{runtime::Runtime, Extension};

use crate::{engine::Engine, sketch::SketchId, store::Store};

pub trait Plugin: Send + Sync {
    fn extension(&self) -> Extension;

    fn build(&self, _engine: &mut Engine, _store: &Store) {}

    fn before_engine_update(&self, _engine: &mut Engine, _store: &Store) {}

    fn before_sketch_update(&self, _sketch_id: &SketchId, _runtime: &mut Runtime, _store: &Store) {}
    fn after_sketch_update(&self, _sketch_id: &SketchId, _runtime: &mut Runtime, _store: &Store) {}

    fn engine_render(
        &self,
        _engine: &mut Engine,
        _store: &Store,
        _encoder: &mut nannou::wgpu::CommandEncoder,
    ) {
    }

    fn after_engine_update(&self, _engine: &mut Engine, _store: &Store) {}
}

pluginator::plugin_trait!(Plugin);

pub use pluginator::plugin_implementation;

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
