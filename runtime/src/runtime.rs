use std::{path::Path, rc::Rc};

use anyhow::Error;
use deno_core::{error::AnyError, v8, Extension, JsRuntime, ModuleId};
use draw::draw::Draw;

use crate::{ops::OPS, ts_module_loader::TsModuleLoader};

pub enum SketchFunction {
    Setup,
    Update,
    Draw,
}

impl SketchFunction {
    fn export_name(&self) -> &str {
        match self {
            SketchFunction::Setup => "setup",
            SketchFunction::Update => "update",
            SketchFunction::Draw => "draw",
        }
    }
}

pub enum RuntimeExecuteFunctionResult {
    Success,
    Missing,
    Error(Error),
}

pub struct Runtime {
    js_runtime: JsRuntime,
    main_module_id: ModuleId,
}

impl Runtime {
    pub async fn compile(
        path: &Path,
        draw: Draw,
    ) -> Result<(Option<Self>, Option<Error>), AnyError> {
        let js_extension = Extension {
            name: "sketch",
            ops: std::borrow::Cow::Borrowed(&OPS),
            op_state_fn: Some(Box::new(|state| {
                state.put(draw);
            })),
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

        let main_module_id = match js_runtime.load_main_es_module(&main_module).await {
            Ok(main_module_id) => main_module_id,
            Err(error) => return Ok((None, Some(error))),
        };

        let result_receiver = js_runtime.mod_evaluate(main_module_id);

        let run_event_loop_result = js_runtime
            .run_event_loop(deno_core::PollEventLoopOptions::default())
            .await;

        let runtime = Self {
            js_runtime,
            main_module_id,
        };

        if let Err(error) = run_event_loop_result {
            return Ok((Some(runtime), Some(error)));
        }

        if let Err(error) = result_receiver.await {
            return Ok((Some(runtime), Some(error)));
        }

        Ok((Some(runtime), None))
    }

    pub async fn execute_runtime_function(
        &mut self,
        sketch_function: SketchFunction,
    ) -> RuntimeExecuteFunctionResult {
        self.execute_sketch_function_inner(&sketch_function)
            .await
            .expect(&format!(
                "Unexpected: could not call {} function on sketch",
                sketch_function.export_name(),
            ))
    }

    async fn execute_sketch_function_inner(
        &mut self,
        sketch_function: &SketchFunction,
    ) -> Result<RuntimeExecuteFunctionResult, AnyError> {
        let function_export_name = sketch_function.export_name();

        let function = {
            let module_namespace = self.js_runtime.get_module_namespace(self.main_module_id)?;

            let mut scope = &mut self.js_runtime.handle_scope();

            let module_namespace = v8::Local::new(&mut scope, module_namespace);

            let function_export_string = v8::String::new(scope, function_export_name)
                .unwrap_or_else(|| {
                    panic!(
                        "Unexpected: could not create v8 string for {function_export_name} export"
                    )
                });

            let function_export = module_namespace
                .get(scope, function_export_string.into())
                .unwrap_or_else(|| {
                    panic!("Unexpected: could not find {function_export_name} export")
                });

            if let Ok(function) = v8::Local::<v8::Function>::try_from(function_export) {
                v8::Global::new(scope, function)
            } else {
                return Ok(RuntimeExecuteFunctionResult::Missing);
            }
        };

        let result_receiver = self.js_runtime.call(&function);

        let run_event_loop_result = self
            .js_runtime
            .run_event_loop(deno_core::PollEventLoopOptions::default())
            .await;

        if let Err(error) = run_event_loop_result {
            return Ok(RuntimeExecuteFunctionResult::Error(error));
        }

        if let Err(error) = result_receiver.await {
            return Ok(RuntimeExecuteFunctionResult::Error(error));
        }

        Ok(RuntimeExecuteFunctionResult::Success)
    }
}
