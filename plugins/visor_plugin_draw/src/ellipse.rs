use std::collections::HashMap;

use deno_core::{op2, OpState};
use nannou::{
    color,
    draw::{primitive::Ellipse, Drawing},
};
use visor_engine::plugin::AccessSketchStore;

use crate::{DrawId, ShapeCommand, ShapeId, SketchState};

pub(crate) type EllipseCommandMap = HashMap<ShapeId, (DrawId, Vec<EllipseCommand>)>;

pub(crate) enum EllipseCommand {
    Xy { x: f32, y: f32 },
    Xyz { x: f32, y: f32, z: f32 },
    Wh { w: f32, h: f32 },
    FillRgba { r: f32, g: f32, b: f32, a: f32 },
    FillHsva { h: f32, s: f32, v: f32, a: f32 },
    NoFill,
    StrokeRgba { r: f32, g: f32, b: f32, a: f32 },
    StrokeHsva { h: f32, s: f32, v: f32, a: f32 },
    StrokeWeight { w: f32 },
}

impl ShapeCommand<Ellipse> for EllipseCommand {
    fn apply<'a>(&self, drawing: Drawing<'a, Ellipse>) -> Drawing<'a, Ellipse> {
        match *self {
            Self::Xy { x, y } => drawing.x_y(x, y),
            Self::Xyz { x, y, z } => drawing.x_y_z(x, y, z),
            Self::Wh { w, h } => drawing.w_h(w, h),
            Self::FillRgba { r, g, b, a } => drawing.rgba(r, g, b, a),
            Self::FillHsva { h, s, v, a } => drawing.hsva(h, s, v, a),
            Self::NoFill => drawing.no_fill(),
            Self::StrokeRgba { r, g, b, a } => drawing.stroke(color::rgba(r, g, b, a)),
            Self::StrokeHsva { h, s, v, a } => drawing.stroke(color::hsva(h, s, v, a)),
            Self::StrokeWeight { w } => drawing.stroke_weight(w),
        }
    }
}

#[op2(fast)]
pub(crate) fn op_draw_ellipse(state: &mut OpState, draw_id: u32) -> u32 {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.start_drawing_ellipse(DrawId(draw_id)).0
}

#[op2(fast)]
pub(crate) fn op_draw_ellipse_xy(state: &mut OpState, shape_id: u32, x: f32, y: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_ellipse_command(ShapeId(shape_id), EllipseCommand::Xy { x, y });
}

#[op2(fast)]
pub(crate) fn op_draw_ellipse_xyz(state: &mut OpState, shape_id: u32, x: f32, y: f32, z: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_ellipse_command(ShapeId(shape_id), EllipseCommand::Xyz { x, y, z });
}

#[op2(fast)]
pub(crate) fn op_draw_ellipse_wh(state: &mut OpState, shape_id: u32, w: f32, h: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_ellipse_command(ShapeId(shape_id), EllipseCommand::Wh { w, h });
}

#[op2(fast)]
pub(crate) fn op_draw_ellipse_fill_rgba(
    state: &mut OpState,
    shape_id: u32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_ellipse_command(ShapeId(shape_id), EllipseCommand::FillRgba { r, g, b, a });
}

#[op2(fast)]
pub(crate) fn op_draw_ellipse_fill_hsva(
    state: &mut OpState,
    shape_id: u32,
    h: f32,
    s: f32,
    v: f32,
    a: f32,
) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_ellipse_command(ShapeId(shape_id), EllipseCommand::FillHsva { h, s, v, a });
}

#[op2(fast)]
pub(crate) fn op_draw_ellipse_no_fill(state: &mut OpState, shape_id: u32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_ellipse_command(ShapeId(shape_id), EllipseCommand::NoFill);
}

#[op2(fast)]
pub(crate) fn op_draw_ellipse_stroke_rgba(
    state: &mut OpState,
    shape_id: u32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state
        .store_ellipse_command(ShapeId(shape_id), EllipseCommand::StrokeRgba { r, g, b, a });
}

#[op2(fast)]
pub(crate) fn op_draw_ellipse_stroke_hsva(
    state: &mut OpState,
    shape_id: u32,
    h: f32,
    s: f32,
    v: f32,
    a: f32,
) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state
        .store_ellipse_command(ShapeId(shape_id), EllipseCommand::StrokeHsva { h, s, v, a });
}

#[op2(fast)]
pub(crate) fn op_draw_ellipse_stroke_weight(state: &mut OpState, shape_id: u32, w: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_ellipse_command(ShapeId(shape_id), EllipseCommand::StrokeWeight { w });
}
