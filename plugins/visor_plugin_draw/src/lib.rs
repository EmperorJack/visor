use std::collections::HashMap;

use bevy_math::{
    Vec2,
    cubic_splines::{CubicCardinalSpline, CubicGenerator},
};
use deno_core::{Extension, OpState, extension, op2};
use ellipse::*;
use nannou::{
    draw::Drawing,
    noise::{NoiseFn, Perlin},
};
use polyline::*;
use quad::*;
use rect::*;
use spline::*;
use visor_engine::{
    draw::Draw,
    engine::Engine,
    plugin::{AccessSketchStore, Plugin},
    sketch::SketchId,
    sketch_store::SketchStore,
    store::Store,
};

mod ellipse;
mod polyline;
mod quad;
mod rect;
mod spline;

pub struct DrawPlugin;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct DrawId(u32);
type DrawMap = HashMap<DrawId, Draw>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct ShapeId(u32);

struct SketchState {
    draw: Draw,
    draw_map: DrawMap,
    next_draw_id: DrawId,
    next_shape_id: ShapeId,
    ellipse_command_map: EllipseCommandMap,
    rect_command_map: RectCommandMap,
    quad_command_map: QuadCommandMap,
    polyline_command_map: PolylineCommandMap,
    spline_command_map: SplineCommandMap,
    noise: Perlin,
}

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

    fn start_drawing_ellipse(&mut self, draw_id: DrawId) -> ShapeId {
        self.next_shape_id.0 += 1;

        let draw_id = self.clamp_draw_id(draw_id);

        self.ellipse_command_map
            .insert(self.next_shape_id, (draw_id, Vec::new()));

        self.next_shape_id
    }

    fn store_ellipse_command(&mut self, id: ShapeId, command: EllipseCommand) {
        self.ellipse_command_map
            .get_mut(&id)
            .expect("Unexpected: could not find shape commands for given id")
            .1
            .push(command);
    }

    fn start_drawing_rect(&mut self, draw_id: DrawId) -> ShapeId {
        self.next_shape_id.0 += 1;

        let draw_id = self.clamp_draw_id(draw_id);

        self.rect_command_map
            .insert(self.next_shape_id, (draw_id, Vec::new()));

        self.next_shape_id
    }

    fn store_rect_command(&mut self, id: ShapeId, command: RectCommand) {
        self.rect_command_map
            .get_mut(&id)
            .expect("Unexpected: could not find shape commands for given id")
            .1
            .push(command);
    }

    fn start_drawing_quad(&mut self, draw_id: DrawId) -> ShapeId {
        self.next_shape_id.0 += 1;

        let draw_id = self.clamp_draw_id(draw_id);

        self.quad_command_map
            .insert(self.next_shape_id, (draw_id, Vec::new()));

        self.next_shape_id
    }

    fn store_quad_command(&mut self, id: ShapeId, command: QuadCommand) {
        self.quad_command_map
            .get_mut(&id)
            .expect("Unexpected: could not find shape commands for given id")
            .1
            .push(command);
    }

    fn start_drawing_polyline(&mut self, draw_id: DrawId) -> ShapeId {
        self.next_shape_id.0 += 1;

        let draw_id = self.clamp_draw_id(draw_id);

        self.polyline_command_map
            .insert(self.next_shape_id, (draw_id, Vec::new()));

        self.next_shape_id
    }

    fn store_polyline_command(&mut self, id: ShapeId, command: PolylineCommand) {
        self.polyline_command_map
            .get_mut(&id)
            .expect("Unexpected: could not find shape commands for given id")
            .1
            .push(command);
    }

    fn start_drawing_spline(&mut self, draw_id: DrawId) -> ShapeId {
        self.next_shape_id.0 += 1;

        let draw_id = self.clamp_draw_id(draw_id);

        self.spline_command_map
            .insert(self.next_shape_id, (draw_id, Vec::new()));

        self.next_shape_id
    }

    fn store_spline_command(&mut self, id: ShapeId, command: SplineCommand) {
        self.spline_command_map
            .get_mut(&id)
            .expect("Unexpected: could not find shape commands for given id")
            .1
            .push(command);
    }

    fn clamp_draw_id(&self, id: DrawId) -> DrawId {
        if id.0 == 0 {
            return id;
        }

        if id.0 <= self.next_draw_id.0 {
            return id;
        }

        DrawId(0)
    }

    fn apply_shape_commands(&mut self) {
        for (draw_id, commands) in self.ellipse_command_map.values() {
            let draw = self.get_draw(*draw_id);

            let mut ellipse = draw.inner.ellipse();

            for command in commands {
                ellipse = command.apply(ellipse);
            }
        }

        for (draw_id, commands) in self.rect_command_map.values() {
            let draw = self.get_draw(*draw_id);

            let mut rect = draw.inner.rect();

            for command in commands {
                rect = command.apply(rect);
            }
        }

        for (draw_id, commands) in self.quad_command_map.values() {
            let draw = self.get_draw(*draw_id);

            let mut quad = draw.inner.quad();

            for command in commands {
                quad = command.apply(quad);
            }
        }

        for (draw_id, commands) in self.polyline_command_map.values() {
            let draw = self.get_draw(*draw_id);

            let mut polyline = draw.inner.polyline();

            let mut points: Vec<(f32, f32)> = vec![];

            for command in commands {
                match command {
                    PolylineCommand::Point { x, y } => points.push((*x, *y)),
                    _ => polyline = command.apply(polyline),
                }
            }

            let _polyline = polyline.points(points);
        }

        for (draw_id, commands) in self.spline_command_map.values() {
            let draw = self.get_draw(*draw_id);

            let mut spline = draw.inner.polyline();

            let mut points: Vec<Vec2> = vec![];
            let mut tension: f32 = 0.5;
            let mut resolution: Option<usize> = None;

            for command in commands {
                match command {
                    SplineCommand::Point { x, y } => points.push((*x, *y).into()),
                    SplineCommand::Tension { t } => tension = *t,
                    SplineCommand::Resolution { n } => resolution = Some(*n as usize),
                    _ => spline = command.apply(spline),
                }
            }

            let resolution = resolution.unwrap_or_else(|| points.len() * 20);

            let curve = CubicCardinalSpline::new(tension, points).to_curve();

            let points = curve
                .iter_positions(resolution)
                .map(|point| (point.x, point.y));

            let _polyline = spline.points(points);
        }

        self.ellipse_command_map.clear();
        self.rect_command_map.clear();
        self.quad_command_map.clear();
        self.polyline_command_map.clear();
        self.spline_command_map.clear();
    }

    fn reset(&mut self) {
        self.draw.inner.reset();

        self.draw_map.clear();

        self.next_draw_id.0 = 0;
        self.next_shape_id.0 = 0;
    }
}

