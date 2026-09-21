use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::RwLock,
};

use anyhow::{Result, anyhow};
use bevy_math::{
    Vec2,
    cubic_splines::{CubicCardinalSpline, CubicGenerator},
};
use deno_core::{Extension, OpState, extension, op2};
use nannou::draw::Drawing;
use tokio::sync::mpsc;
use visor_engine::{AccessSketchStore, Draw, Engine, Plugin, SketchId, SketchStore, Store};

use crate::ellipse::*;
use crate::fullscreen_shader::*;
use crate::path::*;
use crate::polygon::*;
use crate::polyline::*;
use crate::quad::*;
use crate::rect::*;
use crate::spline::*;

pub struct DrawPlugin;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct DrawId(pub(crate) u32);
type DrawMap = HashMap<DrawId, Draw>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ShapeId(pub(crate) u32);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct FullscreenShaderId(pub(crate) u32);

impl FullscreenShaderId {
    fn new(path: &str) -> Self {
        let mut hasher = std::hash::DefaultHasher::new();

        path.hash(&mut hasher);

        FullscreenShaderId(hasher.finish() as u32)
    }
}

pub(crate) struct SketchState {
    draw_map: DrawMap,
    next_draw_id: DrawId,
    pub(crate) next_shape_id: ShapeId,
    shape_order: Vec<(ShapeId, ShapeType)>,
    pub(crate) ellipse_command_map: EllipseCommandMap,
    pub(crate) rect_command_map: RectCommandMap,
    pub(crate) quad_command_map: QuadCommandMap,
    pub(crate) polygon_command_map: PolygonCommandMap,
    pub(crate) polyline_command_map: PolylineCommandMap,
    pub(crate) spline_command_map: SplineCommandMap,
    pub(crate) path_command_map: PathCommandMap,
    pub(crate) fullscreen_shader_command_map: FullscreenShaderCommandMap,
    width: u32,
    height: u32,
    fullscreen_shader_map: HashMap<FullscreenShaderId, FullscreenShader>,
    pub(crate) fullscreen_shader_event_sender: mpsc::Sender<FullscreenShaderEvent>,
    fullscreen_shader_event_receiver: mpsc::Receiver<FullscreenShaderEvent>,
}

enum ShapeType {
    Ellipse,
    Rect,
    Quad,
    Polygon,
    Polyline,
    Spline,
    Path,
    FullscreenShader,
}

type SketchSizeState = HashMap<SketchId, [u32; 2]>;

fn get_draw(sketch_store: &SketchStore, id: DrawId) -> &Draw {
    if id.0 == 0 {
        return sketch_store.get();
    }

    let sketch_state = sketch_store.get::<SketchState>();

    if let Some(draw) = sketch_state.draw_map.get(&id) {
        return draw;
    }

    // Return base draw if the given draw ID is invalid
    sketch_store.get()
}

impl SketchState {
    fn store_draw(&mut self, draw: Draw) -> DrawId {
        self.next_draw_id.0 += 1;

        self.draw_map.insert(self.next_draw_id, draw);

        self.next_draw_id
    }

    pub(crate) fn start_drawing_ellipse(&mut self, draw_id: DrawId) -> ShapeId {
        self.next_shape_id.0 += 1;

        let draw_id = self.clamp_draw_id(draw_id);

        self.ellipse_command_map
            .insert(self.next_shape_id, (draw_id, Vec::new()));

        self.shape_order
            .push((self.next_shape_id, ShapeType::Ellipse));

        self.next_shape_id
    }

    pub(crate) fn store_ellipse_command(&mut self, id: ShapeId, command: EllipseCommand) {
        self.ellipse_command_map
            .get_mut(&id)
            .expect("Unexpected: could not find shape commands for given id")
            .1
            .push(command);
    }

    pub(crate) fn start_drawing_rect(&mut self, draw_id: DrawId) -> ShapeId {
        self.next_shape_id.0 += 1;

        let draw_id = self.clamp_draw_id(draw_id);

        self.rect_command_map
            .insert(self.next_shape_id, (draw_id, Vec::new()));

        self.shape_order.push((self.next_shape_id, ShapeType::Rect));

        self.next_shape_id
    }

    pub(crate) fn store_rect_command(&mut self, id: ShapeId, command: RectCommand) {
        self.rect_command_map
            .get_mut(&id)
            .expect("Unexpected: could not find shape commands for given id")
            .1
            .push(command);
    }

