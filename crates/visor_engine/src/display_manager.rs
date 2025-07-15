use std::collections::HashMap;

use tao::window::WindowId;

use crate::display::{Display, DisplayId};

pub(crate) struct DisplayManager {
    displays: HashMap<DisplayId, Display>,
    display_id_map: HashMap<WindowId, DisplayId>,
}

impl DisplayManager {
    pub(crate) fn new() -> Self {
        Self {
            displays: HashMap::new(),
            display_id_map: HashMap::new(),
        }
    }

    pub(crate) fn displays(&self) -> &HashMap<DisplayId, Display> {
        &self.displays
    }

    pub(crate) fn displays_mut(&mut self) -> &mut HashMap<DisplayId, Display> {
        &mut self.displays
    }

    pub(crate) fn display_id_for_window_id(&self, window_id: &WindowId) -> &DisplayId {
        self.display_id_map.get(window_id).unwrap_or_else(|| {
            panic!(
                "Unexpected: could not find display id for window with id {:?}",
                window_id
            )
        })
    }

    pub(crate) fn manage_display(&mut self, display: Display) -> &Display {
        let id = display.id();
        // TODO: should display id just be a window id?
        let window_id = display.window().id();

        self.display_id_map.insert(window_id, *id);
        self.displays.entry(*id).or_insert(display)
    }

    pub(crate) fn remove_display(&mut self, id: &DisplayId) {
        self.display_id_map.retain(|_, v| v != id);
        self.displays.remove(id);
    }

    pub(crate) fn render(&mut self) {
        self.displays.values_mut().for_each(|display| {
            display.render();
        });
    }
}
