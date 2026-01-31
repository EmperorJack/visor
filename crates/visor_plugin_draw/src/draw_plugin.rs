use std::{collections::HashMap, sync::RwLock};

use deno_core::{Extension, OpState, extension, op2};
use visor_engine::{AccessSketchStore, Draw, Engine, Plugin, SketchId, SketchStore, Store};

use crate::polyline::*;
use crate::quad::*;
use crate::rect::*;
use crate::spline::*;
use crate::{ellipse::*, shape::Shape};
use crate::{path::*, shape::ShapeKind};
use crate::{polygon::*, shape::ShapeCommand};

pub struct DrawPlugin;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct DrawId(pub(crate) u32);
type DrawMap = HashMap<DrawId, Draw>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ShapeId(pub(crate) u32);

pub(crate) struct SketchState {
    draw: Draw,
    draw_map: DrawMap,
    next_draw_id: DrawId,
    pub(crate) next_shape_id: ShapeId,
    shapes: HashMap<ShapeId, Shape>,
    width: u32,
    height: u32,
}

type SketchSizeState = HashMap<SketchId, [u32; 2]>;

impl SketchState {
    fn get_draw(&self, id: DrawId) -> &Draw {
        if id.0 == 0 {
            return &self.draw;
        }

        if let Some(draw) = self.draw_map.get(&id) {
            return draw;
        }

        // Return base draw if the given draw ID is invalid
        &self.draw
    }

    fn store_draw(&mut self, draw: Draw) -> DrawId {
        self.next_draw_id.0 += 1;

        self.draw_map.insert(self.next_draw_id, draw);

        self.next_draw_id
    }

    pub(crate) fn start_drawing_shape(&mut self, kind: ShapeKind, draw_id: DrawId) -> ShapeId {
        self.next_shape_id.0 += 1;

        let draw_id = self.clamp_draw_id(draw_id);

        self.shapes.insert(
            self.next_shape_id,
            Shape {
                kind,
                draw_id,
                commands: Vec::new(),
            },
        );

        self.next_shape_id
    }

    pub(crate) fn store_shape_command(&mut self, id: ShapeId, command: ShapeCommand) {
        self.shapes
            .get_mut(&id)
            .expect("Unexpected: could not find shape commands for given id")
            .commands
            .push(command);
    }

    pub(crate) fn clamp_draw_id(&self, id: DrawId) -> DrawId {
        if id.0 == 0 {
            return id;
        }

        if id.0 <= self.next_draw_id.0 {
            return id;
        }

        DrawId(0)
    }

    fn apply_shape_commands(&mut self) {
        let shapes: Vec<_> = self.shapes.drain().collect();

        for (_, shape) in shapes {
            let draw = self.get_draw(shape.draw_id);

            match shape.kind {
                ShapeKind::Ellipse => build_ellipse(draw, shape.commands),
                ShapeKind::Rect => build_rect(draw, shape.commands),
                ShapeKind::Quad => build_quad(draw, shape.commands),
                ShapeKind::Polygon => build_polygon(draw, shape.commands),
                ShapeKind::Polyline => build_polyline(draw, shape.commands),
                ShapeKind::Spline => build_spline(draw, shape.commands),
                ShapeKind::Path => build_path(draw, shape.commands),
            }
        }
    }

    fn reset(&mut self) {
        self.draw.inner.reset();

        self.draw_map.clear();

        self.next_draw_id.0 = 0;
        self.next_shape_id.0 = 0;
    }
}

extension!(
    visor_plugin_draw,
    ops = [
        op_draw_background_rgb,
        op_draw_background_hsv,
        op_draw_ellipse,
        op_draw_ellipse_xy,
        op_draw_ellipse_xyz,
        op_draw_ellipse_wh,
        op_draw_ellipse_fill_rgba,
        op_draw_ellipse_fill_hsva,
        op_draw_ellipse_no_fill,
        op_draw_ellipse_stroke_rgba,
        op_draw_ellipse_stroke_hsva,
        op_draw_ellipse_stroke_weight,
        op_draw_rect,
        op_draw_rect_xy,
        op_draw_rect_xyz,
        op_draw_rect_wh,
        op_draw_rect_fill_rgba,
        op_draw_rect_fill_hsva,
        op_draw_rect_no_fill,
        op_draw_rect_stroke_rgba,
        op_draw_rect_stroke_hsva,
        op_draw_rect_stroke_weight,
        op_draw_quad,
        op_draw_quad_xy,
        op_draw_quad_xyz,
        op_draw_quad_points,
        op_draw_quad_fill_rgba,
        op_draw_quad_fill_hsva,
        op_draw_quad_no_fill,
        op_draw_quad_stroke_rgba,
        op_draw_quad_stroke_hsva,
        op_draw_quad_stroke_weight,
        op_draw_polygon,
        op_draw_polygon_xy,
        op_draw_polygon_xyz,
        op_draw_polygon_point,
        op_draw_polygon_fill_rgba,
        op_draw_polygon_fill_hsva,
        op_draw_polygon_no_fill,
        op_draw_polygon_stroke_rgba,
        op_draw_polygon_stroke_hsva,
        op_draw_polygon_stroke_weight,
        op_draw_polyline,
        op_draw_polyline_xyz,
        op_draw_polyline_point,
        op_draw_polyline_stroke_rgba,
        op_draw_polyline_stroke_hsva,
        op_draw_polyline_stroke_weight,
        op_draw_spline,
        op_draw_spline_xyz,
        op_draw_spline_point,
        op_draw_spline_stroke_rgba,
        op_draw_spline_stroke_hsva,
        op_draw_spline_stroke_weight,
        op_draw_spline_tension,
        op_draw_spline_resolution,
        op_draw_path,
        op_draw_path_xy,
        op_draw_path_xyz,
        op_draw_path_point,
        op_draw_path_fill_rgba,
        op_draw_path_fill_hsva,
        op_draw_path_tension,
        op_draw_path_resolution,
        op_draw_translate,
        op_draw_rotate,
        op_draw_scale,
        op_draw_scale_x,
        op_draw_scale_y,
        op_draw_width,
        op_draw_height,
    ],
    esm_entry_point = "ext:visor_plugin_draw/src/draw-plugin.ts",
    esm = [
        "src/color.ts",
        "src/draw.ts",
        "src/ellipse.ts",
        "src/ops.ts",
        "src/path.ts",
        "src/polygon.ts",
        "src/polyline.ts",
        "src/quad.ts",
        "src/rect.ts",
        "src/spline.ts",
        "src/draw-plugin.ts"
    ]
);

