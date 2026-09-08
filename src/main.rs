mod cli;
mod tree;
mod viz;
use anyhow::Result;
use std::io::{self, Write as _};
use std::process::ExitCode;

use clap::Parser as _;
use cli::Cli;

use viz::visualize_tree;

use crate::tree::Tree;

fn main() -> ExitCode {
    let args = Cli::parse();

    run(args).unwrap_or_else(|err| {
        #[allow(
            clippy::let_underscore_must_use,
            reason = "Cannot recover in meaningful way at this point"
        )]
        let _ = writeln!(io::stderr(), "{err}");
        ExitCode::FAILURE
    })
}

fn run(args: Cli) -> Result<ExitCode> {
    let dir = Cli::resolve_directory(args.directory)?;
    let tree: Tree = Tree::build(dir, args.ignore.as_ref())?;

    visualize_tree(tree)?;

    Ok(ExitCode::SUCCESS)
}

// TESTS! :-[)
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    #[allow(clippy::expect_used, reason = "unit test")]
    fn uses_dir_from_arg() {
        let expected_dir = PathBuf::from("/test/hello");

        let actual = Cli::resolve_directory(Some(expected_dir.clone()))
            .expect("Error in resolving directory");
        assert_eq!(actual, expected_dir);
    }

    #[test]
    #[allow(clippy::expect_used, reason = "unit test")]
    fn uses_cd_when_no_arg() {
        let expected_dir = std::env::current_dir().expect("Error in getting cd");
        let actual = Cli::resolve_directory(None).expect("Error in resolving directory");
        assert_eq!(actual, expected_dir);
    }
}
