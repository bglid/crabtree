use crate::{
    tree::{EntryType, Tree, TreeEntry, TreeIter},
    viz::TreePart::{FinalFile, TreeBranch, TreeFile},
};
use anyhow::Result;

// better visualizing chars
const BRANCH: char = '├';
const LEAF: char = '└';
const HOR: char = '─';
const VERT: char = '│';

// enum for which point in the tree it is?
enum TreePart {
    TreeBranch,
    TreeFile,
    FinalFile,
}

impl TreePart {
    fn resolve_part(entry: &TreeEntry) -> Self {
        if entry.last_entry {
            return FinalFile;
        }

        // placeholder
        match entry.entrytype {
            EntryType::Dir => TreeBranch,
            EntryType::File => TreeFile,
            EntryType::SymL => TreeFile,
            EntryType::Other => unimplemented!(),
        }
    }

    fn print_tree_part(&self, e: &TreeEntry) {
        match self {
            TreeBranch => {
                if e.depth > 0 {
                    println!(
                        "{}{}{}{}/",
                        "  ".repeat(e.depth),
                        BRANCH,
                        HOR,
                        e.path.display()
                    )
                } else {
                    println!("{}", e.path.display())
                }
            }
            TreeFile => {
                println!(
                    "{}{}{}{}",
                    "  ".repeat(e.depth),
                    BRANCH,
                    HOR,
                    e.path.display()
                )
            }
            FinalFile => {
                println!(
                    "{}{}{}{}",
                    "  ".repeat(e.depth),
                    LEAF,
                    HOR,
                    e.path.display()
                )
            }
        }
    }
}

pub fn visualize_tree(tree: Tree) {
    let mut t_iter: TreeIter = tree.into_iter();
    while let Some(t_ent) = t_iter.next() {
        match t_ent {
            Ok(e) => {
                let part = TreePart::resolve_part(&e);
                part.print_tree_part(&e);
            }
            Err(e) => eprintln!("{}", e),
        }
    }

    // tree.into_iter().for_each(|entry| match entry {
    //     Ok(e) => match e.entrytype {
    //         EntryType::Dir => {
    //             if e.depth == 1 {
    //                 println!(
    //                     "{}{}{}{}/",
    //                     "  ".repeat(e.depth - 1),
    //                     BRANCH,
    //                     HOR,
    //                     e.path.display()
    //                 )
    //             } else if e.depth > 1 {
    //                 println!(
    //                     "{}{}{}{}{}/",
    //                     VERT,
    //                     "  ".repeat(e.depth - 1),
    //                     LEAF,
    //                     HOR,
    //                     e.path.display()
    //                 )
    //             } else {
    //                 println!("{}", e.path.display())
    //             }
    //         }
    //         EntryType::File => {
    //             if e.depth - 1 == 0 {
    //                 println!(
    //                     "{}{}{}{}",
    //                     "  ".repeat(e.depth - 1),
    //                     BRANCH,
    //                     HOR,
    //                     e.path.display()
    //                 )
    //             } else {
    //                 println!(
    //                     "{}{}{}{}{}",
    //                     VERT,
    //                     "  ".repeat(e.depth - 1),
    //                     BRANCH,
    //                     HOR,
    //                     e.path.display()
    //                 )
    //             }
    //         }
    //         _ => unimplemented!(),
    //     },
    //     Err(e) => eprintln!("{}", e),
    // });
}
