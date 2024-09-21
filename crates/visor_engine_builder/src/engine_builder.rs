use tokio::runtime::Runtime;
use visor_engine::{
    engine::{Engine, WindowCreator},
    plugin::Plugin,
};
use visor_plugin_draw::DrawPlugin;

#[derive(Default)]
pub struct EngineBuilder {
    runtime: Option<Runtime>,
    window_creator: Option<Box<dyn WindowCreator>>,
}

impl EngineBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_runtime(mut self, runtime: Runtime) -> Self {
        self.runtime = Some(runtime);
        self
    }

    pub fn with_window_creator(mut self, callback: Box<dyn WindowCreator>) -> Self {
        self.window_creator = Some(callback);
        self
    }

    pub fn build(self) -> Engine {
        let plugins: Vec<Box<dyn Plugin>> = vec![Box::new(DrawPlugin)];

        Engine::new(self.runtime, self.window_creator, plugins)
    }
}
