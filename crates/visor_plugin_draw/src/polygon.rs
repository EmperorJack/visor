use deno_core::{OpState, op2};
use nannou::color;
use visor_engine::AccessSketchStore;

use crate::{
    draw_plugin::{DrawId, ShapeId, SketchState},
    shape::{ShapeCommand, ShapeKind},
};

pub(crate) fn build_polygon(draw: &visor_engine::Draw, commands: Vec<ShapeCommand>) {
    let mut points: Vec<(f32, f32)> = vec![];

    let polygon =
        commands
            .into_iter()
            .fold(draw.inner.polygon(), |drawing, command| match command {
                ShapeCommand::Xy { x, y } => drawing.x_y(x, y),
                ShapeCommand::Xyz { x, y, z } => drawing.x_y_z(x, y, z),
                ShapeCommand::Point { x, y } => {
                    points.push((x, y));
                    drawing
                }
                ShapeCommand::FillRgba { r, g, b, a } => drawing.rgba(r, g, b, a),
                ShapeCommand::FillHsva { h, s, v, a } => drawing.hsva(h, s, v, a),
                ShapeCommand::NoFill => drawing.no_fill(),
                ShapeCommand::StrokeRgba { r, g, b, a } => drawing.stroke(color::rgba(r, g, b, a)),
                ShapeCommand::StrokeHsva { h, s, v, a } => drawing.stroke(color::hsva(h, s, v, a)),
                ShapeCommand::StrokeWeight { w } => drawing.stroke_weight(w),
                _ => panic!("Unexpected: invalid shape command for polygon"),
            });

    if points.is_empty() {
        return;
    }

    let _polygon = polygon.points(points);
}

#[op2(fast)]
pub(crate) fn op_draw_polygon(state: &mut OpState, draw_id: u32) -> u32 {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state
        .start_drawing_shape(ShapeKind::Polygon, DrawId(draw_id))
        .0
}

#[op2(fast)]
pub(crate) fn op_draw_polygon_xy(state: &mut OpState, shape_id: u32, x: f32, y: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::Xy { x, y });
}

#[op2(fast)]
pub(crate) fn op_draw_polygon_xyz(state: &mut OpState, shape_id: u32, x: f32, y: f32, z: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::Xyz { x, y, z });
}

#[op2(fast)]
pub(crate) fn op_draw_polygon_point(state: &mut OpState, shape_id: u32, x: f32, y: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::Point { x, y });
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

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::FillRgba { r, g, b, a });
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

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::FillHsva { h, s, v, a });
}

#[op2(fast)]
pub(crate) fn op_draw_polygon_no_fill(state: &mut OpState, shape_id: u32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::NoFill);
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

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::StrokeRgba { r, g, b, a });
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

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::StrokeHsva { h, s, v, a });
}

#[op2(fast)]
pub(crate) fn op_draw_polygon_stroke_weight(state: &mut OpState, shape_id: u32, w: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::StrokeWeight { w });
}
