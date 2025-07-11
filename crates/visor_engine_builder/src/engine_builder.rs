use std::path::PathBuf;

use tokio::runtime::Handle;
use visor_engine::{engine::Engine, plugin::Plugin};
use visor_plugin_draw::DrawPlugin;
use visor_plugin_log::LogPlugin;
use visor_plugin_math::MathPlugin;
use visor_plugin_midi::MidiPlugin;
use visor_plugin_state::StatePlugin;
use visor_plugin_time::TimePlugin;

pub struct EngineBuilder {
    runtime_handle: Option<Handle>,
    plugins: Vec<Box<dyn Plugin>>,
    linked_plugin_paths: Vec<PathBuf>,
}

impl Default for EngineBuilder {
    fn default() -> Self {
        Self {
            runtime_handle: None,
            plugins: EngineBuilder::default_plugins(),
            linked_plugin_paths: Default::default(),
        }
    }
}

impl EngineBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn default_plugins() -> Vec<Box<dyn Plugin>> {
        vec![
            Box::new(TimePlugin),
            Box::new(LogPlugin),
            Box::new(MathPlugin),
            Box::new(DrawPlugin),
            Box::new(StatePlugin),
            Box::new(MidiPlugin),
        ]
    }

    pub fn with_runtime_handle(mut self, runtime_handle: Handle) -> Self {
        self.runtime_handle = Some(runtime_handle);
        self
    }

    pub fn with_plugins(mut self, plugins: Vec<Box<dyn Plugin>>) -> Self {
        self.plugins = plugins;
        self
    }

    pub fn extend_plugins(mut self, plugins: Vec<Box<dyn Plugin>>) -> Self {
        self.plugins.extend(plugins);
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
