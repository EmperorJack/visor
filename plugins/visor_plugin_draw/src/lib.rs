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
use visor_engine::{
    draw::Draw,
    engine::Engine,
    plugin::{AccessSketchStore, Plugin},
    sketch::SketchId,
    sketch_store::SketchStore,
    store::Store,
};

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

    fn build_sketch(
        &self,
        sketch_id: &SketchId,
        engine: &mut Engine,
        _store: &Store,
        sketch_store: &mut SketchStore,
    ) {
        let draw = engine
            .sketches()
            .get(sketch_id)
            .expect("Unexpected: could not find sketch")
            .draw();

        sketch_store.set(draw.clone());

        sketch_store.set(DrawMap::default());
        sketch_store.set(EllipseCommandMap::default());
        sketch_store.set(RectCommandMap::default());
    }

    fn before_sketch_update(
        &self,
        _sketch_id: &SketchId,
        _store: &Store,
        sketch_store: &mut SketchStore,
    ) {
        sketch_store.get::<Draw>().inner.reset();

        sketch_store.set(DrawId(0));
        sketch_store.set(ShapeId(0));

        sketch_store.get_mut::<DrawMap>().clear();
    }

    fn after_sketch_update(
        &self,
        _sketch_id: &SketchId,
        _store: &Store,
        sketch_store: &mut SketchStore,
    ) {
        let mut ellipse_command_map = sketch_store.take::<EllipseCommandMap>();
        let mut rect_command_map = sketch_store.take::<RectCommandMap>();

        for (_, (draw_id, commands)) in ellipse_command_map.drain() {
            let draw = get_draw(&sketch_store, &draw_id);

            let mut ellipse = draw.inner.ellipse();

            for command in commands {
                ellipse = command.apply(ellipse);
            }
        }

        for (_, (draw_id, commands)) in rect_command_map.drain() {
            let draw = get_draw(sketch_store, &draw_id);

            let mut rect = draw.inner.rect();

            for command in commands {
                rect = command.apply(rect);
            }
        }

        sketch_store.set(ellipse_command_map);
        sketch_store.set(rect_command_map);
    }
}

fn get_draw<'a>(store: &'a SketchStore, id: &DrawId) -> &'a Draw {
    if id.0 == 0 {
        return store.get::<Draw>();
    }

    if let Some(draw) = store.get::<DrawMap>().get(id) {
        return draw;
    }

    // Return base draw if the given draw ID is invalid
    return store.get::<Draw>();
}

pub(crate) fn clamp_draw_id(store: &SketchStore, id: DrawId) -> DrawId {
    if id.0 == 0 {
        return id;
    }

    if id.0 <= store.get::<DrawId>().0 {
        return id;
    }

    return DrawId(0);
}

fn store_draw(store: &mut SketchStore, draw: Draw) -> DrawId {
    let mut id = store.take::<DrawId>();
    id.increment();

    let draw_map = store.get_mut::<DrawMap>();
    draw_map.insert(id, draw);

    store.set(id);

    id
}

#[op2(fast)]
fn op_draw_background(state: &OpState, id: u32, r: f32, g: f32, b: f32) {
    let store = state.sketch_store();

    let draw = get_draw(store, &DrawId(id));

    draw.inner.background().rgb(r, g, b);
}

#[op2(fast)]
fn op_draw_translate(state: &mut OpState, id: u32, x: f32, y: f32) -> u32 {
    let store = state.sketch_store_mut();

    let draw = get_draw(store, &DrawId(id));

    let draw = draw.inner.x_y(x, y);

    store_draw(store, Draw { inner: draw }).0
}

#[op2(fast)]
fn op_draw_rotate(state: &mut OpState, id: u32, radians: f32) -> u32 {
    let store = state.sketch_store_mut();

    let draw = get_draw(store, &DrawId(id));

    let draw = draw.inner.rotate(radians);

    store_draw(store, Draw { inner: draw }).0
}

#[op2(fast)]
fn op_draw_scale(state: &mut OpState, id: u32, s: f32) -> u32 {
    let store = state.sketch_store_mut();

    let draw = get_draw(store, &DrawId(id));

    let draw = draw.inner.scale(s);

    store_draw(store, Draw { inner: draw }).0
}
