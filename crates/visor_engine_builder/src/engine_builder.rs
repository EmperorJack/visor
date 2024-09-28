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
}

impl Default for EngineBuilder {
    fn default() -> Self {
        Self {
            runtime: None,
            window_creator: None,
            plugins: EngineBuilder::default_plugins(),
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

    pub fn build(self) -> Engine {
        Engine::new(self.runtime, self.window_creator, self.plugins)
    }
}
