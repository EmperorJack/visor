use tokio::sync::{mpsc, oneshot};

use crate::draw::Draw;

pub(crate) enum SketchWorkerTask {
    Update(oneshot::Sender<()>),
}

pub(crate) struct SketchWorker {
    draw: Draw,
    task_receiver: mpsc::Receiver<SketchWorkerTask>,
}

impl SketchWorker {
    pub fn new(draw: Draw, task_receiver: mpsc::Receiver<SketchWorkerTask>) -> Self {
        Self {
            draw,
            task_receiver,
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
                    SketchWorkerTask::Update(result_sender) => {
                        self.draw
                            .inner
                            .rect()
                            .w_h(50.0, 50.0)
                            .color(nannou::prelude::RED);

                        result_sender
                            .send(())
                            .expect("Unexpected: could not send result back to sketch");
                    }
                }
            }
        });
    }
}
