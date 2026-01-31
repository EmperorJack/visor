use deno_core::{OpState, op2};
use nannou::color;
use visor_engine::{AccessSketchStore, Draw};

use crate::{
    draw_plugin::{DrawId, ShapeId, SketchState},
    shape::{ShapeCommand, ShapeKind},
};

pub(crate) fn build_rect(draw: &Draw, commands: Vec<ShapeCommand>) {
    let _rect = commands
        .into_iter()
        .fold(draw.inner.rect(), |drawing, command| match command {
            ShapeCommand::Xy { x, y } => drawing.x_y(x, y),
            ShapeCommand::Xyz { x, y, z } => drawing.x_y_z(x, y, z),
            ShapeCommand::Wh { w, h } => drawing.w_h(w, h),
            ShapeCommand::FillRgba { r, g, b, a } => drawing.rgba(r, g, b, a),
            ShapeCommand::FillHsva { h, s, v, a } => drawing.hsva(h, s, v, a),
            ShapeCommand::NoFill => drawing.no_fill(),
            ShapeCommand::StrokeRgba { r, g, b, a } => drawing.stroke(color::rgba(r, g, b, a)),
            ShapeCommand::StrokeHsva { h, s, v, a } => drawing.stroke(color::hsva(h, s, v, a)),
            ShapeCommand::StrokeWeight { w } => drawing.stroke_weight(w),
            _ => panic!("Unexpected: invalid shape command for rect"),
        });
}

#[op2(fast)]
pub(crate) fn op_draw_rect(state: &mut OpState, draw_id: u32) -> u32 {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state
        .start_drawing_shape(ShapeKind::Rect, DrawId(draw_id))
        .0
}

#[op2(fast)]
pub(crate) fn op_draw_rect_xy(state: &mut OpState, shape_id: u32, x: f32, y: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::Xy { x, y });
}

#[op2(fast)]
pub(crate) fn op_draw_rect_xyz(state: &mut OpState, shape_id: u32, x: f32, y: f32, z: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::Xyz { x, y, z });
}

#[op2(fast)]
pub(crate) fn op_draw_rect_wh(state: &mut OpState, shape_id: u32, w: f32, h: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::Wh { w, h });
}

#[op2(fast)]
pub(crate) fn op_draw_rect_fill_rgba(
    state: &mut OpState,
    shape_id: u32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::FillRgba { r, g, b, a });
}

#[op2(fast)]
pub(crate) fn op_draw_rect_fill_hsva(
    state: &mut OpState,
    shape_id: u32,
    h: f32,
    s: f32,
    v: f32,
    a: f32,
) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::FillHsva { h, s, v, a });
}

#[op2(fast)]
pub(crate) fn op_draw_rect_no_fill(state: &mut OpState, shape_id: u32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::NoFill);
}

#[op2(fast)]
pub(crate) fn op_draw_rect_stroke_rgba(
    state: &mut OpState,
    shape_id: u32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::StrokeRgba { r, g, b, a });
}

#[op2(fast)]
pub(crate) fn op_draw_rect_stroke_hsva(
    state: &mut OpState,
    shape_id: u32,
    h: f32,
    s: f32,
    v: f32,
    a: f32,
) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::StrokeHsva { h, s, v, a });
}

#[op2(fast)]
pub(crate) fn op_draw_rect_stroke_weight(state: &mut OpState, shape_id: u32, w: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::StrokeWeight { w });
}
