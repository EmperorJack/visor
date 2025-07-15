use std::collections::HashMap;

use deno_core::{OpState, op2};
use nannou::{
    color,
    draw::{Drawing, primitive::Quad},
};
use visor_engine::AccessSketchStore;

use crate::{DrawId, ShapeCommand, ShapeId, SketchState};

pub(crate) type QuadCommandMap = HashMap<ShapeId, (DrawId, Vec<QuadCommand>)>;

pub(crate) enum QuadCommand {
    Xy {
        x: f32,
        y: f32,
    },
    Xyz {
        x: f32,
        y: f32,
        z: f32,
    },
    Points {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        x4: f32,
        y4: f32,
    },
    FillRgba {
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    },
    FillHsva {
        h: f32,
        s: f32,
        v: f32,
        a: f32,
    },
    NoFill,
    StrokeRgba {
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    },
    StrokeHsva {
        h: f32,
        s: f32,
        v: f32,
        a: f32,
    },
    StrokeWeight {
        w: f32,
    },
}

impl ShapeCommand<Quad> for QuadCommand {
    fn apply<'a>(&self, drawing: Drawing<'a, Quad>) -> Drawing<'a, Quad> {
        match *self {
            Self::Xy { x, y } => drawing.x_y(x, y),
            Self::Xyz { x, y, z } => drawing.x_y_z(x, y, z),
            Self::Points {
                x1,
                y1,
                x2,
                y2,
                x3,
                y3,
                x4,
                y4,
            } => drawing.points((x1, y1), (x2, y2), (x3, y3), (x4, y4)),
            Self::FillRgba { r, g, b, a } => drawing.rgba(r, g, b, a),
            Self::FillHsva { h, s, v, a } => drawing.hsva(h, s, v, a),
            Self::NoFill => drawing.no_fill(),
            Self::StrokeRgba { r, g, b, a } => drawing.stroke_color(color::rgba(r, g, b, a)),
            Self::StrokeHsva { h, s, v, a } => drawing.stroke_color(color::hsva(h, s, v, a)),
            Self::StrokeWeight { w } => drawing.stroke_weight(w),
        }
    }
}

#[op2(fast)]
pub(crate) fn op_draw_quad(state: &mut OpState, draw_id: u32) -> u32 {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.start_drawing_quad(DrawId(draw_id)).0
}

#[op2(fast)]
pub(crate) fn op_draw_quad_xy(state: &mut OpState, shape_id: u32, x: f32, y: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_quad_command(ShapeId(shape_id), QuadCommand::Xy { x, y });
}

#[op2(fast)]
pub(crate) fn op_draw_quad_xyz(state: &mut OpState, shape_id: u32, x: f32, y: f32, z: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_quad_command(ShapeId(shape_id), QuadCommand::Xyz { x, y, z });
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

    sketch_state.store_quad_command(
        ShapeId(shape_id),
        QuadCommand::Points {
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

    sketch_state.store_quad_command(ShapeId(shape_id), QuadCommand::FillRgba { r, g, b, a });
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

    sketch_state.store_quad_command(ShapeId(shape_id), QuadCommand::FillHsva { h, s, v, a });
}

#[op2(fast)]
pub(crate) fn op_draw_quad_no_fill(state: &mut OpState, shape_id: u32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_quad_command(ShapeId(shape_id), QuadCommand::NoFill);
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

    sketch_state.store_quad_command(ShapeId(shape_id), QuadCommand::StrokeRgba { r, g, b, a });
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

    sketch_state.store_quad_command(ShapeId(shape_id), QuadCommand::StrokeHsva { h, s, v, a });
}

#[op2(fast)]
pub(crate) fn op_draw_quad_stroke_weight(state: &mut OpState, shape_id: u32, w: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_quad_command(ShapeId(shape_id), QuadCommand::StrokeWeight { w });
}
