use std::{path::Path, rc::Rc};

use anyhow::{Error, Result};
use deno_core::{Extension, JsRuntime, ModuleId, error::AnyError, v8};

use crate::sketch_runtime::{
    startup_snapshot::STARTUP_SNAPSHOT_CELL, ts_module_loader::TsModuleLoader,
};

pub(crate) enum RuntimeExecuteFunctionResult {
    Success,
    Missing,
    Error(Error),
}

pub(crate) struct SketchRuntime {
    tokio_runtime_handle: tokio::runtime::Handle,
    js_runtime: JsRuntime,
    main_module_id: Option<ModuleId>,
}

impl SketchRuntime {
    pub(crate) fn new(
        tokio_runtime_handle: tokio::runtime::Handle,
        extensions: Vec<Extension>,
    ) -> Self {
        let startup_snapshot = STARTUP_SNAPSHOT_CELL
            .get()
            .expect("Unexpected: startup snapshot should be created by now");

        let js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
            module_loader: Some(Rc::new(TsModuleLoader)),
            startup_snapshot: Some(&startup_snapshot.0),
            extensions,
            ..Default::default()
        });

        Self {
            tokio_runtime_handle,
            js_runtime,
            main_module_id: None,
        }
    }

    pub(crate) fn compile(&mut self, path: &Path) -> Result<Option<Error>, AnyError> {
        self.tokio_runtime_handle.block_on(async {
            let path = path
                .to_str()
                .expect("Unexpected: could not convert file path into string");
            let current_dir = std::env::current_dir()
                .expect("Unexpected: could not get current working directory");

            let main_module = deno_core::resolve_path(path, &current_dir)?;

            let main_module_id = match self.js_runtime.load_main_es_module(&main_module).await {
                Ok(main_module_id) => main_module_id,
                Err(error) => return Ok(Some(error.into())),
            };

            let result_receiver = self.js_runtime.mod_evaluate(main_module_id);

            let run_event_loop_result = self
                .js_runtime
                .run_event_loop(deno_core::PollEventLoopOptions::default())
                .await;

            if let Err(error) = run_event_loop_result {
                return Ok(Some(error.into()));
            }

            if let Err(error) = result_receiver.await {
                return Ok(Some(error.into()));
            }

            self.main_module_id = Some(main_module_id);

            Ok(None)
        })
    }

    pub(crate) fn put_state<T: 'static>(&mut self, state: T) {
        self.js_runtime.op_state().borrow_mut().put(state);
    }

    pub(crate) fn take_state<T: 'static>(&mut self) -> T {
        self.js_runtime.op_state().borrow_mut().take()
    }

    pub(crate) fn execute_function(
        &mut self,
        function_export_name: &str,
    ) -> RuntimeExecuteFunctionResult {
        self.execute_sketch_function_inner(function_export_name)
            .unwrap_or_else(|_| {
                panic!(
                    "Unexpected: could not call {} function on sketch",
                    function_export_name
                )
            })
    }

    fn execute_sketch_function_inner(
        &mut self,
        function_export_name: &str,
    ) -> Result<RuntimeExecuteFunctionResult, AnyError> {
        self.tokio_runtime_handle.block_on(async {
            let function = {
                let module_namespace = self
                    .js_runtime
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
                return Ok(RuntimeExecuteFunctionResult::Error(error.into()));
            }

            if let Err(error) = result_receiver.await {
                return Ok(RuntimeExecuteFunctionResult::Error(error.into()));
            }

            Ok(RuntimeExecuteFunctionResult::Success)
        })
    }
}
