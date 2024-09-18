use std::path::PathBuf;

use draw::draw::Draw;
use runtime::runtime::Runtime;
use tokio::sync::{mpsc, oneshot};

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
        self.draw
            .inner
            .rect()
            .w_h(50.0, 50.0)
            .color(nannou::prelude::RED);

        if self.request_compile {
            // Drop the current runtime if there is one
            self.runtime = None;

            let (runtime, compile_error) = Runtime::compile(&self.file_path)
                .await
                .expect("Unexpected: could not compile sketch into runtime");

            if let Some(error) = compile_error {
                println!("{}", error);
            }

            self.runtime = Some(runtime);

            self.request_compile = false;
        }
    }
}
