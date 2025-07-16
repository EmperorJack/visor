use std::{fs::File, io::Write, path::PathBuf};

use clap::Parser;
use visor_core::{EngineBuilder, default_plugins};

#[derive(Parser)]
pub(crate) struct TypesArgs {
    #[arg(help = "Path to write the TypeScript declarations file e.g: ~/visor/types.d.ts")]
    output_file_path: PathBuf,
}

pub(crate) fn generate_types(args: TypesArgs, plugins: Option<Vec<PathBuf>>) {
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
