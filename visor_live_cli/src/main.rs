use std::{
    path::PathBuf,
    sync::{mpsc, Arc},
};

use clap::Parser;
use notify::{RecommendedWatcher, Watcher};
use tao::{event::Event, event_loop::EventLoopWindowTarget};
use visor_engine::engine::{EngineBuilder, WindowCreator};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    sketch_file_path: PathBuf,

    #[arg(short, long)]
    watch: bool,
}

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
    let args = Args::parse();

    let watcher_state = if args.watch {
        let (watcher_event_sender, watcher_event_receiver) =
            mpsc::channel::<Result<notify::Event, notify::Error>>();

        let mut watcher = RecommendedWatcher::new(
            move |event_result| {
                watcher_event_sender
                    .send(event_result)
                    .expect("Unexpected: could not send watch event to channel");
            },
            notify::Config::default(),
        )
        .expect("Unexpected: could not setup file watcher");

        watcher
            .watch(&args.sketch_file_path, notify::RecursiveMode::NonRecursive)
            .expect("Unexpected: could not watch sketch file path");

        Some((watcher, watcher_event_receiver))
    } else {
        None
    };

    let event_loop = tao::event_loop::EventLoop::new();

    let event_loop_window_creator = EventLoopWindowCreator {
        event_loop_window_target: event_loop.clone(),
    };

    let mut engine = EngineBuilder::default()
        .with_window_creator(Box::new(event_loop_window_creator))
        .build();

    let sketch_id = engine.create_sketch(args.sketch_file_path);

    let render_texture_id = engine.create_render_texture(600, 400);
    engine.set_sketch_target_render_texture_id(&sketch_id, Some(&render_texture_id));

    let display_id = engine.create_display("Display".into(), 600, 400);
    engine.set_display_source_texture(&display_id, Some(&render_texture_id));

    let engine_window_event_sender = engine.tao_window_event_sender();

    event_loop.run(move |event, _event_loop, control_flow| {
        *control_flow = tao::event_loop::ControlFlow::Poll;

        if let Some((_, watcher_receiver)) = &watcher_state {
            let mut sketch_file_updated = false;

            while let Ok(event_result) = watcher_receiver.try_recv() {
                let event = event_result.expect("Unexpected: watcher event error");

                match event.kind {
                    notify::EventKind::Create(_)
                    | notify::EventKind::Modify(_)
                    | notify::EventKind::Remove(_) => {
                        sketch_file_updated = true;
                    }
                    _ => {}
                }
            }

            if sketch_file_updated {
                println!("Detected file change, recompiling sketch...");
                engine.recompile_sketch(&sketch_id);
            }
        }

        match event {
            Event::WindowEvent {
                window_id, event, ..
            } => {
                if let Some(event) = event.to_static() {
                    engine_window_event_sender
                        .send((window_id, event))
                        .expect("Unexpected: could not send window event to engine");
                }
            }

            Event::MainEventsCleared => {
                engine.update();
            }
            _ => (),
        }
    });
}
