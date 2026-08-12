use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    /// Directory to build tree FROM
    pub directory: Option<PathBuf>,

    /// Directories to ignore
    #[arg(short = 'I', long = "ignore-dir")]
    pub ignore: Vec<String>,
}
impl Cli {
    pub fn resolve_directory(path_buf: Option<PathBuf>) -> Result<PathBuf> {
        match path_buf {
            Some(dir) => Ok(dir),
            None => std::env::current_dir().context("Failed to get current directory"),
        }
    }
}