    pub(crate) fn start_drawing_quad(&mut self, draw_id: DrawId) -> ShapeId {
        self.next_shape_id.0 += 1;

        let draw_id = self.clamp_draw_id(draw_id);

        self.quad_command_map
            .insert(self.next_shape_id, (draw_id, Vec::new()));

        self.shape_order.push((self.next_shape_id, ShapeType::Quad));

        self.next_shape_id
    }

    pub(crate) fn store_quad_command(&mut self, id: ShapeId, command: QuadCommand) {
        self.quad_command_map
            .get_mut(&id)
            .expect("Unexpected: could not find shape commands for given id")
            .1
            .push(command);
    }

    pub(crate) fn start_drawing_polygon(&mut self, draw_id: DrawId) -> ShapeId {
        self.next_shape_id.0 += 1;

        let draw_id = self.clamp_draw_id(draw_id);

        self.polygon_command_map
            .insert(self.next_shape_id, (draw_id, Vec::new()));

        self.shape_order
            .push((self.next_shape_id, ShapeType::Polygon));

        self.next_shape_id
    }

    pub(crate) fn store_polygon_command(&mut self, id: ShapeId, command: PolygonCommand) {
        self.polygon_command_map
            .get_mut(&id)
            .expect("Unexpected: could not find shape commands for given id")
            .1
            .push(command);
    }

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

    pub(crate) fn start_drawing_spline(&mut self, draw_id: DrawId) -> ShapeId {
        self.next_shape_id.0 += 1;

        let draw_id = self.clamp_draw_id(draw_id);

        self.spline_command_map
            .insert(self.next_shape_id, (draw_id, Vec::new()));

        self.shape_order
            .push((self.next_shape_id, ShapeType::Spline));

        self.next_shape_id
    }

    pub(crate) fn store_spline_command(&mut self, id: ShapeId, command: SplineCommand) {
        self.spline_command_map
            .get_mut(&id)
            .expect("Unexpected: could not find shape commands for given id")
            .1
            .push(command);
    }

    pub(crate) fn start_drawing_path(&mut self, draw_id: DrawId) -> ShapeId {
        self.next_shape_id.0 += 1;

        let draw_id = self.clamp_draw_id(draw_id);

        self.path_command_map
            .insert(self.next_shape_id, (draw_id, Vec::new()));

        self.shape_order.push((self.next_shape_id, ShapeType::Path));

        self.next_shape_id
    }

    pub(crate) fn store_path_command(&mut self, id: ShapeId, command: PathCommand) {
        self.path_command_map
            .get_mut(&id)
            .expect("Unexpected: could not find shape commands for given id")
            .1
            .push(command);
    }

