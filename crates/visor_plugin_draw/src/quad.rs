use deno_core::{OpState, op2};
use nannou::color;
use visor_engine::{AccessSketchStore, Draw};

use crate::{
    draw_plugin::{DrawId, ShapeId, SketchState},
    shape::{ShapeCommand, ShapeKind},
};

pub(crate) fn build_quad(draw: &Draw, commands: Vec<ShapeCommand>) {
    let _quad = commands
        .into_iter()
        .fold(draw.inner.quad(), |drawing, command| match command {
            ShapeCommand::Xy { x, y } => drawing.x_y(x, y),
            ShapeCommand::Xyz { x, y, z } => drawing.x_y_z(x, y, z),
            ShapeCommand::Wh { w, h } => drawing.w_h(w, h),
            ShapeCommand::QuadPoints {
                x1,
                y1,
                x2,
                y2,
                x3,
                y3,
                x4,
                y4,
            } => drawing.points((x1, y1), (x2, y2), (x3, y3), (x4, y4)),
            ShapeCommand::FillRgba { r, g, b, a } => drawing.rgba(r, g, b, a),
            ShapeCommand::FillHsva { h, s, v, a } => drawing.hsva(h, s, v, a),
            ShapeCommand::NoFill => drawing.no_fill(),
            ShapeCommand::StrokeRgba { r, g, b, a } => {
                drawing.stroke_color(color::rgba(r, g, b, a))
            }
            ShapeCommand::StrokeHsva { h, s, v, a } => {
                drawing.stroke_color(color::hsva(h, s, v, a))
            }
            ShapeCommand::StrokeWeight { w } => drawing.stroke_weight(w),
            _ => panic!("Unexpected: invalid shape command for quad"),
        });
}

#[op2(fast)]
pub(crate) fn op_draw_quad(state: &mut OpState, draw_id: u32) -> u32 {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state
        .start_drawing_shape(ShapeKind::Quad, DrawId(draw_id))
        .0
}

#[op2(fast)]
pub(crate) fn op_draw_quad_xy(state: &mut OpState, shape_id: u32, x: f32, y: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::Xy { x, y });
}

#[op2(fast)]
pub(crate) fn op_draw_quad_xyz(state: &mut OpState, shape_id: u32, x: f32, y: f32, z: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::Xyz { x, y, z });
}

#[op2(fast)]
#[allow(clippy::too_many_arguments)]
pub(crate) fn op_draw_quad_points(
    state: &mut OpState,
    shape_id: u32,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    x4: f32,
    y4: f32,
) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(
        ShapeId(shape_id),
        ShapeCommand::QuadPoints {
            x1,
            y1,
            x2,
            y2,
            x3,
            y3,
            x4,
            y4,
        },
    );
}

#[op2(fast)]
pub(crate) fn op_draw_quad_fill_rgba(
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
pub(crate) fn op_draw_quad_fill_hsva(
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
pub(crate) fn op_draw_quad_no_fill(state: &mut OpState, shape_id: u32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::NoFill);
}

#[op2(fast)]
pub(crate) fn op_draw_quad_stroke_rgba(
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
pub(crate) fn op_draw_quad_stroke_hsva(
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
pub(crate) fn op_draw_quad_stroke_weight(state: &mut OpState, shape_id: u32, w: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_shape_command(ShapeId(shape_id), ShapeCommand::StrokeWeight { w });
}
