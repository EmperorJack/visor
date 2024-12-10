use std::path::PathBuf;

use anyhow::Error;
use tokio::sync::{mpsc, oneshot};
use visor_runtime::runtime::{Runtime, RuntimeExecuteFunctionResult};

use crate::{draw::Draw, engine::Engine, sketch::SketchId, store::ENGINE_STORE};

#[derive(Debug)]
pub(crate) struct SketchUpdateResult {
    pub id: SketchId,
    pub compile_error: Option<String>,
    pub runtime_error: Option<String>,
}

pub(crate) enum SketchWorkerTask {
    RequestCompile(oneshot::Sender<()>),
    Update(oneshot::Sender<SketchUpdateResult>),
}

pub(crate) struct SketchWorker {
    id: SketchId,
    file_path: PathBuf,
    draw: Draw,
    task_receiver: mpsc::Receiver<SketchWorkerTask>,
    runtime: Option<Runtime>,
    request_compile: bool,
    request_setup: bool,
    compile_error: Option<String>,
    runtime_error: Option<String>,
}

impl SketchWorker {
    pub fn new(
        id: SketchId,
        file_path: PathBuf,
        draw: Draw,
        task_receiver: mpsc::Receiver<SketchWorkerTask>,
    ) -> Self {
        Self {
            id,
            file_path,
            draw,
            task_receiver,
            runtime: None,
            request_compile: true,
            request_setup: true,
            compile_error: None,
            runtime_error: None,
        }
    }

    pub fn run(&mut self) {
        let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Unexpected: could not create tokio runtime for sketch worker");

        tokio_runtime.block_on(async {
            while let Some(task) = self.task_receiver.recv().await {
                match task {
                    SketchWorkerTask::RequestCompile(result_sender) => {
                        self.request_compile = true;

                        result_sender.send(()).expect(
                            "Unexpected: could not send request compile result back to sketch",
                        );
                    }

                    SketchWorkerTask::Update(result_sender) => {
                        self.update().await;

                        result_sender
                            .send(SketchErrors {
                                id: self.id,
                                compile_error: self.compile_error.clone(),
                                runtime_error: self.runtime_error.clone(),
                            })
                            .expect("Unexpected: could not send update result back to sketch");
                    }
                }
            }
        });
    }

    async fn update(&mut self) {
        if self.request_compile {
            // Drop the current runtime if there is one
            self.runtime = None;

            let mut runtime = Runtime::default();

            runtime.put_state(self.draw.clone());

            for plugin in Engine::plugins() {
                plugin.before_sketch_update(&self.id, &mut runtime, &ENGINE_STORE);
            }

            let compile_error = runtime
                .compile(&self.file_path)
                .await
                .expect("Unexpected: could not compile sketch into runtime");

            self.runtime = Some(runtime);

            self.compile_error = compile_error.map(|error| error.to_string());

            self.request_compile = false;
        } else if let Some(runtime) = &mut self.runtime {
            for plugin in Engine::plugins() {
                plugin.before_sketch_update(&self.id, runtime, &ENGINE_STORE);
            }
        }

        if let Some(runtime) = &mut self.runtime {
            if self.compile_error.is_none() {
                let runtime_error =
                    Self::execute_sketch_lifecycle(self.request_setup, &self.draw.inner, runtime)
                        .await;

                self.request_setup = false;

                self.runtime_error = runtime_error.map(|error| error.to_string());
            }

            for plugin in Engine::plugins() {
                plugin.after_sketch_update(&self.id, runtime, &ENGINE_STORE);
            }
        }
    }

    // TODO: should this return a Result?
    async fn execute_sketch_lifecycle(
        request_setup: bool,
        draw: &nannou::Draw,
        runtime: &mut Runtime,
    ) -> Option<Error> {
        if request_setup {
            if let RuntimeExecuteFunctionResult::Error(error) = runtime
                .execute_runtime_function(SketchFunction::Setup)
                .await
            {
                return Some(error);
            }
        }

        if let RuntimeExecuteFunctionResult::Error(error) = runtime
            .execute_runtime_function(SketchFunction::Update)
            .await
        {
            return Some(error);
        }

        draw.reset();
        draw.background().rgba(0.0, 0.0, 0.0, 0.0);

        if let RuntimeExecuteFunctionResult::Error(error) =
            runtime.execute_runtime_function(SketchFunction::Draw).await
        {
            return Some(error);
        }

        None
    }
}
