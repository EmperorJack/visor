use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use deno_core::{OpState, op2};
use deno_error::JsErrorBox;
use nannou::draw::{Drawing, primitive::Texture};
use visor_engine::AccessSketchStore;

use crate::draw_plugin::{DrawId, ShapeCommand, ShapeId, ShapeType, SketchState};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct TextureId(pub(crate) u32);

impl TextureId {
    fn new(path: &Path) -> Self {
        let mut hasher = std::hash::DefaultHasher::new();

        path.hash(&mut hasher);

        Self(hasher.finish() as u32)
    }
}

pub(crate) type TextureCommandMap = HashMap<ShapeId, (DrawId, TextureId, Vec<TextureCommand>)>;

pub(crate) enum TextureCommand {
    Xy { x: f32, y: f32 },
    Xyz { x: f32, y: f32, z: f32 },
    Wh { w: f32, h: f32 },
    Opacity { opacity: f32 },
}

impl ShapeCommand<Texture> for TextureCommand {
    fn apply<'a>(&self, drawing: Drawing<'a, Texture>) -> Drawing<'a, Texture> {
        match *self {
            Self::Xy { x, y } => drawing.x_y(x, y),
            Self::Xyz { x, y, z } => drawing.x_y_z(x, y, z),
            Self::Wh { w, h } => drawing.w_h(w, h),
            Self::Opacity { opacity } => drawing.opacity(opacity),
        }
    }
}

impl SketchState {
    pub(crate) fn load_texture(&mut self, path: PathBuf) -> Result<TextureId> {
        let id = TextureId::new(&path);

        let texture = nannou::wgpu::Texture::load_from_path(
            &self.wgpu_handle.device,
            &self.wgpu_handle.queue,
            nannou::wgpu::TextureBuilder::default_image_texture_usage(),
            &path,
        )
        .with_context(|| format!("Could not load texture at path {}", path.display()))?;

        self.texture_map.insert(id, texture);

        Ok(id)
    }

    pub(crate) fn start_drawing_texture(
        &mut self,
        draw_id: DrawId,
        texture_id: TextureId,
    ) -> ShapeId {
        self.next_shape_id.0 += 1;

        let draw_id = self.clamp_draw_id(draw_id);

        self.texture_command_map
            .insert(self.next_shape_id, (draw_id, texture_id, Vec::new()));

        self.shape_order
            .push((self.next_shape_id, ShapeType::Texture));

        self.next_shape_id
    }

    pub(crate) fn store_texture_command(&mut self, id: ShapeId, command: TextureCommand) {
        self.texture_command_map
            .get_mut(&id)
            .expect("Unexpected: could not find shape commands for given id")
            .2
            .push(command);
    }
}

#[op2(fast)]
pub(crate) fn op_draw_texture_load(
    state: &mut OpState,
    #[string] path: String,
) -> Result<u32, JsErrorBox> {
    let state = state.sketch_store_mut().get_mut::<SketchState>();

    let path = build_path(&state.file_path, path.into());

    state
        .load_texture(path)
        .map(|id| id.0)
        .map_err(|error| JsErrorBox::generic(error.to_string()))
}

#[op2(fast)]
pub(crate) fn op_draw_texture_width(state: &OpState, texture_id: u32) -> u32 {
    state
        .sketch_store()
        .get::<SketchState>()
        .texture_map
        .get(&TextureId(texture_id))
        .expect("Unexpected: could not find texture for given id")
        .width()
}

#[op2(fast)]
pub(crate) fn op_draw_texture_height(state: &OpState, texture_id: u32) -> u32 {
    state
        .sketch_store()
        .get::<SketchState>()
        .texture_map
        .get(&TextureId(texture_id))
        .expect("Unexpected: could not find texture for given id")
        .height()
}

#[op2(fast)]
pub(crate) fn op_draw_texture(state: &mut OpState, draw_id: u32, texture_id: u32) -> u32 {
    state
        .sketch_store_mut()
        .get_mut::<SketchState>()
        .start_drawing_texture(DrawId(draw_id), TextureId(texture_id))
        .0
}

#[op2(fast)]
pub(crate) fn op_draw_texture_xy(state: &mut OpState, shape_id: u32, x: f32, y: f32) {
    state
        .sketch_store_mut()
        .get_mut::<SketchState>()
        .store_texture_command(ShapeId(shape_id), TextureCommand::Xy { x, y });
}

#[op2(fast)]
pub(crate) fn op_draw_texture_xyz(state: &mut OpState, shape_id: u32, x: f32, y: f32, z: f32) {
    state
        .sketch_store_mut()
        .get_mut::<SketchState>()
        .store_texture_command(ShapeId(shape_id), TextureCommand::Xyz { x, y, z });
}

#[op2(fast)]
pub(crate) fn op_draw_texture_wh(state: &mut OpState, shape_id: u32, w: f32, h: f32) {
    state
        .sketch_store_mut()
        .get_mut::<SketchState>()
        .store_texture_command(ShapeId(shape_id), TextureCommand::Wh { w, h });
}

#[op2(fast)]
pub(crate) fn op_draw_texture_opacity(state: &mut OpState, shape_id: u32, opacity: f32) {
    state
        .sketch_store_mut()
        .get_mut::<SketchState>()
        .store_texture_command(ShapeId(shape_id), TextureCommand::Opacity { opacity });
}

fn build_path(base: &Path, target: PathBuf) -> PathBuf {
    if let Some(dir) = base.parent() {
        return dir.join(target);
    }

    target
}
