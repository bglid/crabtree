mod cli;
mod tree;
use anyhow::Result;
use std::process::ExitCode;

use clap::Parser;
use cli::Cli;

use crate::tree::{EntryType, Tree};

fn main() -> ExitCode {
    let args = Cli::parse();

    println!("Searching...");
    run(args).unwrap_or_else(|err| {
        eprintln!("Error, {}", err);
        ExitCode::FAILURE
    })
}

fn run(args: Cli) -> Result<ExitCode> {
    let dir = Cli::resolve_directory(args.directory)?;
    let tree: Tree = Tree::build(dir)?;

    tree.into_iter()
        // .filter(|e| {
        //     args.ignore
        //         .iter()
        //         // THIS NEEDS TO VCHANGE!!!!
        //         .any(|ignores| {
        //             !ignores.starts_with(&String::from(e.as_ref().unwrap().path.to_str().unwrap()))
        //         })
        // })
        .for_each(|entry| match entry {
            Ok(e) => match e.entrytype {
                EntryType::Dir => {
                    // dbg!(&e);
                    if e.depth == 0 {
                        println!("{}{}", "-".repeat(e.depth), e.path.display())
                    } else {
                        println!("{}-{}", "-".repeat(e.depth), e.path.display())
                    }
                }
                EntryType::File => {
                    // dbg!(&e);
                    if e.depth == 0 {
                        println!("{}{}", "-".repeat(e.depth), e.path.display())
                    } else {
                        println!("{}>{}", "-".repeat(e.depth), e.path.display())
                    }
                }
                _ => unimplemented!(),
            },
            Err(e) => eprintln!("{}", e),
        });

    Ok(ExitCode::SUCCESS)
}

// TESTS! :-[)
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn uses_dir_from_arg() {
        let expected_dir = PathBuf::from("/test/hello");

        let actual = Cli::resolve_directory(Some(expected_dir.clone()))
            .unwrap_or_else(|_err| panic!("Failure in resolving path"));
        assert_eq!(actual, expected_dir)
    }

    #[test]
    fn uses_cd_when_no_arg() {
        let expected_dir = std::env::current_dir().expect("Error in getting cd");
        let actual = Cli::resolve_directory(None).expect("Error in resolving directory");
        assert_eq!(actual, expected_dir)
    }
}
