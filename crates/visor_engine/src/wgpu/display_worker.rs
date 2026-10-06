use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use tokio::sync::mpsc;

use crate::WgpuHandle;

pub(crate) enum WgpuDisplayWorkerTask {
    Resize { width: u32, height: u32 },
    SetSourceTexture(Option<nannou::wgpu::TextureView>),
}

pub(crate) struct WgpuDisplayWorker {
    surface: nannou::wgpu::Surface<'static>,
    surface_config: nannou::wgpu::SurfaceConfiguration,
    surface_format: nannou::wgpu::TextureFormat,
    source_texture_reshaper: Option<nannou::wgpu::TextureReshaper>,
    task_receiver: mpsc::Receiver<WgpuDisplayWorkerTask>,
    frame_counter: std::sync::Arc<AtomicU64>,
    wgpu_handle: Arc<WgpuHandle>,
}

impl WgpuDisplayWorker {
    pub(crate) fn new(
        surface: nannou::wgpu::Surface<'static>,
        surface_config: nannou::wgpu::SurfaceConfiguration,
        surface_format: nannou::wgpu::TextureFormat,
        task_receiver: mpsc::Receiver<WgpuDisplayWorkerTask>,
        frame_counter: std::sync::Arc<AtomicU64>,
        wgpu_handle: Arc<WgpuHandle>,
    ) -> Self {
        Self {
            surface,
            surface_config,
            surface_format,
            source_texture_reshaper: None,
            task_receiver,
            frame_counter,
            wgpu_handle,
        }
    }

    pub(crate) fn run(&mut self) {
        let mut last_presented = u64::MAX;

        loop {
            while let Ok(task) = self.task_receiver.try_recv() {
                match task {
                    WgpuDisplayWorkerTask::Resize { width, height } => self.resize(width, height),
                    WgpuDisplayWorkerTask::SetSourceTexture(texture_view) => {
                        self.set_source_texture(texture_view)
                    }
                }
            }

            let counter = self.frame_counter.load(Ordering::Acquire);
            if counter == last_presented {
                std::thread::sleep(std::time::Duration::from_millis(1));
                continue;
            }

            match self.render() {
                Ok(()) => {}
                Err(nannou::wgpu::SurfaceError::Lost) => {
                    log::error!("Surface error: display surface texture lost!");

                    panic!("Surface error: display surface texture lost!")

                    // TODO: this might need to be handled, but ideally window remains generic
                    // let size = self.window.inner_size();
                    // self.resize(size.width, size.height);
                }
                Err(nannou::wgpu::SurfaceError::OutOfMemory) => {
                    log::error!("Surface error: out of memory!");

                    panic!("Surface error: out of memory!")
                }
                Err(e) => {
                    log::error!("Surface error: {:?}", e);
                }
            }

            last_presented = counter;
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;

            self.surface
                .configure(&self.wgpu_handle.device, &self.surface_config);
        }
    }

    fn set_source_texture(&mut self, texture_view: Option<nannou::wgpu::TextureView>) {
        self.source_texture_reshaper = texture_view.map(|texture_view| {
            nannou::wgpu::TextureReshaper::new(
                &self.wgpu_handle.device,
                &texture_view,
                texture_view.info().sample_count,
                texture_view.sample_type(),
                1,
                self.surface_format,
            )
        });
    }

    fn render(&self) -> Result<(), nannou::wgpu::SurfaceError> {
        if let Some(source_texture_reshaper) = &self.source_texture_reshaper {
            let t = std::time::Instant::now();
            let surface_texture = self.surface.get_current_texture()?;
            let acquire = t.elapsed();

            let mut encoder = self.wgpu_handle.device.create_command_encoder(
                &nannou::wgpu::CommandEncoderDescriptor {
                    label: Some("Display surface texture render encoder"),
                },
            );

            let surface_texture_view = surface_texture
                .texture
                .create_view(&nannou::wgpu::TextureViewDescriptor::default());

            source_texture_reshaper.encode_render_pass(&surface_texture_view, &mut encoder);

            let t = std::time::Instant::now();
            self.wgpu_handle.queue.submit(Some(encoder.finish()));

            surface_texture.present();
            let submit_present = t.elapsed();

            if acquire.as_millis() > 20 || submit_present.as_millis() > 20 {
                eprintln!("~~~");

                if acquire.as_millis() > 20 {
                    eprintln!("slow: acquire {acquire:?}");
                }

                if submit_present.as_millis() > 20 {
                    eprintln!("slow: submit+present {submit_present:?}");
                }
            }
        };

        Ok(())
    }
}
