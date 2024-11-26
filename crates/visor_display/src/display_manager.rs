use std::{collections::HashMap, sync::Arc};

use nannou::wgpu::{Instance, TextureView};
use tao::window::{Window, WindowId};
use tokio::runtime::Handle;

use crate::display::{Display, DisplayId};

pub struct DisplayManager {
    runtime_handle: Handle,
    displays: HashMap<DisplayId, Display>,
    display_id_map: HashMap<WindowId, DisplayId>,
}

impl DisplayManager {
    pub fn new(runtime_handle: Handle) -> Self {
        Self {
            runtime_handle,
            displays: HashMap::new(),
            display_id_map: HashMap::new(),
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

    pub fn add_display(
        &mut self,
        id: DisplayId,
        wgpu_instance: &Instance,
        window: Arc<Window>,
    ) -> &Display {
        // TODO: should display id just be a window id?
        let window_id = window.id();

        let display = self
            .runtime_handle
            .block_on(async { Display::new(id, wgpu_instance, window).await });

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

    pub fn set_display_source_texture(
        &mut self,
        id: &DisplayId,
        texture_view: Option<&TextureView>,
    ) {
        self.displays
            .get_mut(id)
            .unwrap_or_else(|| panic!("Unexpected: could not find display with id {}", id.0))
            .set_source_texture(texture_view);
    }

    pub fn render(&mut self) {
        self.displays.values_mut().for_each(|display| {
            display.render();
        });
    }
}
