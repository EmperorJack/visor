use std::{path::Path, rc::Rc};

use anyhow::Error;
use deno_core::{error::AnyError, Extension, JsRuntime, ModuleId};

use crate::ts_module_loader::TsModuleLoader;

pub struct Runtime {
    js_runtime: JsRuntime,
    main_module_id: ModuleId,
}

impl Runtime {
    pub async fn compile(path: &Path) -> Result<(Self, Option<Error>), AnyError> {
        let js_extension = Extension {
            name: "sketch",
            ..Default::default()
        };

        let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
            module_loader: Some(Rc::new(TsModuleLoader)),
            extensions: vec![js_extension],
            ..Default::default()
        });

        let path = path
            .to_str()
            .expect("Unexpected: could not convert file path into string");
        let current_dir =
            std::env::current_dir().expect("Unexpected: could not get current working directory");

        let main_module = deno_core::resolve_path(path, &current_dir)?;

        let main_module_id = js_runtime.load_main_es_module(&main_module).await?;

        let result_receiver = js_runtime.mod_evaluate(main_module_id);

        let run_event_loop_result = js_runtime
            .run_event_loop(deno_core::PollEventLoopOptions::default())
            .await;

        let runtime = Self {
            js_runtime,
            main_module_id,
        };

        if let Err(error) = run_event_loop_result {
            return Ok((runtime, Some(error)));
        }

        if let Err(error) = result_receiver.await {
            return Ok((runtime, Some(error)));
        }

        Ok((runtime, None))
    }
}
