use std::{collections::HashMap, sync::Arc};

use nannou::wgpu::{Instance, TextureView};
use tao::window::Window;
use tokio::runtime::Runtime;
use uuid::Uuid;

use crate::display::Display;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct DisplayId(Uuid);

pub struct DisplayManager {
    runtime: Arc<Runtime>,
    displays: HashMap<DisplayId, Display>,
}

impl DisplayManager {
    pub fn new(runtime: Arc<Runtime>) -> Self {
        Self {
            runtime,
            displays: HashMap::new(),
        }
    }

    // TODO: handle window events

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

        self.displays.insert(id, display);

        id
    }

    pub fn focus_display(&mut self, id: DisplayId) {
        self.get_display(&id).focus();
    }

    pub fn remove_display(&mut self, id: DisplayId) {
        self.displays.remove(&id);
    }

    fn get_display(&self, id: &DisplayId) -> &Display {
        self.displays
            .get(id)
            .unwrap_or_else(|| panic!("Unexpected: could not find display with id {}", id.0))
    }

    pub fn set_display_fullscreen(&self, id: DisplayId, enabled: bool) {
        self.get_display(&id).set_fullscreen(enabled);
    }

    pub fn render(&mut self) {
        self.displays.values_mut().for_each(|display| {
            display.render();
        });
    }
}
