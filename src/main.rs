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
    let dir = args.directory.unwrap_or_else(|| {
        std::env::current_dir().unwrap_or_else(|err| {
            eprintln!("Error getting current directory {}", err);
            PathBuf::from(".")
        })
    });
    println!("Current dir:\n {}", dir.display());

    Ok(ExitCode::SUCCESS)
}
