use tokio::sync::{mpsc, oneshot};

pub(crate) enum SketchWorkerTask {
    Update(oneshot::Sender<()>),
}

pub(crate) struct SketchWorker {
    task_receiver: mpsc::Receiver<SketchWorkerTask>,
}

impl SketchWorker {
    pub fn new(task_receiver: mpsc::Receiver<SketchWorkerTask>) -> Self {
        Self { task_receiver }
    }

    pub fn run(&mut self) {
        let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Unexpected: could not create tokio runtime for sketch worker");

        tokio_runtime.block_on(async {
            while let Some(task) = self.task_receiver.recv().await {
                match task {
                    SketchWorkerTask::Update(result_sender) => {
                        // TODO: update sketch

                        result_sender
                            .send(())
                            .expect("Unexpected: could not send result back to sketch");
                    }
                }
            }
        });
    }
}
