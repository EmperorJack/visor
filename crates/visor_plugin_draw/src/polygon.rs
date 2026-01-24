use std::collections::HashMap;

use deno_core::{OpState, op2};
use nannou::{
    color,
    draw::{Drawing, primitive::PolygonInit},
};
use visor_engine::AccessSketchStore;

use crate::draw_plugin::{DrawId, ShapeCommand, ShapeId, SketchState};

pub(crate) type PolygonCommandMap = HashMap<ShapeId, (DrawId, Vec<PolygonCommand>)>;

pub(crate) enum PolygonCommand {
    Xy { x: f32, y: f32 },
    Xyz { x: f32, y: f32, z: f32 },
    Point { x: f32, y: f32 },
    FillRgba { r: f32, g: f32, b: f32, a: f32 },
    FillHsva { h: f32, s: f32, v: f32, a: f32 },
    NoFill,
    StrokeRgba { r: f32, g: f32, b: f32, a: f32 },
    StrokeHsva { h: f32, s: f32, v: f32, a: f32 },
    StrokeWeight { w: f32 },
}

impl ShapeCommand<PolygonInit> for PolygonCommand {
    fn apply<'a>(&self, drawing: Drawing<'a, PolygonInit>) -> Drawing<'a, PolygonInit> {
        match *self {
            Self::Xy { x, y } => drawing.x_y(x, y),
            Self::Xyz { x, y, z } => drawing.x_y_z(x, y, z),
            Self::Point { .. } => panic!("Unexpected: cannot apply polygon point command"),
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
pub(crate) fn op_draw_polygon(state: &mut OpState, draw_id: u32) -> u32 {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.start_drawing_polygon(DrawId(draw_id)).0
}

#[op2(fast)]
pub(crate) fn op_draw_polygon_xy(state: &mut OpState, shape_id: u32, x: f32, y: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_polygon_command(ShapeId(shape_id), PolygonCommand::Xy { x, y });
}

#[op2(fast)]
pub(crate) fn op_draw_polygon_xyz(state: &mut OpState, shape_id: u32, x: f32, y: f32, z: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_polygon_command(ShapeId(shape_id), PolygonCommand::Xyz { x, y, z });
}

#[op2(fast)]
pub(crate) fn op_draw_polygon_point(state: &mut OpState, shape_id: u32, x: f32, y: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_polygon_command(ShapeId(shape_id), PolygonCommand::Point { x, y });
}

#[op2(fast)]
pub(crate) fn op_draw_polygon_fill_rgba(
    state: &mut OpState,
    shape_id: u32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_polygon_command(ShapeId(shape_id), PolygonCommand::FillRgba { r, g, b, a });
}

#[op2(fast)]
pub(crate) fn op_draw_polygon_fill_hsva(
    state: &mut OpState,
    shape_id: u32,
    h: f32,
    s: f32,
    v: f32,
    a: f32,
) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_polygon_command(ShapeId(shape_id), PolygonCommand::FillHsva { h, s, v, a });
}

#[op2(fast)]
pub(crate) fn op_draw_polygon_no_fill(state: &mut OpState, shape_id: u32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_polygon_command(ShapeId(shape_id), PolygonCommand::NoFill);
}

#[op2(fast)]
pub(crate) fn op_draw_polygon_stroke_rgba(
    state: &mut OpState,
    shape_id: u32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state
        .store_polygon_command(ShapeId(shape_id), PolygonCommand::StrokeRgba { r, g, b, a });
}

#[op2(fast)]
pub(crate) fn op_draw_polygon_stroke_hsva(
    state: &mut OpState,
    shape_id: u32,
    h: f32,
    s: f32,
    v: f32,
    a: f32,
) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state
        .store_polygon_command(ShapeId(shape_id), PolygonCommand::StrokeHsva { h, s, v, a });
}

#[op2(fast)]
pub(crate) fn op_draw_polygon_stroke_weight(state: &mut OpState, shape_id: u32, w: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_polygon_command(ShapeId(shape_id), PolygonCommand::StrokeWeight { w });
}
