#[cfg(target_os = "macos")]
use objc::{msg_send, runtime::Object, sel, sel_impl};
use tokio::sync::mpsc;

use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use tao::rwh_06;

use crate::wgpu::{
    display_worker::{WgpuDisplayWorker, WgpuDisplayWorkerTask},
    handle::WgpuHandle,
};

#[derive(Debug)]
pub struct WgpuDisplay {
    worker_task_sender: mpsc::Sender<WgpuDisplayWorkerTask>,
    frame_counter: std::sync::Arc<AtomicU64>,
}

impl WgpuDisplay {
    pub async fn new<W>(wgpu: Arc<WgpuHandle>, window: W, width: u32, height: u32) -> Self
    where
        W: rwh_06::HasWindowHandle + rwh_06::HasDisplayHandle + Send + Sync + 'static,
    {
        #[cfg(target_os = "macos")]
        let ns_view: *mut Object = match window
            .window_handle()
            .expect("Unexpected: could not get macOS window handle")
            .as_raw()
        {
            rwh_06::RawWindowHandle::AppKit(handle) => handle.ns_view.as_ptr() as *mut Object,
            _ => panic!("Unexpected: expected a macOS AppKit window handle"),
        };

        let surface = wgpu
            .instance
            .create_surface(window)
            .expect("Unexpeced: could not create wgpu surface");

        let adapter = wgpu
            .instance
            .request_adapter(&nannou::wgpu::RequestAdapterOptions {
                power_preference: nannou::wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Unexpected: could not request wgpu adapter");

        let surface_capabilities = surface.get_capabilities(&adapter);

        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);

        let surface_config = nannou::wgpu::SurfaceConfiguration {
            usage: nannou::wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: nannou::wgpu::PresentMode::Fifo,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 1,
        };

        surface.configure(&wgpu.device, &surface_config);

        // Ensure macOS does not color manage the window
        #[cfg(target_os = "macos")]
        Self::set_layer_color_space_raw(ns_view);

        let frame_counter = std::sync::Arc::new(AtomicU64::new(0));

        let (worker_task_sender, worker_task_receiver) = mpsc::channel::<WgpuDisplayWorkerTask>(64);

        {
            let frame_counter = frame_counter.clone();

            std::thread::spawn(move || {
                WgpuDisplayWorker::new(
                    surface,
                    surface_config,
                    surface_format,
                    worker_task_receiver,
                    frame_counter,
                    wgpu,
                )
                .run();
            });
        }

        Self {
            worker_task_sender,
            frame_counter,
        }
    }

    #[cfg(target_os = "macos")]
    fn set_layer_color_space_raw(ns_view: *mut Object) {
        #![allow(unexpected_cfgs)]
        unsafe {
            let layer: *mut Object = msg_send![ns_view, layer];
            if layer.is_null() {
                return;
            }

            let _: () = msg_send![layer, setColorspace: std::ptr::null_mut::<std::ffi::c_void>()];
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.worker_task_sender
            .try_send(WgpuDisplayWorkerTask::Resize { width, height })
            .expect("Unexpected: could not send resize task to wgpu display worker");
    }

    pub fn set_source_texture(&mut self, texture_view: Option<&nannou::wgpu::TextureView>) {
        self.worker_task_sender
            .try_send(WgpuDisplayWorkerTask::SetSourceTexture(
                texture_view.cloned(),
            ))
            .expect("Unexpected: could not send set source texture task to wgpu display worker");
    }

    pub fn render(&self) {
        self.frame_counter.fetch_add(1, Ordering::Release);
    }
}