impl Plugin for DrawPlugin {
    fn extension(&self) -> Extension {
        visor_plugin_draw::init()
    }

    fn typescript_declaration(&self) -> Option<String> {
        Some(include_str!("draw-plugin.d.ts").into())
    }

    fn build(&self, _engine: &mut Engine, store: &Store) {
        store.set(RwLock::new(SketchSizeState::default()));
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

        sketch_store.set(SketchState {
            draw: draw.clone(),
            draw_map: Default::default(),
            next_draw_id: DrawId(0),
            next_shape_id: ShapeId(0),
            shapes: Default::default(),
            width: 0,
            height: 0,
        });
    }

    fn before_engine_update(&self, engine: &mut Engine, store: &Store) {
        let mut sketch_size_state = store
            .get::<RwLock<SketchSizeState>>()
            .write()
            .expect("Unexpected: could not acquire write lock for sketch size state");

        for sketch_id in engine.sketches().keys() {
            let render_texture_id = engine
                .sketches()
                .get(sketch_id)
                .expect("Unexpected: could not find sketch")
                .get_target_render_texture_id();

            let render_texture = render_texture_id.map(|id| {
                engine
                    .render_textures()
                    .get(id)
                    .expect("Unexpected: could not find render texture")
            });

            let size = render_texture
                .map(|render_texture| render_texture.texture_view().size())
                .unwrap_or([0, 0]);

            sketch_size_state.insert(*sketch_id, size);
        }
    }

    fn before_sketch_update(
        &self,
        sketch_id: &SketchId,
        store: &Store,
        sketch_store: &mut SketchStore,
    ) {
        let sketch_state = sketch_store.get_mut::<SketchState>();

        sketch_state.reset();

        let sketch_size_state = store
            .get::<RwLock<SketchSizeState>>()
            .read()
            .expect("Unexpected: could not acquire read lock for sketch size state");

        let sketch_size = sketch_size_state
            .get(sketch_id)
            .expect("Unexpected: could not get sketch size");

        sketch_state.width = sketch_size[0];
        sketch_state.height = sketch_size[1];
    }

    fn after_sketch_update(
        &self,
        _sketch_id: &SketchId,
        _store: &Store,
        sketch_store: &mut SketchStore,
    ) {
        let sketch_state = sketch_store.get_mut::<SketchState>();

        sketch_state.apply_shape_commands();
    }
}

#[op2(fast)]
fn op_draw_background_rgb(state: &OpState, id: u32, r: f32, g: f32, b: f32) {
    let sketch_state = state.sketch_store().get::<SketchState>();

    let draw = sketch_state.get_draw(DrawId(id));

    draw.inner.background().rgb(r, g, b);
}

#[op2(fast)]
fn op_draw_background_hsv(state: &OpState, id: u32, h: f32, s: f32, v: f32) {
    let sketch_state = state.sketch_store().get::<SketchState>();

    let draw = sketch_state.get_draw(DrawId(id));

    draw.inner.background().hsv(h, s, v);
}

#[op2(fast)]
fn op_draw_translate(state: &mut OpState, id: u32, x: f32, y: f32) -> u32 {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    let draw = sketch_state.get_draw(DrawId(id));

    let draw = draw.inner.x_y(x, y);

    sketch_state.store_draw(draw.into()).0
}

#[op2(fast)]
fn op_draw_rotate(state: &mut OpState, id: u32, radians: f32) -> u32 {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    let draw = sketch_state.get_draw(DrawId(id));

    let draw = draw.inner.rotate(radians);

    sketch_state.store_draw(draw.into()).0
}

#[op2(fast)]
fn op_draw_scale(state: &mut OpState, id: u32, s: f32) -> u32 {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    let draw = sketch_state.get_draw(DrawId(id));

    let draw = draw.inner.scale(s);

    sketch_state.store_draw(draw.into()).0
}

#[op2(fast)]
fn op_draw_scale_x(state: &mut OpState, id: u32, s: f32) -> u32 {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    let draw = sketch_state.get_draw(DrawId(id));

    let draw = draw.inner.scale_x(s);

    sketch_state.store_draw(draw.into()).0
}

#[op2(fast)]
fn op_draw_scale_y(state: &mut OpState, id: u32, s: f32) -> u32 {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    let draw = sketch_state.get_draw(DrawId(id));

    let draw = draw.inner.scale_y(s);

    sketch_state.store_draw(draw.into()).0
}

#[op2(fast)]
fn op_draw_width(state: &OpState) -> u32 {
    let sketch_state = state.sketch_store().get::<SketchState>();

    sketch_state.width
}

#[op2(fast)]
fn op_draw_height(state: &OpState) -> u32 {
    let sketch_state = state.sketch_store().get::<SketchState>();

    sketch_state.height
}
