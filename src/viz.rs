use std::io::{self, Write as _};
use std::{fs, path::PathBuf};

use crate::{
    tree::{Tree, TreeEntry},
    viz::TreePart::{FinalEnt, NormalEnt},
};
use anyhow::Result;

// better visualizing chars
const BRANCH: char = '\u{251c}'; // ├
const LEAF: char = '\u{2514}'; // └
const HOR: char = '\u{2500}'; // ─
const VERT: char = '\u{2502}'; // │;

// enum for which point in the tree it is
enum TreePart {
    NormalEnt,
    FinalEnt,
}

impl TreePart {
    fn resolve_part(entry: &TreeEntry) -> Self {
        if entry.last_entry {
            return FinalEnt;
        }
        NormalEnt
    }

    fn print_tree_part(&self, e: &TreeEntry) -> Result<()> {
        match *self {
            NormalEnt => {
                if e.depth > 0 {
                    print_ancestors(e)?;
                    print_tree_line(e, BRANCH)
                } else {
                    writeln!(io::stdout(), "{}", e.path.display())?;
                    Ok(())
                }
            }
            FinalEnt => {
                print_ancestors(e)?;
                print_tree_line(e, LEAF)
            }
        }
    }
}

fn format_symlink(e: &TreeEntry) -> String {
    let link = fs::read_link(&e.path).unwrap_or(e.path.clone());
    format!(" -> {}", link.display())
}

fn print_ancestors(e: &TreeEntry) -> Result<()> {
    let res: String = e
        .ancestor_has_sib
        .iter()
        .skip(1)
        .map(|has_sib| {
            if *has_sib {
                format!("{VERT} ")
            } else {
                "  ".to_owned()
            }
        })
        .collect();

    write!(io::stdout(), "{res}")?;
    Ok(())
}

fn print_tree_line(e: &TreeEntry, tree_char: char) -> Result<()> {
    let mut buffer = format!("{}{}{}", tree_char, HOR, e.path.display());
    if e.symlink {
        buffer.clear();
        buffer = format!(
            "{}{}{}{}",
            tree_char,
            HOR,
            PathBuf::from((e.path).file_name().unwrap_or_else(|| e.path.as_os_str())).display(),
            format_symlink(e),
        );
    }
    write!(io::stdout(), "{buffer}")?;
    Ok(())
}

pub fn visualize_tree(tree: Tree) -> Result<()> {
    for entry in tree {
        match entry {
            Ok(ent) => {
                let part = TreePart::resolve_part(&ent);
                part.print_tree_part(&ent)?;
            }
            // Err(e) => eprintln!("{e}"),
            Err(e) => writeln!(io::stderr(), "{e}")?,
        }
    }
    Ok(())
}
