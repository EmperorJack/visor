use std::{
    path::PathBuf,
    sync::{Arc, mpsc},
};

use clap::Parser;
use notify::{RecommendedWatcher, Watcher};
use tao::{
    event::{Event, WindowEvent},
    window::WindowBuilder,
};
use visor_core::engine_builder::EngineBuilder;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    sketch_file_path: PathBuf,

    #[arg(short, long)]
    plugins: Option<Vec<PathBuf>>,

    #[arg(short, long)]
    watch: bool,
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

    let mut engine_builder = EngineBuilder::default();

    if let Some(plugins) = args.plugins {
        engine_builder = engine_builder.with_linked_plugins(plugins)
    }

    let mut engine = engine_builder.build();

    let sketch_id = *engine.create_sketch(args.sketch_file_path).id();

    let render_texture = engine.create_render_texture(600, 400);
    let render_texture_id = *render_texture.id();
    let render_texture_view = render_texture.texture_view();

    engine
        .sketches_mut()
        .get_mut(&sketch_id)
        .expect("Unexpected: could not find sketch")
        .set_target_render_texture_id(Some(&render_texture_id));

    let window = WindowBuilder::new()
        .with_title("Display")
        .with_inner_size(tao::dpi::PhysicalSize::new(600, 400))
        .build(&event_loop)
        .expect("Unexpected: could not build display window");

    let display_id = *engine.create_display(Arc::new(window)).id();
    engine
        .displays_mut()
        .get_mut(&display_id)
        .expect("Unexpected: could not find display")
        .set_source_texture(Some(&render_texture_view));

    let mut just_recompiled = true;

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
                println!("[Live CLI] Detected file change, recompiling sketch...");

                // TODO: should calling recompile actually do the compile?
                // The issue is it might actually draw, if code is outside the lifecycle functions, maybe that is fine?
                engine
                    .sketches()
                    .get(&sketch_id)
                    .expect("Unexpected: could not find sketch")
                    .request_compile();

                just_recompiled = true;
            }
        }

        match event {
            Event::WindowEvent {
                window_id, event, ..
            } => match event {
                WindowEvent::CloseRequested => {
                    let display_id = *engine.display_id_for_window_id(&window_id);

                    engine.remove_display(&display_id);
                }

                WindowEvent::Destroyed => {
                    if engine.displays().is_empty() {
                        std::process::exit(0)
                    }
                }

                WindowEvent::Resized(size) => {
                    let display_id = *engine.display_id_for_window_id(&window_id);

                    let display = engine
                        .displays_mut()
                        .get_mut(&display_id)
                        .unwrap_or_else(|| {
                            panic!(
                                "Unexpected: could not find display with id {}",
                                display_id.0
                            )
                        });

                    display.resize_surface(size);
                }
                _ => {}
            },

            Event::MainEventsCleared => {
                engine.update();

                let sketch = engine
                    .sketches()
                    .get(&sketch_id)
                    .expect("Unexpected: could not find sketch");

                if just_recompiled {
                    if let Some(error) = sketch.compile_error() {
                        println!("[Sketch compile error] {}", error);
                    }

                    just_recompiled = false;
                }

                if let Some(error) = sketch.runtime_error() {
                    println!("[Sketch runtime error] {}", error);
                }

                let sketch_store = engine
                    .sketch_stores()
                    .get(&sketch_id)
                    .expect("Unexpected: could not find sketch store");

                let logs = visor_plugin_log::LogPlugin::get_state(sketch_store);

                for log in logs {
                    match log.message_type {
                        visor_plugin_log::LogEntryType::Stdout => {
                            println!("[Sketch log] {}", log.message)
                        }
                        visor_plugin_log::LogEntryType::Stderr => {
                            eprintln!("[Sketch error] {}", log.message)
                        }
                    };
                }
            }
            _ => (),
        }
    });
}
