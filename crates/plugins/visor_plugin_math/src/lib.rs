use deno_core::{Extension, OpState, extension, op2};
use nannou::noise::{NoiseFn, Perlin};
use visor_engine::{AccessSketchStore, Engine, Plugin, SketchId, SketchStore, Store};

pub struct MathPlugin;

struct SketchState {
    noise: Perlin,
}

extension!(
    visor_plugin_math,
    ops = [op_draw_noise],
    esm_entry_point = "ext:visor_plugin_math/src/visor-plugin-math.ts",
    esm = ["src/ops.ts", "src/visor-plugin-math.ts"]
);

impl Plugin for MathPlugin {
    fn extension(&self) -> Extension {
        visor_plugin_math::init()
    }

    fn typescript_declaration(&self) -> Option<String> {
        Some(include_str!("visor-plugin-math.d.ts").into())
    }

    fn build_sketch(
        &self,
        _sketch_id: &SketchId,
        _engine: &mut Engine,
        _store: &Store,
        sketch_store: &mut SketchStore,
    ) {
        sketch_store.set(SketchState {
            noise: Default::default(),
        });
    }
}

#[op2(fast)]
fn op_draw_noise(state: &mut OpState, x: f32, y: f32, z: f32) -> f32 {
    let sketch_state = state.sketch_store_mut().get_mut::<SketchState>();

    sketch_state.noise.get([x as f64, y as f64, z as f64]) as f32
}
