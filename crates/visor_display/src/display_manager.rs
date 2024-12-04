use std::{collections::HashMap, sync::Arc};

use tao::window::{Window, WindowId};
use tokio::runtime::Handle;
use visor_wgpu::handle::WgpuHandle;

use crate::display::{Display, DisplayId};

pub struct DisplayManager {
    runtime_handle: Handle,
    displays: HashMap<DisplayId, Display>,
    display_id_map: HashMap<WindowId, DisplayId>,
    wgpu_handle: Arc<WgpuHandle>,
}

impl DisplayManager {
    pub fn new(runtime_handle: Handle, wgpu_handle: Arc<WgpuHandle>) -> Self {
        Self {
            runtime_handle,

            displays: HashMap::new(),
            display_id_map: HashMap::new(),
            wgpu_handle,
        }
    }

    pub fn displays(&self) -> &HashMap<DisplayId, Display> {
        &self.displays
    }

    pub fn displays_mut(&mut self) -> &mut HashMap<DisplayId, Display> {
        &mut self.displays
    }

    pub fn display_id_for_window_id(&self, window_id: &WindowId) -> &DisplayId {
        self.display_id_map.get(window_id).unwrap_or_else(|| {
            panic!(
                "Unexpected: could not find display id for window with id {:?}",
                window_id
            )
        })
    }

    pub fn add_display(&mut self, id: DisplayId, window: Arc<Window>) -> &Display {
        // TODO: should display id just be a window id?
        let window_id = window.id();

        let display = self
            .runtime_handle
            .block_on(async { Display::new(self.wgpu_handle.clone(), id, window).await });

        self.display_id_map.insert(window_id, id);
        self.displays.entry(id).or_insert(display)
    }

    pub fn manage_display(&mut self, display: Display) -> &Display {
        let id = display.id();
        let window_id = display.window().id();

        self.display_id_map.insert(window_id, *id);
        self.displays.entry(*id).or_insert(display)
    }

    pub fn remove_display(&mut self, id: &DisplayId) {
        self.display_id_map.retain(|_, v| v != id);
        self.displays.remove(id);
    }

    pub fn render(&mut self) {
        self.displays.values_mut().for_each(|display| {
            display.render();
        });
    }
}
