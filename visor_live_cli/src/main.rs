use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    sync::{Arc, mpsc},
};

use clap::Parser;
use notify::{RecommendedWatcher, Watcher};
use tao::{
    event::{Event, WindowEvent},
    window::WindowBuilder,
};
use visor_core::{DisplayBuilder, EngineBuilder, SketchBuilder, default_plugins};

#[derive(Parser)]
#[command(
    version,
    about = "Utility for running Visor sketches from the command line."
)]
struct Args {
    #[clap(subcommand)]
    command: Command,

    #[arg(
        short,
        long,
        help = "List of Visor plugin file paths e.g: ~/visor/plugin_a.dylib ~/visor/plugin_b.dylib",
        num_args = 1..,
        value_delimiter = ' ',
        global = true,
    )]
    plugins: Option<Vec<PathBuf>>,
}

#[derive(Parser)]
enum Command {
    #[clap(about = "Run a Visor sketch")]
    Run(RunArgs),
    #[clap(about = "Generate TypeScript declarations for the Visor API")]
    Types(TypesArgs),
}

#[derive(Parser)]
struct RunArgs {
    #[arg(help = "Path to a Visor sketch file e.g: ~/visor/sketch.ts")]
    sketch_file_path: PathBuf,

    #[arg(short, long, help = "Watch the sketch file and hot reload on changes")]
    watch: bool,
}

#[derive(Parser)]
struct TypesArgs {
    #[arg(help = "Path to write the TypeScript declarations file e.g: ~/visor/types.d.ts")]
    output_file_path: PathBuf,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Run(run_args) => run_sketch(run_args, args.plugins),
        Command::Types(types_args) => generate_types(types_args, args.plugins),
    }
}

fn run_sketch(args: RunArgs, plugins: Option<Vec<PathBuf>>) {
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

    let mut engine_builder = EngineBuilder::default().with_plugins(default_plugins());

    if let Some(plugins) = plugins {
        engine_builder = engine_builder.with_linked_plugins(plugins)
    }

    let mut engine = engine_builder.build();

    let sketch_id = *SketchBuilder::new(args.sketch_file_path)
        .build(&mut engine)
        .id();

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

    let display_id = *DisplayBuilder::new(Arc::new(window))
        .build(&mut engine)
        .id();

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

fn generate_types(args: TypesArgs, plugins: Option<Vec<PathBuf>>) {
    println!("Outputting: {:?}", plugins);

    let mut engine_builder = EngineBuilder::default().with_plugins(default_plugins());

    if let Some(plugins) = plugins {
        engine_builder = engine_builder.with_linked_plugins(plugins)
    }

    let engine = engine_builder.build();

    let typescript_declarations = engine
        .typescript_declarations()
        .expect("Unexpected: engine typescript declarations should exist");

    let output_file_path_string = args.output_file_path.clone().display().to_string();

    let mut file = File::create(args.output_file_path).unwrap_or_else(|_| {
        panic!(
            "Failed to create file at output path: {}",
            output_file_path_string
        )
    });

    file.write_all(typescript_declarations.as_bytes())
        .unwrap_or_else(|_| {
            panic!(
                "Failed to write TypeScript declarations to file: {}",
                output_file_path_string
            )
        });

    println!(
        "[Live CLI] Wrote TypeScript declarations to output file {}",
        output_file_path_string
    );
}
