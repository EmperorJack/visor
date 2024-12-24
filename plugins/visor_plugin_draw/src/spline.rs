use std::collections::HashMap;

use deno_core::{op2, OpState};
use nannou::{
    color,
    draw::{primitive::PathStroke, Drawing},
};
use visor_engine::plugin::AccessSketchStore;

use crate::{DrawId, ShapeCommand, ShapeId, SketchState};

pub(crate) type SplineCommandMap = HashMap<ShapeId, (DrawId, Vec<SplineCommand>)>;

pub(crate) enum SplineCommand {
    Xyz { x: f32, y: f32, z: f32 },
    Point { x: f32, y: f32 },
    StrokeRgba { r: f32, g: f32, b: f32, a: f32 },
    StrokeHsva { h: f32, s: f32, v: f32, a: f32 },
    StrokeWeight { w: f32 },
    Tension { t: f32 },
    Resolution { n: u32 },
}

impl ShapeCommand<PathStroke> for SplineCommand {
    fn apply<'a>(&self, drawing: Drawing<'a, PathStroke>) -> Drawing<'a, PathStroke> {
        match *self {
            Self::Xyz { x, y, z } => drawing.x_y_z(x, y, z),
            Self::Point { .. } => panic!("Unexpected: cannot apply spline point command"),
            Self::StrokeRgba { r, g, b, a } => drawing.color(color::rgba(r, g, b, a)),
            Self::StrokeHsva { h, s, v, a } => drawing.color(color::hsva(h, s, v, a)),
            Self::StrokeWeight { w } => drawing.weight(w),
            Self::Tension { .. } => panic!("Unexpected: cannot apply spline tension command"),
            Self::Resolution { .. } => panic!("Unexpected: cannot apply spline resolution command"),
        }
    }
}

#[op2(fast)]
pub(crate) fn op_draw_spline(state: &mut OpState, draw_id: u32) -> u32 {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.start_drawing_spline(DrawId(draw_id)).0
}

#[op2(fast)]
pub(crate) fn op_draw_spline_xyz(state: &mut OpState, shape_id: u32, x: f32, y: f32, z: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_spline_command(ShapeId(shape_id), SplineCommand::Xyz { x, y, z });
}

#[op2(fast)]
pub(crate) fn op_draw_spline_point(state: &mut OpState, shape_id: u32, x: f32, y: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_spline_command(ShapeId(shape_id), SplineCommand::Point { x, y });
}

#[op2(fast)]
pub(crate) fn op_draw_spline_stroke_rgba(
    state: &mut OpState,
    shape_id: u32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_spline_command(ShapeId(shape_id), SplineCommand::StrokeRgba { r, g, b, a });
}

#[op2(fast)]
pub(crate) fn op_draw_spline_stroke_hsva(
    state: &mut OpState,
    shape_id: u32,
    h: f32,
    s: f32,
    v: f32,
    a: f32,
) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_spline_command(ShapeId(shape_id), SplineCommand::StrokeHsva { h, s, v, a });
}

#[op2(fast)]
pub(crate) fn op_draw_spline_stroke_weight(state: &mut OpState, shape_id: u32, w: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_spline_command(ShapeId(shape_id), SplineCommand::StrokeWeight { w });
}

#[op2(fast)]
pub(crate) fn op_draw_spline_tension(state: &mut OpState, shape_id: u32, t: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_spline_command(ShapeId(shape_id), SplineCommand::Tension { t });
}

#[op2(fast)]
pub(crate) fn op_draw_spline_resolution(state: &mut OpState, shape_id: u32, n: u32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_spline_command(ShapeId(shape_id), SplineCommand::Resolution { n });
}
