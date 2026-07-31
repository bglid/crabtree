mod cli;
use anyhow::{Context, Result};
use std::{path::PathBuf, process::ExitCode};

use clap::Parser;
use cli::Cli;

fn main() -> ExitCode {
    let args = Cli::parse();

    println!("Searching...");
    run(args).unwrap_or_else(|err| {
        eprintln!("Error, {}", err);
        ExitCode::FAILURE
    })
}

fn run(args: Cli) -> Result<ExitCode> {
    let dirs: PathBuf = match args.directory {
        Some(dir) => dir,
        None => std::env::current_dir().context("Failed to get current directory")?,
    };

    println!("Current dir:\n {}", dirs.display());

    Ok(ExitCode::SUCCESS)
}
