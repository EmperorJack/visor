use nannou::wgpu::{Device, Instance, Queue};

#[derive(Debug)]
pub struct WgpuHandle {
    pub(crate) instance: Instance,
    pub device: Device,
    pub queue: Queue,
}
