mod cli;
mod tree;
use anyhow::{Context, Ok, Result};
use std::{path::PathBuf, process::ExitCode};

use clap::Parser;
use cli::Cli;
use tree::Tree;

fn resolve_directory(path_buf: Option<PathBuf>) -> Result<PathBuf> {
    match path_buf {
        Some(dir) => Ok(dir),
        None => std::env::current_dir().context("Failed to get current directory"),
    }
}

/// Builds a path buf until we hit the bottom
fn walk_path(root: PathBuf) -> Result<Vec<PathBuf>> {
    let tree = Tree::build(root)?;
    Ok(tree.children)
}

fn main() -> ExitCode {
    let args = Cli::parse();

    println!("Searching...");
    run(args).unwrap_or_else(|err| {
        eprintln!("Error, {}", err);
        ExitCode::FAILURE
    })
}

fn run(args: Cli) -> Result<ExitCode> {
    let dir = resolve_directory(args.directory)?;

    for child in walk_path(dir)? {
        println!("{}", child.display())
    }

    Ok(ExitCode::SUCCESS)
}

// TESTS! :-[)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_dir_from_arg() {
        let expected_dir = PathBuf::from("/test/hello");

        let actual = resolve_directory(Some(expected_dir.clone()))
            .unwrap_or_else(|_err| panic!("Failure in resolving path"));
        assert_eq!(actual, expected_dir)
    }

    #[test]
    fn uses_cd_when_no_arg() {
        let expected_dir = std::env::current_dir().expect("Error in getting cd");
        let actual = resolve_directory(None).expect("Error in resolving directory");
        assert_eq!(actual, expected_dir)
    }
}
