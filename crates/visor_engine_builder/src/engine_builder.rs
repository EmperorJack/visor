use std::path::PathBuf;

use tokio::runtime::Runtime;
use visor_engine::{
    engine::{Engine, WindowCreator},
    plugin::Plugin,
};
use visor_plugin_draw::DrawPlugin;
use visor_plugin_log::LogPlugin;
use visor_plugin_time::TimePlugin;

pub struct EngineBuilder {
    runtime: Option<Runtime>,
    window_creator: Option<Box<dyn WindowCreator>>,
    plugins: Vec<Box<dyn Plugin>>,
    linked_plugin_paths: Vec<PathBuf>,
}

impl Default for EngineBuilder {
    fn default() -> Self {
        Self {
            runtime: None,
            window_creator: None,
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
            Box::new(DrawPlugin),
        ]
    }

    pub fn with_runtime(mut self, runtime: Runtime) -> Self {
        self.runtime = Some(runtime);
        self
    }

    pub fn with_window_creator(mut self, callback: Box<dyn WindowCreator>) -> Self {
        self.window_creator = Some(callback);
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
        Engine::new(
            self.runtime,
            self.window_creator,
            self.plugins,
            self.linked_plugin_paths,
        )
    }
}
