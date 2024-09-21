use deno_core::{error::AnyError, extension, op2, Extension, OpState};
use visor_draw::draw::Draw;
use visor_plugin::plugin::Plugin;

pub struct DrawPlugin;

extension!(
    draw_plugin,
    ops = [op_draw_rect],
    esm_entry_point = "visor:draw",
    esm = [
        dir "src",
        "visor:draw" = "visor-draw.js",
    ]
);

impl Plugin for DrawPlugin {
    fn extension(&self) -> Extension {
        draw_plugin::init_ops_and_esm()
    }
}

#[op2(fast)]
pub fn op_draw_rect(state: &mut OpState, x: f32, y: f32, d: f32) -> Result<(), AnyError> {
    let draw = state.borrow::<Draw>();

    draw.inner
        .rect()
        .x_y(x, y)
        .w_h(d, d)
        .color(nannou::prelude::RED);

    Ok(())
}
