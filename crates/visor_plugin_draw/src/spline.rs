use bevy_math::{
    Vec2,
    cubic_splines::{CubicCardinalSpline, CubicGenerator},
};
use deno_core::{OpState, op2};
use nannou::color;
use visor_engine::AccessSketchStore;

use crate::{
    draw_plugin::{DrawId, ShapeId, SketchState},
    shape::{ShapeCommand, ShapeKind},
};

pub(crate) fn build_spline(draw: &visor_engine::Draw, commands: Vec<ShapeCommand>) {
    let mut points: Vec<Vec2> = vec![];
    let mut tension: f32 = 0.5;
    let mut resolution: Option<usize> = None;

    let spline =
        commands
            .into_iter()
            .fold(draw.inner.polyline(), |drawing, command| match command {
                ShapeCommand::Xyz { x, y, z } => drawing.x_y_z(x, y, z),
                ShapeCommand::Point { x, y } => {
                    points.push((x, y).into());
                    drawing
                }
                ShapeCommand::StrokeRgba { r, g, b, a } => drawing.color(color::rgba(r, g, b, a)),
                ShapeCommand::StrokeHsva { h, s, v, a } => drawing.color(color::hsva(h, s, v, a)),
                ShapeCommand::StrokeWeight { w } => drawing.stroke_weight(w),
                ShapeCommand::Tension { t } => {
                    tension = t;
                    drawing
                }
                ShapeCommand::Resolution { n } => {
                    resolution = Some(n as usize);
                    drawing
                }
                _ => panic!("Unexpected: invalid shape command for spline"),
            });

    if points.is_empty() {
        return;
    }

    let points: Vec<_> = if tension > 0.0 {
        let resolution = resolution.unwrap_or_else(|| points.len() * 20);

        let curve = CubicCardinalSpline::new(tension, points).to_curve();

        curve
            .iter_positions(resolution)
            .map(|point| (point.x, point.y))
            .collect()
    } else {
        points.into_iter().map(|point| (point.x, point.y)).collect()
    };

    let _spline = spline.points(points);
}

#[op2(fast)]
pub(crate) fn op_draw_spline(state: &mut OpState, draw_id: u32) -> u32 {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state
        .start_drawing_shape(ShapeKind::Spline, DrawId(draw_id))
        .0
}

#[op2(fast)]
pub(crate) fn op_draw_spline_xyz(state: &mut OpState, shape_id: u32, x: f32, y: f32, z: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::Xyz { x, y, z });
}

#[op2(fast)]
pub(crate) fn op_draw_spline_point(state: &mut OpState, shape_id: u32, x: f32, y: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::Point { x, y });
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

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::StrokeRgba { r, g, b, a });
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

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::StrokeHsva { h, s, v, a });
}

#[op2(fast)]
pub(crate) fn op_draw_spline_stroke_weight(state: &mut OpState, shape_id: u32, w: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::StrokeWeight { w });
}

#[op2(fast)]
pub(crate) fn op_draw_spline_tension(state: &mut OpState, shape_id: u32, t: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::Tension { t });
}

#[op2(fast)]
pub(crate) fn op_draw_spline_resolution(state: &mut OpState, shape_id: u32, n: u32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::Resolution { n });
}