pub(crate) trait ShapeCommand<T> {
    fn apply<'a>(&self, drawing: Drawing<'a, T>) -> Drawing<'a, T>;
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
        op_draw_translate,
        op_draw_rotate,
        op_draw_scale,
        op_draw_noise,
    ],
    esm_entry_point = "ext:visor_plugin_draw/src/visor-plugin-draw.ts",
    esm = ["src/visor-plugin-draw.ts"]
);

impl Plugin for DrawPlugin {
    fn extension(&self) -> Extension {
        visor_plugin_draw::init()
    }

    fn typescript_declaration(&self) -> Option<String> {
        Some(include_str!("visor-plugin-draw.d.ts").into())
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
            ellipse_command_map: Default::default(),
            rect_command_map: Default::default(),
            quad_command_map: Default::default(),
            polyline_command_map: Default::default(),
            spline_command_map: Default::default(),
            noise: Default::default(),
        });
    }

    fn before_sketch_update(
        &self,
        _sketch_id: &SketchId,
        _store: &Store,
        sketch_store: &mut SketchStore,
    ) {
        sketch_store.get_mut::<SketchState>().reset();
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
fn op_draw_noise(state: &mut OpState, x: f32, y: f32, z: f32) -> f32 {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.noise.get([x as f64, y as f64, z as f64]) as f32
}
