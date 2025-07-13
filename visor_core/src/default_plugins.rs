use visor_engine::plugin::Plugin;

use visor_plugin_draw::DrawPlugin;
use visor_plugin_log::LogPlugin;
use visor_plugin_math::MathPlugin;
use visor_plugin_midi::MidiPlugin;
use visor_plugin_state::StatePlugin;
use visor_plugin_time::TimePlugin;

pub fn default_plugins() -> Vec<Box<dyn Plugin>> {
    vec![
        Box::new(TimePlugin),
        Box::new(LogPlugin),
        Box::new(MathPlugin),
        Box::new(DrawPlugin),
        Box::new(StatePlugin),
        Box::new(MidiPlugin),
    ]
}
