use std::ops::Deref;

use visor_runtime::{Extension, OpState};

use crate::{engine::Engine, sketch::SketchId, sketch_store::SketchStore, store::Store};

pub trait Plugin: Send + Sync {
    fn extension(&self) -> Extension;

    fn build(&self, _engine: &mut Engine, _store: &Store) {}

    fn build_sketch(
        &self,
        _sketch_id: &SketchId,
        _engine: &mut Engine,
        _store: &Store,
        _sketch_store: &mut SketchStore,
    ) {
    }

    fn before_engine_update(&self, _engine: &mut Engine, _store: &Store) {}

    fn before_sketch_update(
        &self,
        _sketch_id: &SketchId,
        _store: &Store,
        _sketch_store: &mut SketchStore,
    ) {
    }

    fn after_sketch_update(
        &self,
        _sketch_id: &SketchId,
        _store: &Store,
        _sketch_store: &mut SketchStore,
    ) {
    }

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

pub trait AccessSketchStore {
    fn sketch_store(&self) -> &SketchStore;
    fn sketch_store_mut(&mut self) -> &mut SketchStore;
}

impl AccessSketchStore for OpState {
    fn sketch_store(&self) -> &SketchStore {
        self.borrow()
    }

    fn sketch_store_mut(&mut self) -> &mut SketchStore {
        self.borrow_mut()
    }
}
