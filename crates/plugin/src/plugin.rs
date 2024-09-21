use deno_core::Extension;
pub trait Plugin: Send {
    fn extension(&self) -> Extension;
}
