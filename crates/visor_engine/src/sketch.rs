use std::path::PathBuf;

use tokio::{
    runtime::Handle,
    sync::{mpsc, oneshot},
};
use uuid::Uuid;

use crate::{
    draw::Draw,
    engine::RenderTextureId,
    sketch_worker::{SketchWorker, SketchWorkerTask},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SketchId(pub Uuid);

pub struct Sketch {
    runtime_handle: Handle,
    id: SketchId,
    file_path: PathBuf,
    draw: Draw,
    is_enabled: bool,
    target_render_texture_id: Option<RenderTextureId>,
    worker_task_sender: mpsc::Sender<SketchWorkerTask>,
}

impl Sketch {
    pub(crate) fn new(runtime_handle: Handle, id: SketchId, file_path: PathBuf) -> Self {
        let (worker_task_sender, worker_task_receiver) = mpsc::channel::<SketchWorkerTask>(1);

        let draw = Draw::default();

        {
            let file_path = file_path.clone();
            let draw = draw.clone();

            std::thread::spawn(move || {
                SketchWorker::new(id, file_path, draw, worker_task_receiver).run();
            });
        }

        Self {
            runtime_handle,
            id,
            file_path,
            draw,
            is_enabled: true,
            target_render_texture_id: None,
            worker_task_sender,
        }
    }

    pub fn id(&self) -> &SketchId {
        &self.id
    }

    pub fn file_path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn draw(&self) -> &nannou::Draw {
        &self.draw.inner
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    pub fn set_enabled(&mut self, id: bool) {
        self.is_enabled = id;
    }

    pub fn get_target_render_texture_id(&self) -> Option<&RenderTextureId> {
        self.target_render_texture_id.as_ref()
    }

    pub fn set_target_render_texture_id(&mut self, id: Option<&RenderTextureId>) {
        self.target_render_texture_id = id.copied();
    }

    pub fn recompile(&self) {
        self.runtime_handle.block_on(async {
            let (result_sender, result_receiver) = oneshot::channel::<()>();

            self.worker_task_sender
                .send(SketchWorkerTask::Compile(result_sender))
                .await
                .expect("Unexpected: could not send update task to sketch worker");

            result_receiver
                .await
                .expect("Unexpected: error occurred during sketch compile");
        });
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
