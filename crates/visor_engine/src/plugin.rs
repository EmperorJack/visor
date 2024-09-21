use visor_runtime::Extension;

pub trait Plugin: Send {
    fn extension(&self) -> Extension;
}
