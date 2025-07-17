use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use log::LevelFilter;

use crate::{
    run::{RunArgs, run_sketch},
    types::{TypesArgs, generate_types},
};

mod run;
mod types;

#[derive(Parser)]
#[command(version, about = "Command line interface for running Visor sketches.")]
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

fn main() -> Result<()> {
    let env = env_logger::Env::new();

    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .filter_module("naga", LevelFilter::Off)
        .filter_module("tao", LevelFilter::Off)
        .filter_module("wgpu", LevelFilter::Off)
        .filter_module("wgpu_core", LevelFilter::Off)
        .format_timestamp(None)
        .parse_env(env)
        .init();

    let args = Args::parse();

    match args.command {
        Command::Run(run_args) => run_sketch(run_args, args.plugins),
        Command::Types(types_args) => generate_types(types_args, args.plugins),
    }
}
