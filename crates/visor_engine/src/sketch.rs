use std::path::PathBuf;

use tokio::{
    runtime::Handle,
    sync::{mpsc, oneshot},
};
use uuid::Uuid;

use crate::{
    draw::Draw,
    sketch_store::SketchStore,
    sketch_worker::{SketchUpdateResult, SketchWorker, SketchWorkerTask},
    wgpu::render_texture::RenderTextureId,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SketchId(pub Uuid);

pub struct Sketch {
    runtime_handle: Handle,
    id: SketchId,
    file_path: PathBuf,
    draw: Draw,
    is_built: bool,
    is_enabled: bool,
    target_render_texture_id: Option<RenderTextureId>,
    worker_task_sender: mpsc::Sender<SketchWorkerTask>,
    compile_error: Option<String>,
    runtime_error: Option<String>,
}

impl Sketch {
    pub(crate) fn new(
        runtime_handle: Handle,
        id: SketchId,
        file_path: PathBuf,
        draw: Draw,
    ) -> Self {
        let (worker_task_sender, worker_task_receiver) = mpsc::channel::<SketchWorkerTask>(1);

        {
            let file_path = file_path.clone();

            std::thread::spawn(move || {
                SketchWorker::new(id, file_path, worker_task_receiver).run();
            });
        }

        Self {
            runtime_handle,
            id,
            file_path,
            draw,
            is_built: false,
            is_enabled: true,
            target_render_texture_id: None,
            worker_task_sender,
            compile_error: None,
            runtime_error: None,
        }
    }

    pub fn id(&self) -> &SketchId {
        &self.id
    }

    pub fn file_path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn draw(&self) -> &Draw {
        &self.draw
    }

    pub(crate) fn is_built(&self) -> bool {
        self.is_built
    }

    pub(crate) fn mark_built(&mut self) {
        self.is_built = true
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

    pub fn request_compile(&self) {
        self.runtime_handle.block_on(async {
            let (result_sender, result_receiver) = oneshot::channel();

            self.worker_task_sender
                .try_send(SketchWorkerTask::RequestCompile(result_sender))
                .expect("Unexpected: could not send request compile task to sketch worker");

            result_receiver
                .await
                .expect("Unexpected: error occurred during request sketch compile");
        });
    }

    pub fn request_setup(&self) {
        self.runtime_handle.block_on(async {
            let (result_sender, result_receiver) = oneshot::channel();

            self.worker_task_sender
                .try_send(SketchWorkerTask::RequestSetup(result_sender))
                .expect("Unexpected: could not send request setup task to sketch worker");

            result_receiver
                .await
                .expect("Unexpected: error occurred during request sketch setup");
        });
    }

    pub(crate) fn request_update(
        &self,
        store: SketchStore,
    ) -> oneshot::Receiver<SketchUpdateResult> {
        let (result_sender, result_receiver) = oneshot::channel();

        self.worker_task_sender
            .try_send(SketchWorkerTask::Update(store, result_sender))
            .expect("Unexpected: could not send update task to sketch worker");

        result_receiver
    }

    pub(crate) fn set_errors(
        &mut self,
        compile_error: Option<String>,
        runtime_error: Option<String>,
    ) {
        self.compile_error = compile_error;
        self.runtime_error = runtime_error;
    }

    pub fn compile_error(&self) -> &Option<String> {
        &self.compile_error
    }

    pub fn runtime_error(&self) -> &Option<String> {
        &self.runtime_error
    }
}
