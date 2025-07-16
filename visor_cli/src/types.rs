use std::{fs::File, io::Write, path::PathBuf};

use anyhow::{Result, bail};
use clap::Parser;
use visor_core::{EngineBuilder, core_plugins};

#[derive(Parser)]
pub(crate) struct TypesArgs {
    #[arg(help = "Path to write the TypeScript declarations file e.g: ~/visor/types.d.ts")]
    output_file_path: PathBuf,
}

pub(crate) fn generate_types(args: TypesArgs, plugins: Option<Vec<PathBuf>>) -> Result<()> {
    let mut engine_builder = EngineBuilder::default().with_plugins(core_plugins());

    if let Some(plugins) = plugins {
        engine_builder = engine_builder.with_linked_plugins(plugins)
    }

    let engine = engine_builder.build();

    let typescript_declarations = engine
        .typescript_declarations()
        .expect("Unexpected: engine typescript declarations should exist");

    let output_file_path_string = args.output_file_path.clone().display().to_string();

    let Ok(mut file) = File::create(args.output_file_path) else {
        bail!(
            "Failed to create file at output path: {}",
            output_file_path_string
        )
    };

    if file.write_all(typescript_declarations.as_bytes()).is_err() {
        bail!(
            "Failed to write TypeScript declarations to file: {}",
            output_file_path_string
        )
    }

    println!(
        "[CLI] Wrote TypeScript declarations to output file {}",
        output_file_path_string
    );

    Ok(())
}
