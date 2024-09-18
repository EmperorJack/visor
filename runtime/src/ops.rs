use deno_core::{error::AnyError, op2, OpDecl, OpState};
use draw::draw::Draw;

pub const OPS: [OpDecl; 1] = [op_draw_rect()];

#[op2(fast)]
fn op_draw_rect(state: &mut OpState, x: f32, y: f32, d: f32) -> Result<(), AnyError> {
    let draw = state.borrow::<Draw>();

    draw.inner
        .rect()
        .x_y(x, y)
        .w_h(d, d)
        .color(nannou::prelude::RED);

    Ok(())
}
