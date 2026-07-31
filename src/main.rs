mod cli;
use std::path::PathBuf;

use clap::Parser;
use cli::Cli;

fn main() {
    let args = Cli::parse();

    println!("Searching...");
    let dir = args.directory.unwrap_or_else(|| {
        std::env::current_dir().unwrap_or_else(|err| {
            eprintln!("Error getting current directory {}", err);
            PathBuf::from(".")
        })
    });

    println!("Current dir:\n {}", dir.display());
}
