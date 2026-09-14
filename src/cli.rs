use anyhow::{Context as _, Result};
use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'd', long = "directory")]
    /// Directory to build tree FROM.
    pub directory: Option<PathBuf>,

    /// Directories to ignore.
    #[arg(short = 'i', long = "ignore-dir", num_args = 1.., value_delimiter = ',')]
    pub ignore: Option<Vec<String>>,

    #[arg(long = "max-depth", num_args = 1)]
    pub max_depth: Option<usize>,
}
impl Cli {
    pub fn resolve_directory(path_buf: Option<&Path>) -> Result<PathBuf> {
        match path_buf {
            Some(dir) => Ok(dir.to_path_buf()),
            None => std::env::current_dir().context("Failed to get current directory"),
        }
    }
}
