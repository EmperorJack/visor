use std::{path::Path, rc::Rc};

use anyhow::Error;
use deno_core::{error::AnyError, v8, Extension, JsRuntime, ModuleId};

use crate::{startup_snapshot::STARTUP_SNAPSHOT_CELL, ts_module_loader::TsModuleLoader};

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
    main_module_id: Option<ModuleId>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
    pub fn new() -> Self {
        let startup_snapshot = STARTUP_SNAPSHOT_CELL
            .get()
            .expect("Unexpected: startup snapshot should be created by now");

        let ops_extension = Extension {
            name: "ops",
            ops: std::borrow::Cow::Borrowed(&startup_snapshot.ops),
            ..Default::default()
        };

        let js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
            module_loader: Some(Rc::new(TsModuleLoader)),
            startup_snapshot: Some(&startup_snapshot.snapshot),
            skip_op_registration: true,
            extensions: vec![ops_extension],
            ..Default::default()
        });

        Self {
            js_runtime,
            main_module_id: None,
        }
    }

    pub async fn compile(&mut self, path: &Path) -> Result<Option<Error>, AnyError> {
        let path = path
            .to_str()
            .expect("Unexpected: could not convert file path into string");
        let current_dir =
            std::env::current_dir().expect("Unexpected: could not get current working directory");

        let main_module = deno_core::resolve_path(path, &current_dir)?;

        let main_module_id = match self.js_runtime.load_main_es_module(&main_module).await {
            Ok(main_module_id) => main_module_id,
            Err(error) => return Ok(Some(error)),
        };

        let result_receiver = self.js_runtime.mod_evaluate(main_module_id);

        let run_event_loop_result = self
            .js_runtime
            .run_event_loop(deno_core::PollEventLoopOptions::default())
            .await;

        if let Err(error) = run_event_loop_result {
            return Ok(Some(error));
        }

        if let Err(error) = result_receiver.await {
            return Ok(Some(error));
        }

        self.main_module_id = Some(main_module_id);

        Ok(None)
    }

    pub fn put_state<T: 'static>(&mut self, state: T) {
        self.js_runtime.op_state().borrow_mut().put(state);
    }

    pub fn take_state<T: 'static>(&mut self) -> T {
        self.js_runtime.op_state().borrow_mut().take()
    }

    pub fn has_state<T: 'static>(&mut self) -> bool {
        self.js_runtime.op_state().borrow().has::<T>()
    }

    pub async fn execute_runtime_function(
        &mut self,
        sketch_function: SketchFunction,
    ) -> RuntimeExecuteFunctionResult {
        self.execute_sketch_function_inner(&sketch_function)
            .await
            .unwrap_or_else(|_| {
                panic!(
                    "Unexpected: could not call {} function on sketch",
                    sketch_function.export_name()
                )
            })
    }

    async fn execute_sketch_function_inner(
        &mut self,
        sketch_function: &SketchFunction,
    ) -> Result<RuntimeExecuteFunctionResult, AnyError> {
        let function_export_name = sketch_function.export_name();

        let function = {
            let module_namespace =
                self.js_runtime
                    .get_module_namespace(self.main_module_id.expect(
                    "Runtime error: runtime must be compiled before executing a sketch function!",
                ))?;

            let scope = &mut self.js_runtime.handle_scope();

            let module_namespace = v8::Local::new(scope, module_namespace);

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
