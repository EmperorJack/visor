use std::path::PathBuf;

use uuid::Uuid;

use crate::{
    SketchStore,
    draw::Draw,
    engine::Engine,
    sketch::{Sketch, SketchId},
};

pub struct SketchBuilder {
    id: Option<SketchId>,
    file_path: PathBuf,
}

impl SketchBuilder {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            id: None,
            file_path,
        }
    }

    pub fn with_id(mut self, id: SketchId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn build(self, engine: &mut Engine) -> &Sketch {
        let id = self.id.unwrap_or(SketchId(Uuid::new_v4()));

        let draw = Draw::default();

        let sketch = Sketch::new(
            engine.runtime_handle.clone(),
            id,
            self.file_path,
            draw.clone(),
        );

        let mut sketch_store = SketchStore::default();
        sketch_store.set(draw);

        engine.manage_sketch(sketch, sketch_store)
    }
}
