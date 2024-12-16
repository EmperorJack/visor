use std::collections::HashMap;

use deno_core::{op2, OpState};
use nannou::draw::{primitive::Rect, Drawing};
use visor_engine::plugin::AccessSketchStore;

use crate::{DrawId, ShapeCommand, ShapeId, SketchState};

pub(crate) type RectCommandMap = HashMap<ShapeId, (DrawId, Vec<RectCommand>)>;

pub(crate) enum RectCommand {
    XY { x: f32, y: f32 },
    XYZ { x: f32, y: f32, z: f32 },
    WH { w: f32, h: f32 },
    RGB { r: f32, g: f32, b: f32 },
    RGBA { r: f32, g: f32, b: f32, a: f32 },
    HSV { h: f32, s: f32, v: f32 },
    HSVA { h: f32, s: f32, v: f32, a: f32 },
}

impl ShapeCommand<Rect> for RectCommand {
    fn apply<'a>(&self, drawing: Drawing<'a, Rect>) -> Drawing<'a, Rect> {
        match *self {
            Self::XY { x, y } => drawing.x_y(x, y),
            Self::XYZ { x, y, z } => drawing.x_y_z(x, y, z),
            Self::WH { w, h } => drawing.w_h(w, h),
            Self::RGB { r, g, b } => drawing.rgb(r, g, b),
            Self::RGBA { r, g, b, a } => drawing.rgba(r, g, b, a),
            Self::HSV { h, s, v } => drawing.hsv(h, s, v),
            Self::HSVA { h, s, v, a } => drawing.hsva(h, s, v, a),
        }
    }
}

#[op2(fast)]
pub(crate) fn op_draw_rect(state: &mut OpState, draw_id: u32) -> u32 {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.start_drawing_rect(DrawId(draw_id)).0
}

#[op2(fast)]
pub(crate) fn op_draw_rect_xy(state: &mut OpState, shape_id: u32, x: f32, y: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_rect_command(ShapeId(shape_id), RectCommand::XY { x, y });
}

#[op2(fast)]
pub(crate) fn op_draw_rect_xyz(state: &mut OpState, shape_id: u32, x: f32, y: f32, z: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_rect_command(ShapeId(shape_id), RectCommand::XYZ { x, y, z });
}

#[op2(fast)]
pub(crate) fn op_draw_rect_wh(state: &mut OpState, shape_id: u32, w: f32, h: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_rect_command(ShapeId(shape_id), RectCommand::WH { w, h });
}

#[op2(fast)]
pub(crate) fn op_draw_rect_rgb(state: &mut OpState, shape_id: u32, r: f32, g: f32, b: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_rect_command(ShapeId(shape_id), RectCommand::RGB { r, g, b });
}

#[op2(fast)]
pub(crate) fn op_draw_rect_rgba(
    state: &mut OpState,
    shape_id: u32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_rect_command(ShapeId(shape_id), RectCommand::RGBA { r, g, b, a });
}

#[op2(fast)]
pub(crate) fn op_draw_rect_hsv(state: &mut OpState, shape_id: u32, h: f32, s: f32, v: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_rect_command(ShapeId(shape_id), RectCommand::HSV { h, s, v });
}

#[op2(fast)]
pub(crate) fn op_draw_rect_hsva(
    state: &mut OpState,
    shape_id: u32,
    h: f32,
    s: f32,
    v: f32,
    a: f32,
) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_rect_command(ShapeId(shape_id), RectCommand::HSVA { h, s, v, a });
}
