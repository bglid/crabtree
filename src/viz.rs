use std::{fs, path::PathBuf};

use crate::{
    tree::{Tree, TreeEntry},
    viz::TreePart::{FinalEnt, NormalEnt},
};

// better visualizing chars
const BRANCH: char = '├';
const LEAF: char = '└';
const HOR: char = '─';
const VERT: char = '│';

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

    fn print_ancestors(&self, e: &TreeEntry) {
        let res: String = e
            .ancestor_has_sib
            .iter()
            .skip(1)
            .map(|has_sib| {
                if *has_sib {
                    format!("{VERT} ")
                } else {
                    "  ".to_string()
                }
            })
            .collect();

        print!("{}", res)
    }

    fn format_symlink(&self, e: &TreeEntry) -> String {
        let link = fs::read_link(&e.path).unwrap();
        format!(" -> {}", link.display())
    }

    fn print_tree_line(&self, e: &TreeEntry, tree_char: char) {
        let mut buffer = format!("{}{}{}", tree_char, HOR, e.path.display());
        if e.symlink {
            buffer.push_str(&self.format_symlink(e));
        }
        println!("{}", buffer);
    }

    fn print_tree_part(&self, e: &TreeEntry) {
        match self {
            NormalEnt => {
                if e.depth > 0 {
                    self.print_ancestors(e);
                    self.print_tree_line(e, BRANCH);
                } else {
                    println!("{}", e.path.display())
                }
            }
            FinalEnt => {
                self.print_ancestors(e);
                self.print_tree_line(e, LEAF);
            }
        }
    }
}

pub fn visualize_tree(tree: Tree) {
    tree.into_iter().for_each(|t_entry| match t_entry {
        Ok(ent) => {
            let part = TreePart::resolve_part(&ent);
            part.print_tree_part(&ent);
        }
        Err(e) => eprintln!("{}", e),
    });
}
