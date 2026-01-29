use std::collections::HashMap;

use deno_core::{OpState, op2};
use nannou::{
    color,
    draw::{Drawing, primitive::PathFill},
};
use visor_engine::AccessSketchStore;

use crate::draw_plugin::{DrawId, ShapeCommand, ShapeId, SketchState};

pub(crate) type PathCommandMap = HashMap<ShapeId, (DrawId, Vec<PathCommand>)>;

pub(crate) enum PathCommand {
    Xy { x: f32, y: f32 },
    Xyz { x: f32, y: f32, z: f32 },
    Point { x: f32, y: f32 },
    FillRgba { r: f32, g: f32, b: f32, a: f32 },
    FillHsva { h: f32, s: f32, v: f32, a: f32 },
    Tension { t: f32 },
    Resolution { n: u32 },
}

impl ShapeCommand<PathFill> for PathCommand {
    fn apply<'a>(&self, drawing: Drawing<'a, PathFill>) -> Drawing<'a, PathFill> {
        match *self {
            Self::Xy { x, y } => drawing.x_y(x, y),
            Self::Xyz { x, y, z } => drawing.x_y_z(x, y, z),
            Self::Point { .. } => panic!("Unexpected: cannot apply path point command"),
            Self::FillRgba { r, g, b, a } => drawing.color(color::rgba(r, g, b, a)),
            Self::FillHsva { h, s, v, a } => drawing.color(color::hsva(h, s, v, a)),
            Self::Tension { .. } => panic!("Unexpected: cannot apply spline tension command"),
            Self::Resolution { .. } => panic!("Unexpected: cannot apply spline resolution command"),
        }
    }
}

#[op2(fast)]
pub(crate) fn op_draw_path(state: &mut OpState, draw_id: u32) -> u32 {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.start_drawing_path(DrawId(draw_id)).0
}

#[op2(fast)]
pub(crate) fn op_draw_path_xy(state: &mut OpState, shape_id: u32, x: f32, y: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_path_command(ShapeId(shape_id), PathCommand::Xy { x, y });
}

#[op2(fast)]
pub(crate) fn op_draw_path_xyz(state: &mut OpState, shape_id: u32, x: f32, y: f32, z: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_path_command(ShapeId(shape_id), PathCommand::Xyz { x, y, z });
}

#[op2(fast)]
pub(crate) fn op_draw_path_point(state: &mut OpState, shape_id: u32, x: f32, y: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_path_command(ShapeId(shape_id), PathCommand::Point { x, y });
}

#[op2(fast)]
pub(crate) fn op_draw_path_fill_rgba(
    state: &mut OpState,
    shape_id: u32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_path_command(ShapeId(shape_id), PathCommand::FillRgba { r, g, b, a });
}

#[op2(fast)]
pub(crate) fn op_draw_path_fill_hsva(
    state: &mut OpState,
    shape_id: u32,
    h: f32,
    s: f32,
    v: f32,
    a: f32,
) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_path_command(ShapeId(shape_id), PathCommand::FillHsva { h, s, v, a });
}

#[op2(fast)]
pub(crate) fn op_draw_path_tension(state: &mut OpState, shape_id: u32, t: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_path_command(ShapeId(shape_id), PathCommand::Tension { t });
}

#[op2(fast)]
pub(crate) fn op_draw_path_resolution(state: &mut OpState, shape_id: u32, n: u32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_path_command(ShapeId(shape_id), PathCommand::Resolution { n });
}
