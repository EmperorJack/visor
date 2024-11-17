use std::path::PathBuf;

use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use crate::{
    draw::Draw,
    engine::RenderTextureId,
    sketch_worker::{SketchWorker, SketchWorkerTask},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SketchId(pub Uuid);

pub struct Sketch {
    draw: Draw,
    target_render_texture_id: Option<RenderTextureId>,
    worker_task_sender: mpsc::Sender<SketchWorkerTask>,
}

impl Sketch {
    pub(crate) fn new(id: SketchId, file_path: PathBuf) -> Self {
        let (worker_task_sender, worker_task_receiver) = mpsc::channel::<SketchWorkerTask>(1);

        let draw = Draw::default();

        {
            let draw = draw.clone();

            std::thread::spawn(move || {
                SketchWorker::new(id, file_path, draw, worker_task_receiver).run();
            });
        }

        Self {
            draw,
            target_render_texture_id: None,
            worker_task_sender,
        }
    }

    pub fn get_draw(&self) -> &nannou::Draw {
        &self.draw.inner
    }

    pub fn get_target_render_texture_id(&self) -> Option<&RenderTextureId> {
        self.target_render_texture_id.as_ref()
    }

    pub fn set_target_render_texture_id(&mut self, id: Option<&RenderTextureId>) {
        self.target_render_texture_id = id.copied();
    }

    pub(crate) async fn request_compile(&self) -> oneshot::Receiver<()> {
        let (result_sender, result_receiver) = oneshot::channel::<()>();

        self.worker_task_sender
            .send(SketchWorkerTask::Compile(result_sender))
            .await
            .expect("Unexpected: could not send update task to sketch worker");

        result_receiver
    }

    pub(crate) async fn request_update(&self) -> oneshot::Receiver<()> {
        let (result_sender, result_receiver) = oneshot::channel::<()>();

        self.worker_task_sender
            .send(SketchWorkerTask::Update(result_sender))
            .await
            .expect("Unexpected: could not send update task to sketch worker");

        result_receiver
    }
}
