use std::{collections::HashMap, sync::Arc};

use nannou::wgpu::{Instance, TextureView};
use tao::{
    event::WindowEvent,
    window::{Window, WindowId},
};
use tokio::runtime::Runtime;
use uuid::Uuid;

use crate::display::Display;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct DisplayId(pub(crate) Uuid);

pub struct DisplayManager {
    runtime: Arc<Runtime>,
    displays: HashMap<DisplayId, Display>,
    display_id_map: HashMap<WindowId, DisplayId>,
}

impl DisplayManager {
    pub fn new(runtime: Arc<Runtime>) -> Self {
        Self {
            runtime,
            displays: HashMap::new(),
            display_id_map: HashMap::new(),
        }
    }

    pub fn handle_tao_window_event(&mut self, window_id: &WindowId, event: &WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                let display_id = self
                    .display_id_map
                    .get(window_id)
                    .expect(&format!(
                        "Unexpected: could not find display id for window with id {:?}",
                        window_id
                    ))
                    .clone();

                self.remove_display(&display_id);
            }

            WindowEvent::Destroyed => {
                if self.displays.is_empty() {
                    std::process::exit(0)
                }
            }

            WindowEvent::Resized(size) => {
                let display_id = self.display_id_map.get(window_id).expect(&format!(
                    "Unexpected: could not find display id for window with id {:?}",
                    window_id
                ));

                let display = self.displays.get_mut(display_id).expect(&format!(
                    "Unexpected: could not find display with id {}",
                    display_id.0.to_string()
                ));

                display.handle_resize(*size);
            }
            _ => {}
        }
    }

    pub fn add_display(
        &mut self,
        wgpu_instance: &Instance,
        window: Arc<Window>,
        texture_view: TextureView,
    ) -> DisplayId {
        let id = DisplayId(Uuid::new_v4());

        let display = self
            .runtime
            .block_on(async { Display::new(wgpu_instance, window, texture_view).await });

        self.display_id_map.insert(display.window_id(), id);
        self.displays.insert(id, display);

        id
    }

    pub fn focus_display(&mut self, id: &DisplayId) {
        self.get_display(id).focus();
    }

    pub fn remove_display(&mut self, id: &DisplayId) {
        self.display_id_map.retain(|_, v| v != id);
        self.displays.remove(id);
    }

    fn get_display(&self, id: &DisplayId) -> &Display {
        self.displays
            .get(id)
            .unwrap_or_else(|| panic!("Unexpected: could not find display with id {}", id.0))
    }

    pub fn set_display_fullscreen(&self, id: &DisplayId, enabled: bool) {
        self.get_display(id).set_fullscreen(enabled);
    }

    pub fn render(&mut self) {
        self.displays.values_mut().for_each(|display| {
            display.render();
        });
    }
}
