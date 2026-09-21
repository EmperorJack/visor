use std::collections::HashMap;

use deno_core::{OpState, op2};
use nannou::{
    color,
    draw::{Drawing, primitive::PathStroke},
};
use visor_engine::AccessSketchStore;

use crate::draw_plugin::{DrawId, ShapeCommand, ShapeId, ShapeType, SketchState};

pub(crate) type PolylineCommandMap = HashMap<ShapeId, (DrawId, Vec<PolylineCommand>)>;

pub(crate) enum PolylineCommand {
    Xyz { x: f32, y: f32, z: f32 },
    Point { x: f32, y: f32 },
    StrokeRgba { r: f32, g: f32, b: f32, a: f32 },
    StrokeHsva { h: f32, s: f32, v: f32, a: f32 },
    StrokeWeight { w: f32 },
}

impl ShapeCommand<PathStroke> for PolylineCommand {
    fn apply<'a>(&self, drawing: Drawing<'a, PathStroke>) -> Drawing<'a, PathStroke> {
        match *self {
            Self::Xyz { x, y, z } => drawing.x_y_z(x, y, z),
            Self::Point { .. } => panic!("Unexpected: cannot apply polyline point command"),
            Self::StrokeRgba { r, g, b, a } => drawing.color(color::rgba(r, g, b, a)),
            Self::StrokeHsva { h, s, v, a } => drawing.color(color::hsva(h, s, v, a)),
            Self::StrokeWeight { w } => drawing.weight(w),
        }
    }
}

impl SketchState {
    pub(crate) fn start_drawing_polyline(&mut self, draw_id: DrawId) -> ShapeId {
        self.next_shape_id.0 += 1;

        let draw_id = self.clamp_draw_id(draw_id);

        self.polyline_command_map
            .insert(self.next_shape_id, (draw_id, Vec::new()));

        self.shape_order
            .push((self.next_shape_id, ShapeType::Polyline));

        self.next_shape_id
    }

    pub(crate) fn store_polyline_command(&mut self, id: ShapeId, command: PolylineCommand) {
        self.polyline_command_map
            .get_mut(&id)
            .expect("Unexpected: could not find shape commands for given id")
            .1
            .push(command);
    }
}

#[op2(fast)]
pub(crate) fn op_draw_polyline(state: &mut OpState, draw_id: u32) -> u32 {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.start_drawing_polyline(DrawId(draw_id)).0
}

#[op2(fast)]
pub(crate) fn op_draw_polyline_xyz(state: &mut OpState, shape_id: u32, x: f32, y: f32, z: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_polyline_command(ShapeId(shape_id), PolylineCommand::Xyz { x, y, z });
}

#[op2(fast)]
pub(crate) fn op_draw_polyline_point(state: &mut OpState, shape_id: u32, x: f32, y: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_polyline_command(ShapeId(shape_id), PolylineCommand::Point { x, y });
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

    sketch_state.store_polyline_command(
        ShapeId(shape_id),
        PolylineCommand::StrokeRgba { r, g, b, a },
    );
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

    sketch_state.store_polyline_command(
        ShapeId(shape_id),
        PolylineCommand::StrokeHsva { h, s, v, a },
    );
}

#[op2(fast)]
pub(crate) fn op_draw_polyline_stroke_weight(state: &mut OpState, shape_id: u32, w: f32) {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.store_polyline_command(ShapeId(shape_id), PolylineCommand::StrokeWeight { w });
}
