use std::path::PathBuf;

use tokio::runtime::Handle;

use crate::{engine::Engine, plugin::Plugin};

#[derive(Default)]
pub struct EngineBuilder {
    runtime_handle: Option<Handle>,
    plugins: Vec<Box<dyn Plugin>>,
    linked_plugin_paths: Vec<PathBuf>,
}

impl EngineBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_runtime_handle(mut self, runtime_handle: Handle) -> Self {
        self.runtime_handle = Some(runtime_handle);
        self
    }

    pub fn with_plugins(mut self, plugins: Vec<Box<dyn Plugin>>) -> Self {
        self.plugins = plugins;
        self
    }

    pub fn with_linked_plugins(mut self, paths: Vec<PathBuf>) -> Self {
        self.linked_plugin_paths = paths;
        self
    }

    pub fn build(self) -> Engine {
        Engine::new(self.runtime_handle, self.plugins, self.linked_plugin_paths)
    }
}
