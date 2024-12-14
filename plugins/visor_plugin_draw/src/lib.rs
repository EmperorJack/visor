use std::collections::HashMap;

use deno_core::{extension, op2, Extension, OpState};
use ellipse::{
    op_draw_ellipse, op_draw_ellipse_hsv, op_draw_ellipse_hsva, op_draw_ellipse_rgb,
    op_draw_ellipse_rgba, op_draw_ellipse_wh, op_draw_ellipse_xy, op_draw_ellipse_xyz,
    EllipseCommandMap,
};
use nannou::draw::Drawing;
use rect::{
    op_draw_rect, op_draw_rect_hsv, op_draw_rect_hsva, op_draw_rect_rgb, op_draw_rect_rgba,
    op_draw_rect_wh, op_draw_rect_xy, op_draw_rect_xyz, RectCommandMap,
};
use visor_engine::{draw::Draw, plugin::Plugin, sketch::SketchId, store::Store, Runtime};

mod ellipse;
mod rect;

pub struct DrawPlugin;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct DrawId(u32);
type DrawMap = HashMap<DrawId, Draw>;

impl DrawId {
    fn increment(&mut self) {
        self.0 += 1
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct ShapeId(u32);

impl ShapeId {
    fn increment(&mut self) {
        self.0 += 1
    }
}

pub(crate) trait ShapeCommand<T> {
    fn apply<'a>(&self, drawing: Drawing<'a, T>) -> Drawing<'a, T>;
}

extension!(
    extension,
    ops = [
        op_draw_background,
        op_draw_ellipse,
        op_draw_ellipse_xy,
        op_draw_ellipse_xyz,
        op_draw_ellipse_wh,
        op_draw_ellipse_rgb,
        op_draw_ellipse_rgba,
        op_draw_ellipse_hsv,
        op_draw_ellipse_hsva,
        op_draw_rect,
        op_draw_rect_xy,
        op_draw_rect_xyz,
        op_draw_rect_wh,
        op_draw_rect_rgb,
        op_draw_rect_rgba,
        op_draw_rect_hsv,
        op_draw_rect_hsva,
        op_draw_translate,
        op_draw_rotate,
        op_draw_scale,
    ],
    esm_entry_point = "visor:draw",
    esm = [
        dir "src",
        "visor:draw" = "visor-draw.js",
    ]
);

impl Plugin for DrawPlugin {
    fn extension(&self) -> Extension {
        extension::init_ops_and_esm()
    }

    fn before_sketch_update(&self, _sketch_id: &SketchId, runtime: &mut Runtime, _store: &Store) {
        runtime.put_state(DrawId(0));
        runtime.put_state(ShapeId(0));

        // TODO: create maps only when first needed
        runtime.put_state(DrawMap::default());
        runtime.put_state(EllipseCommandMap::default());
        runtime.put_state(RectCommandMap::default());
    }

    fn after_sketch_update(&self, _sketch_id: &SketchId, runtime: &mut Runtime, _store: &Store) {
        let ellipse_command_map = runtime.take_state::<EllipseCommandMap>();
        let rect_command_map = runtime.take_state::<RectCommandMap>();

        let op_state = runtime.op_state();
        let op_state = op_state.borrow_mut();

        for (draw_id, commands) in ellipse_command_map.values() {
            let draw = get_draw(&op_state, draw_id);

            let mut ellipse = draw.inner.ellipse();

            for command in commands {
                ellipse = command.apply(ellipse);
            }
        }

        for (draw_id, commands) in rect_command_map.values() {
            let draw = get_draw(&op_state, draw_id);

            let mut rect = draw.inner.rect();

            for command in commands {
                rect = command.apply(rect);
            }
        }
    }
}

fn get_draw<'a>(state: &'a OpState, id: &DrawId) -> &'a Draw {
    if id.0 == 0 {
        return state.borrow::<Draw>();
    }

    if let Some(draw) = state.borrow::<DrawMap>().get(id) {
        return draw;
    }

    // Return base draw if the given draw ID is invalid
    return state.borrow::<Draw>();
}

pub(crate) fn clamp_draw_id(state: &OpState, id: DrawId) -> DrawId {
    if id.0 == 0 {
        return id;
    }

    if id.0 <= state.borrow::<DrawId>().0 {
        return id;
    }

    return DrawId(0);
}

fn store_draw(state: &mut OpState, draw: Draw) -> DrawId {
    let mut id = state.take::<DrawId>();
    id.increment();

    let draw_map = state.borrow_mut::<DrawMap>();
    draw_map.insert(id, draw);

    state.put(id);

    id
}

#[op2(fast)]
fn op_draw_background(state: &OpState, id: u32, r: f32, g: f32, b: f32) {
    let draw = get_draw(state, &DrawId(id));

    draw.inner.background().rgb(r, g, b);
}

#[op2(fast)]
fn op_draw_translate(state: &mut OpState, id: u32, x: f32, y: f32) -> u32 {
    let draw = get_draw(state, &DrawId(id));

    let draw = draw.inner.x_y(x, y);

    store_draw(state, Draw { inner: draw }).0
}

#[op2(fast)]
fn op_draw_rotate(state: &mut OpState, id: u32, radians: f32) -> u32 {
    let draw = get_draw(state, &DrawId(id));

    let draw = draw.inner.rotate(radians);

    store_draw(state, Draw { inner: draw }).0
}

#[op2(fast)]
fn op_draw_scale(state: &mut OpState, id: u32, s: f32) -> u32 {
    let draw = get_draw(state, &DrawId(id));

    let draw = draw.inner.scale(s);

    store_draw(state, Draw { inner: draw }).0
}
