use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use crate::sketch_worker::{SketchWorker, SketchWorkerTask};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SketchId(pub(crate) Uuid);

pub(crate) struct Sketch {
    worker_task_sender: mpsc::Sender<SketchWorkerTask>,
}

impl Sketch {
    pub fn new() -> Self {
        let (worker_task_sender, worker_task_receiver) = mpsc::channel::<SketchWorkerTask>(1);

        std::thread::spawn(move || {
            SketchWorker::new(worker_task_receiver).run();
        });

        Self { worker_task_sender }
    }

    pub async fn request_update(&self) -> oneshot::Receiver<()> {
        let (result_sender, result_receiver) = oneshot::channel::<()>();

        self.worker_task_sender
            .send(SketchWorkerTask::Update(result_sender))
            .await
            .expect("Unexpected: could not send update task to sketch worker");

        result_receiver
    }
}
