use std::sync::Arc;

use engine::engine::{EngineBuilder, WindowCreator};
use tao::event_loop::EventLoopWindowTarget;

struct EventLoopWindowCreator {
    event_loop_window_target: EventLoopWindowTarget<()>,
}

impl WindowCreator for EventLoopWindowCreator {
    fn create_window(
        &self,
        window_builder: tao::window::WindowBuilder,
    ) -> Arc<tao::window::Window> {
        let window = window_builder
            .build(&self.event_loop_window_target)
            .expect("Unexpected: could not build window");

        Arc::new(window)
    }
}

fn main() {
    let event_loop = tao::event_loop::EventLoop::new();

    let event_loop_window_creator = EventLoopWindowCreator {
        event_loop_window_target: event_loop.clone(),
    };

    let mut engine = EngineBuilder::default()
        .with_window_creator(Box::new(event_loop_window_creator))
        .build();

    let render_texture_id = engine.create_render_texture(600, 400);
    engine.create_display("Display".into(), 600, 400, &render_texture_id);

    event_loop.run(move |event, _event_loop, control_flow| {
        *control_flow = tao::event_loop::ControlFlow::Wait;

        match event {
            tao::event::Event::WindowEvent {
                event: tao::event::WindowEvent::CloseRequested,
                window_id: _,
                ..
            } => {
                std::process::exit(0);
            }

            tao::event::Event::WindowEvent {
                event: tao::event::WindowEvent::Destroyed,
                window_id: _,
                ..
            } => {
                *control_flow = tao::event_loop::ControlFlow::Exit;
            }

            tao::event::Event::MainEventsCleared => {
                engine.update();
            }
            _ => (),
        }
    });
}
