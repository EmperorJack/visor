use std::collections::HashMap;

use deno_core::{op2, OpState};
use nannou::draw::{primitive::Ellipse, Drawing};

use crate::{clamp_draw_id, DrawId, ShapeCommand, ShapeId};

pub(crate) type EllipseCommandMap = HashMap<ShapeId, (DrawId, Vec<EllipseCommand>)>;

pub(crate) enum EllipseCommand {
    XY { x: f32, y: f32 },
    XYZ { x: f32, y: f32, z: f32 },
    WH { w: f32, h: f32 },
    RGB { r: f32, g: f32, b: f32 },
    RGBA { r: f32, g: f32, b: f32, a: f32 },
    HSV { h: f32, s: f32, v: f32 },
    HSVA { h: f32, s: f32, v: f32, a: f32 },
}

impl ShapeCommand<Ellipse> for EllipseCommand {
    fn apply<'a>(&self, drawing: Drawing<'a, Ellipse>) -> Drawing<'a, Ellipse> {
        match *self {
            Self::XY { x, y } => drawing.x_y(x, y),
            Self::XYZ { x, y, z } => drawing.x_y_z(x, y, z),
            Self::WH { w, h } => drawing.w_h(w, h),
            Self::RGB { r, g, b } => drawing.rgb(r, g, b),
            Self::RGBA { r, g, b, a } => drawing.rgba(r, g, b, a),
            Self::HSV { h, s, v } => drawing.hsv(h, s, v),
            Self::HSVA { h, s, v, a } => drawing.hsva(h, s, v, a),
        }
    }
}

fn store_ellipse_command(state: &mut OpState, id: &ShapeId, command: EllipseCommand) {
    let shape_command_map = state.borrow_mut::<EllipseCommandMap>();

    shape_command_map
        .get_mut(id)
        .expect("Unexpected: could not find shape commands for given id")
        .1
        .push(command);
}

#[op2(fast)]
pub(crate) fn op_draw_ellipse(state: &mut OpState, draw_id: u32) -> u32 {
    let mut id = state.take::<ShapeId>();
    id.increment();

    let draw_id = clamp_draw_id(state, DrawId(draw_id));

    let shape_command_map = state.borrow_mut::<EllipseCommandMap>();
    shape_command_map.insert(id, (draw_id, Vec::new()));

    state.put(id);

    id.0
}

#[op2(fast)]
pub(crate) fn op_draw_ellipse_xy(state: &mut OpState, shape_id: u32, x: f32, y: f32) {
    store_ellipse_command(state, &ShapeId(shape_id), EllipseCommand::XY { x, y });
}

#[op2(fast)]
pub(crate) fn op_draw_ellipse_xyz(state: &mut OpState, shape_id: u32, x: f32, y: f32, z: f32) {
    store_ellipse_command(state, &ShapeId(shape_id), EllipseCommand::XYZ { x, y, z });
}

#[op2(fast)]
pub(crate) fn op_draw_ellipse_wh(state: &mut OpState, shape_id: u32, w: f32, h: f32) {
    store_ellipse_command(state, &ShapeId(shape_id), EllipseCommand::WH { w, h });
}

#[op2(fast)]
pub(crate) fn op_draw_ellipse_rgb(state: &mut OpState, shape_id: u32, r: f32, g: f32, b: f32) {
    store_ellipse_command(state, &ShapeId(shape_id), EllipseCommand::RGB { r, g, b });
}

#[op2(fast)]
pub(crate) fn op_draw_ellipse_rgba(
    state: &mut OpState,
    shape_id: u32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
) {
    store_ellipse_command(
        state,
        &ShapeId(shape_id),
        EllipseCommand::RGBA { r, g, b, a },
    );
}

#[op2(fast)]
pub(crate) fn op_draw_ellipse_hsv(state: &mut OpState, shape_id: u32, h: f32, s: f32, v: f32) {
    store_ellipse_command(state, &ShapeId(shape_id), EllipseCommand::HSV { h, s, v });
}

#[op2(fast)]
pub(crate) fn op_draw_ellipse_hsva(
    state: &mut OpState,
    shape_id: u32,
    h: f32,
    s: f32,
    v: f32,
    a: f32,
) {
    store_ellipse_command(
        state,
        &ShapeId(shape_id),
        EllipseCommand::HSVA { h, s, v, a },
    );
}
