use std::collections::HashMap;

use deno_core::{extension, op2, Extension, OpState};
use visor_engine::{draw::Draw, plugin::Plugin, sketch::SketchId, store::Store, Runtime};

pub struct DrawPlugin;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct DrawId(u32);
type DrawMap = HashMap<DrawId, Draw>;

impl DrawId {
    fn increment(&mut self) {
        self.0 += 1
    }
}

extension!(
    extension,
    ops = [op_draw_ellipse, op_draw_rect, op_draw_translate, op_draw_rotate],
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
        runtime.put_state(DrawMap::default());
        runtime.put_state(DrawId(0));
    }
}

fn get_draw(state: &OpState, id: DrawId) -> &Draw {
    if id.0 == 0 {
        return state.borrow::<Draw>();
    }

    if let Some(draw) = state.borrow::<DrawMap>().get(&id) {
        return draw;
    }

    // Return base draw if the given draw ID is invalid
    return state.borrow::<Draw>();
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
fn op_draw_ellipse(state: &OpState, id: u32, x: f32, y: f32, width: f32, height: f32) {
    let draw = get_draw(state, DrawId(id));

    draw.inner
        .ellipse()
        .x_y(x, y)
        .w_h(width, height)
        .color(nannou::prelude::RED);
}

#[op2(fast)]
fn op_draw_rect(state: &OpState, id: u32, x: f32, y: f32, width: f32, height: f32) {
    let draw = get_draw(state, DrawId(id));

    draw.inner
        .rect()
        .x_y(x, y)
        .w_h(width, height)
        .color(nannou::prelude::RED);
}

#[op2(fast)]
fn op_draw_translate(state: &mut OpState, id: u32, x: f32, y: f32) -> u32 {
    let draw = get_draw(state, DrawId(id));

    let draw = draw.inner.x_y(x, y);

    store_draw(state, Draw { inner: draw }).0
}

#[op2(fast)]
fn op_draw_rotate(state: &mut OpState, id: u32, radians: f32) -> u32 {
    let draw = get_draw(state, DrawId(id));

    let draw = draw.inner.rotate(radians);

    store_draw(state, Draw { inner: draw }).0
}
