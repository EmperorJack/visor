use deno_core::{OpState, op2};
use nannou::color;
use visor_engine::AccessSketchStore;

use crate::{
    draw_plugin::{DrawId, ShapeId, SketchState},
    shape::{ShapeCommand, ShapeKind},
};

pub(crate) fn build_polyline(draw: &visor_engine::Draw, commands: Vec<ShapeCommand>) {
    let mut points: Vec<(f32, f32)> = vec![];

    let polyline =
        commands
            .into_iter()
            .fold(draw.inner.polyline(), |drawing, command| match command {
                ShapeCommand::Xyz { x, y, z } => drawing.x_y_z(x, y, z),
                ShapeCommand::Point { x, y } => {
                    points.push((x, y));
                    drawing
                }
                ShapeCommand::StrokeRgba { r, g, b, a } => drawing.color(color::rgba(r, g, b, a)),
                ShapeCommand::StrokeHsva { h, s, v, a } => drawing.color(color::hsva(h, s, v, a)),
                ShapeCommand::StrokeWeight { w } => drawing.stroke_weight(w),
                _ => panic!("Unexpected: invalid shape command for polyline"),
            });

    if points.is_empty() {
        return;
    }

    let _polyline = polyline.points(points);
}

#[op2(fast)]
pub(crate) fn op_draw_polyline(state: &mut OpState, draw_id: u32) -> u32 {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state
        .start_drawing_shape(ShapeKind::Polyline, DrawId(draw_id))
        .0
}

#[op2(fast)]
pub(crate) fn op_draw_polyline_xyz(state: &mut OpState, shape_id: u32, x: f32, y: f32, z: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::Xyz { x, y, z });
}

#[op2(fast)]
pub(crate) fn op_draw_polyline_point(state: &mut OpState, shape_id: u32, x: f32, y: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::Point { x, y });
}

#[op2(fast)]
pub(crate) fn op_draw_polyline_stroke_rgba(
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
pub(crate) fn op_draw_polyline_stroke_hsva(
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
pub(crate) fn op_draw_polyline_stroke_weight(state: &mut OpState, shape_id: u32, w: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::StrokeWeight { w });
}
