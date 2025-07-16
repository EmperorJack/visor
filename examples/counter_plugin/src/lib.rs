mod counter_plugin;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use visor_core::{Engine, EngineBuilder, SketchBuilder, core_plugins};

    use crate::counter_plugin::CounterPlugin;

    fn verify_plugin(mut engine: Engine) {
        let sketch_path =
            PathBuf::from(format!("{}/example-sketch.js", env!("CARGO_MANIFEST_DIR")));

        let sketch_id = *SketchBuilder::new(sketch_path).build(&mut engine).id();

        engine.update();

        let sketch = engine
            .sketches()
            .get(&sketch_id)
            .expect("Unexpected: could not find sketch");

        assert_eq!(*sketch.compile_error(), None);
        assert_eq!(*sketch.runtime_error(), None);

        let sketch_store = engine
            .sketch_stores()
            .get(&sketch_id)
            .expect("Unexpected: could not find sketch store");

        let sketch_logs = visor_plugin_log::LogPlugin::get_state(sketch_store);

        assert_eq!(sketch_logs[0].message, "0");
        assert_eq!(sketch_logs[1].message, "1");
        assert_eq!(sketch_logs[2].message, "2");
    }

    #[test]
    fn as_compiled_plugin() {
        let mut plugins = core_plugins();
        plugins.push(Box::new(CounterPlugin));

        let engine = EngineBuilder::default().with_plugins(plugins).build();

        verify_plugin(engine);
    }

    #[test]
    fn as_linked_plugin() {
        let plugin_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("target/debug/libcounter_plugin.dylib");

        let engine = EngineBuilder::default()
            .with_plugins(core_plugins())
            .with_linked_plugins(vec![plugin_path])
            .build();

        verify_plugin(engine);
    }
}