    pub(crate) fn start_drawing_fullscreen_shader(
        &mut self,
        draw_id: DrawId,
        shader_id: FullscreenShaderId,
    ) {
        let draw_id = self.clamp_draw_id(draw_id);

        self.fullscreen_shader_command_map
            .insert(self.next_shape_id, (draw_id, shader_id));

        if let FullscreenShader::Loaded(shader) = self
            .fullscreen_shader_map
            .get_mut(&shader_id)
            .expect("Unexpected: could not find fullscreen shader for given id")
        {
            shader.is_being_drawn = true;
        }

        self.shape_order
            .push((self.next_shape_id, ShapeType::FullscreenShader));
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

    fn apply_shape_commands(&self, sketch_store: &SketchStore) {
        for (shape_id, shape_type) in self.shape_order.iter() {
            match shape_type {
                ShapeType::Ellipse => {
                    let (draw_id, commands) = self
                        .ellipse_command_map
                        .get(shape_id)
                        .expect("Unexpected: could not find ellipse commands for shape id");
                    let draw = get_draw(sketch_store, *draw_id);

                    let mut ellipse = draw.inner.ellipse();

                    for command in commands {
                        ellipse = command.apply(ellipse);
                    }
                }
                ShapeType::Rect => {
                    let (draw_id, commands) = self
                        .rect_command_map
                        .get(shape_id)
                        .expect("Unexpected: could not find rect commands for shape id");
                    let draw = get_draw(sketch_store, *draw_id);

                    let mut rect = draw.inner.rect();

                    for command in commands {
                        rect = command.apply(rect);
                    }
                }
                ShapeType::Quad => {
                    let (draw_id, commands) = self
                        .quad_command_map
                        .get(shape_id)
                        .expect("Unexpected: could not find quad commands for shape id");
                    let draw = get_draw(sketch_store, *draw_id);

                    let mut quad = draw.inner.quad();

                    for command in commands {
                        quad = command.apply(quad);
                    }
                }
                ShapeType::Polygon => {
                    let (draw_id, commands) = self
                        .polygon_command_map
                        .get(shape_id)
                        .expect("Unexpected: could not find polygon commands for shape id");
                    let draw = get_draw(sketch_store, *draw_id);

                    let mut polygon = draw.inner.polygon();

                    let mut points: Vec<(f32, f32)> = vec![];

                    for command in commands {
                        match command {
                            PolygonCommand::Point { x, y } => points.push((*x, *y)),
                            _ => polygon = command.apply(polygon),
                        }
                    }

                    if points.is_empty() {
                        continue;
                    }

                    let _polygon = polygon.points(points);
                }
                ShapeType::Polyline => {
                    let (draw_id, commands) = self
                        .polyline_command_map
                        .get(shape_id)
                        .expect("Unexpected: could not find polyline commands for shape id");
                    let draw = get_draw(sketch_store, *draw_id);

                    let mut polyline = draw.inner.polyline();

                    let mut points: Vec<(f32, f32)> = vec![];

                    for command in commands {
                        match command {
                            PolylineCommand::Point { x, y } => points.push((*x, *y)),
                            _ => polyline = command.apply(polyline),
                        }
                    }

                    if points.is_empty() {
                        continue;
                    }

                    let _polyline = polyline.points(points);
                }
                ShapeType::Spline => {
                    let (draw_id, commands) = self
                        .spline_command_map
                        .get(shape_id)
                        .expect("Unexpected: could not find spline commands for shape id");
                    let draw = get_draw(sketch_store, *draw_id);

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

                    if points.is_empty() {
                        continue;
                    }

                    let resolution = resolution.unwrap_or_else(|| points.len() * 20);

                    let curve = CubicCardinalSpline::new(tension, points).to_curve();

                    let points: Vec<_> = curve
                        .iter_positions(resolution)
                        .map(|point| (point.x, point.y))
                        .collect();

                    let _spline = spline.points(points);
                }
                ShapeType::Path => {
                    let (draw_id, commands) = self
                        .path_command_map
                        .get(shape_id)
                        .expect("Unexpected: could not find path commands for shape id");
                    let draw = get_draw(sketch_store, *draw_id);

                    let mut path = draw.inner.path().fill();

                    let mut points: Vec<Vec2> = vec![];
                    let mut tension: f32 = 0.0;
                    let mut resolution: Option<usize> = None;

                    for command in commands {
                        match command {
                            PathCommand::Point { x, y } => points.push((*x, *y).into()),
                            PathCommand::Tension { t } => tension = *t,
                            PathCommand::Resolution { n } => resolution = Some(*n as usize),
                            _ => path = command.apply(path),
                        }
                    }

                    if points.is_empty() {
                        continue;
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

                    let _path = path.points(points);
                }
                ShapeType::FullscreenShader => {
                    let (draw_id, shader_id) =
                        self.fullscreen_shader_command_map.get(shape_id).expect(
                            "Unexpected: could not find fullscreen shader commands for shape id",
                        );

                    let draw = get_draw(sketch_store, *draw_id);

                    let shader = self.fullscreen_shader_map.get(shader_id);

                    if let FullscreenShader::Loaded(shader) =
                        shader.expect("Unexpected: could not find fullscreen shader")
                    {
                        draw.inner
                            .texture(&shader.texture_view)
                            .width(self.width as f32)
                            .height(self.height as f32);
                    }
                }
            }
        }
    }

    fn clear_shape_commands(&mut self) {
        self.ellipse_command_map.clear();
        self.rect_command_map.clear();
        self.quad_command_map.clear();
        self.polygon_command_map.clear();
        self.polyline_command_map.clear();
        self.spline_command_map.clear();
        self.path_command_map.clear();
        self.fullscreen_shader_command_map.clear();
    }

    fn reset(&mut self) {
        self.draw_map.clear();

        self.next_draw_id.0 = 0;
        self.next_shape_id.0 = 0;
        self.shape_order.clear();

        for shader in self.fullscreen_shader_map.values_mut() {
            if let FullscreenShader::Loaded(shader) = shader {
                shader.is_being_drawn = false;
            }
        }
    }

    pub(crate) fn load_fullscreen_shader(&mut self, path: String) -> Result<FullscreenShaderId> {
        let shader_id = FullscreenShaderId::new(&path);

        let source = std::fs::read_to_string(&path)
            .map_err(|_| anyhow!("Could not load shader at path {}", path))?;

        naga::front::wgsl::parse_str(&source).map_err(|error| {
            anyhow!(
                "Could not load shader due to invalid WGSL syntax: {}",
                error.to_string()
            )
        })?;

        self.fullscreen_shader_map
            .insert(shader_id, FullscreenShader::Unloaded);

        self.fullscreen_shader_event_sender
            .try_send(FullscreenShaderEvent::Load {
                id: shader_id,
                source,
                width: self.width,
                height: self.height,
            })
            .expect("Unexpected: could not send shader event");

        Ok(shader_id)
    }

    pub(crate) fn set_fullscreen_shader_uniform(
        &mut self,
        id: FullscreenShaderId,
        key: String,
        value: f32,
    ) -> Result<()> {
        // TODO: assign uniform value to unloaded shader anyway so it can be applied immediately after loaded
        if let FullscreenShader::Loaded(shader) = self
            .fullscreen_shader_map
            .get_mut(&id)
            .expect("Unexpected: could not find shader for given id")
        {
            shader.set_uniform(&key, value)?;
        }

        Ok(())
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
        op_draw_fullscreen_shader,
        op_draw_translate,
        op_draw_rotate,
        op_draw_scale,
        op_draw_scale_x,
        op_draw_scale_y,
        op_draw_width,
        op_draw_height,
        op_draw_fullscreen_shader_load,
        op_draw_fullscreen_shader_set_uniform,
    ],
    esm_entry_point = "ext:visor_plugin_draw/src/draw-plugin.ts",
    esm = [
        "src/color.ts",
        "src/draw.ts",
        "src/ellipse.ts",
        "src/fullscreen-shader.ts",
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
        _sketch_id: &SketchId,
        _engine: &mut Engine,
        _store: &Store,
        sketch_store: &mut SketchStore,
    ) {
        let (shader_event_sender, shader_event_receiver) = mpsc::channel::<_>(64);

        sketch_store.set(SketchState {
            draw_map: Default::default(),
            next_draw_id: DrawId(0),
            next_shape_id: ShapeId(0),
            shape_order: Default::default(),
            ellipse_command_map: Default::default(),
            rect_command_map: Default::default(),
            quad_command_map: Default::default(),
            polygon_command_map: Default::default(),
            polyline_command_map: Default::default(),
            spline_command_map: Default::default(),
            path_command_map: Default::default(),
            fullscreen_shader_command_map: Default::default(),
            width: 0,
            height: 0,
            fullscreen_shader_map: Default::default(),
            fullscreen_shader_event_sender: shader_event_sender,
            fullscreen_shader_event_receiver: shader_event_receiver,
        });
    }

    fn before_engine_update(&self, engine: &mut Engine, store: &Store) {
        let mut sketch_size_state = store
            .get::<RwLock<SketchSizeState>>()
            .write()
            .expect("Unexpected: could not acquire write lock for sketch size state");

        let mut resized_sketches: HashMap<SketchId, [u32; 2]> = HashMap::new();

        for (sketch_id, sketch) in engine.sketches().iter() {
            let render_texture_id = sketch.get_target_render_texture_id();

            let render_texture = render_texture_id.map(|id| {
                engine
                    .render_textures()
                    .get(id)
                    .expect("Unexpected: could not find render texture")
            });

            let size = render_texture
                .map(|render_texture| render_texture.texture_view().size())
                .unwrap_or([0, 0]);

            if let Some(current_size) = sketch_size_state.get(sketch_id) {
                if *current_size != size {
                    resized_sketches.insert(*sketch_id, size);
                }
            }

            sketch_size_state.insert(*sketch_id, size);
        }

        for (sketch_id, [width, height]) in resized_sketches {
            engine
                .sketches_mut()
                .get_mut(&sketch_id)
                .expect("Unexpected: could not find sketch")
                .sketch_store_mut()
                .get_mut::<SketchState>()
                .fullscreen_shader_map
                .values_mut()
                .for_each(|shader| {
                    if let FullscreenShader::Loaded(shader) = shader {
                        shader.resize(width, height);
                    }
                });
        }
    }

    fn before_sketch_update(
        &self,
        sketch_id: &SketchId,
        store: &Store,
        sketch_store: &mut SketchStore,
    ) {
        sketch_store.get::<Draw>().inner.reset();

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
        let sketch_state = sketch_store.get::<SketchState>();
        sketch_state.apply_shape_commands(sketch_store);

        let sketch_state = sketch_store.get_mut::<SketchState>();
        sketch_state.clear_shape_commands();
    }

    fn before_engine_render(
        &self,
        engine: &mut Engine,
        _store: &Store,
        encoder: &mut nannou::wgpu::CommandEncoder,
    ) {
        // Fetch shader events
        // TODO: this can be collapsed after wgpu_handle is passed as parameter to plugin callbacks
        let mut shader_events: HashMap<SketchId, Vec<FullscreenShaderEvent>> = HashMap::new();

        for sketch in engine.sketches_mut().values_mut() {
            let sketch_state = sketch.sketch_store_mut().get_mut::<SketchState>();

            if sketch_state.fullscreen_shader_event_receiver.is_empty() {
                continue;
            }

            let mut events = Vec::new();
            while let Ok(event) = sketch_state.fullscreen_shader_event_receiver.try_recv() {
                events.push(event);
            }

            shader_events.insert(*sketch.id(), events);
        }

        // Process shader events
        for (sketch_id, events) in shader_events {
            for event in events {
                match event {
                    FullscreenShaderEvent::Load {
                        id,
                        source,
                        width,
                        height,
                    } => {
                        let shader = FullscreenShaderState::new(source, engine, width, height);

                        engine
                            .sketches_mut()
                            .get_mut(&sketch_id)
                            .expect("Unexpected: could not find sketch")
                            .sketch_store_mut()
                            .get_mut::<SketchState>()
                            .fullscreen_shader_map
                            .insert(id, FullscreenShader::Loaded(shader));
                    }
                }
            }
        }

        // Render shaders
        for sketch in engine.sketches_mut().values_mut() {
            let sketch_state = sketch.sketch_store_mut().get_mut::<SketchState>();

            for shader in sketch_state.fullscreen_shader_map.values_mut() {
                if let FullscreenShader::Loaded(shader) = shader {
                    if !shader.is_being_drawn {
                        continue;
                    }

                    shader.render(encoder);
                }
            }
        }
    }
}

#[op2(fast)]
fn op_draw_background_rgb(state: &OpState, id: u32, r: f32, g: f32, b: f32) {
    let draw = get_draw(state.sketch_store(), DrawId(id));

    draw.inner.background().rgb(r, g, b);
}

#[op2(fast)]
fn op_draw_background_hsv(state: &OpState, id: u32, h: f32, s: f32, v: f32) {
    let draw = get_draw(state.sketch_store(), DrawId(id));

    draw.inner.background().hsv(h, s, v);
}

#[op2(fast)]
fn op_draw_translate(state: &mut OpState, id: u32, x: f32, y: f32) -> u32 {
    let draw = get_draw(state.sketch_store(), DrawId(id));

    let draw = draw.inner.x_y(x, y);

    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();
    sketch_state.store_draw(draw.into()).0
}

#[op2(fast)]
fn op_draw_rotate(state: &mut OpState, id: u32, radians: f32) -> u32 {
    let draw = get_draw(state.sketch_store(), DrawId(id));

    let draw = draw.inner.rotate(radians);

    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();
    sketch_state.store_draw(draw.into()).0
}

#[op2(fast)]
fn op_draw_scale(state: &mut OpState, id: u32, s: f32) -> u32 {
    let draw = get_draw(state.sketch_store(), DrawId(id));

    let draw = draw.inner.scale(s);

    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();
    sketch_state.store_draw(draw.into()).0
}

#[op2(fast)]
fn op_draw_scale_x(state: &mut OpState, id: u32, s: f32) -> u32 {
    let draw = get_draw(state.sketch_store(), DrawId(id));

    let draw = draw.inner.scale_x(s);

    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();
    sketch_state.store_draw(draw.into()).0
}

#[op2(fast)]
fn op_draw_scale_y(state: &mut OpState, id: u32, s: f32) -> u32 {
    let draw = get_draw(state.sketch_store(), DrawId(id));

    let draw = draw.inner.scale_y(s);

    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();
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
