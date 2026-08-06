mod cli;
mod tree;
use anyhow::{Context, Result};
use std::{path::PathBuf, process::ExitCode};

use clap::Parser;
use cli::Cli;

use crate::tree::Tree;

fn resolve_directory(path_buf: Option<PathBuf>) -> Result<PathBuf> {
    match path_buf {
        Some(dir) => Ok(dir),
        None => std::env::current_dir().context("Failed to get current directory"),
    }
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
    let tree: Tree = Tree::build(dir)?;

    tree.into_iter().for_each(|entry| match entry {
        Ok(e) => {
            if e.depth == 0 {
                println!("{}{}", "-".repeat(e.depth), e.path.display())
            } else {
                println!("{}>{}", "-".repeat(e.depth), e.path.display())
            }
        }
        Err(e) => eprintln!("{}", e),
    });

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
