use std::path::PathBuf;

use anyhow::Error;
use tokio::sync::{mpsc, oneshot};
use visor_draw::draw::Draw;
use visor_runtime::runtime::{Runtime, RuntimeExecuteFunctionResult, SketchFunction};

pub(crate) enum SketchWorkerTask {
    Compile(oneshot::Sender<()>),
    Update(oneshot::Sender<()>),
}

pub(crate) struct SketchWorker {
    file_path: PathBuf,
    draw: Draw,
    task_receiver: mpsc::Receiver<SketchWorkerTask>,
    runtime: Option<Runtime>,
    request_compile: bool,
    request_setup: bool,
}

impl SketchWorker {
    pub fn new(
        file_path: PathBuf,
        draw: Draw,
        task_receiver: mpsc::Receiver<SketchWorkerTask>,
    ) -> Self {
        Self {
            file_path,
            draw,
            task_receiver,
            runtime: None,
            request_compile: true,
            request_setup: true,
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
                    SketchWorkerTask::Compile(result_sender) => {
                        self.request_compile = true;

                        result_sender
                            .send(())
                            .expect("Unexpected: could not send result back to sketch");
                    }

                    SketchWorkerTask::Update(result_sender) => {
                        self.update().await;

                        result_sender
                            .send(())
                            .expect("Unexpected: could not send result back to sketch");
                    }
                }
            }
        });
    }

    async fn update(&mut self) {
        let mut error: Option<Error> = None;

        if self.request_compile {
            // Drop the current runtime if there is one
            self.runtime = None;

            let (runtime, compile_error) = Runtime::compile(&self.file_path, self.draw.clone())
                .await
                .expect("Unexpected: could not compile sketch into runtime");

            error = compile_error;

            self.runtime = runtime;

            self.request_compile = false;
        }

        if let Some(error) = error {
            println!("Compile error: {}", error);

            return;
        }

        if let Some(runtime) = &mut self.runtime {
            let runtime_error =
                Self::execute_sketch_lifecycle(self.request_setup, &self.draw.inner, runtime).await;

            self.request_setup = false;

            error = runtime_error;

            if let Some(error) = error {
                println!("Runtime error: {}", error);
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
