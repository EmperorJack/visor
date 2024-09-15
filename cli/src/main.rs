use std::sync::Arc;

use engine::engine::EngineBuilder;

fn main() {
    let mut engine = EngineBuilder::default().build();

    let event_loop = tao::event_loop::EventLoop::new();

    let create_window_callback = Box::new(|window_builder: tao::window::WindowBuilder| {
        let window = window_builder
            .build(&event_loop)
            .expect("Unexpected: could not build window");

        Arc::new(window)
    });

    let render_texture_id = engine.create_render_texture(600, 400);
    engine.create_display(
        "Display".into(),
        600,
        400,
        &render_texture_id,
        create_window_callback,
    );

    event_loop.run(move |event, _, control_flow| {
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
