use crate::{
    tree::{EntryType, Tree, TreeEntry, TreeIter},
    viz::TreePart::{FinalFile, TreeBranch, TreeFile},
};

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

    fn print_ancestors(&self, e: &TreeEntry) {
        // for anc in 0..e.ancestor_has_sib.len() {
        //     if anc == 0 {
        //         continue;
        //     }
        //     if e.ancestor_has_sib[anc] {
        //         print!("{}{}", VERT, " ".repeat(e.depth.saturating_sub(anc)),);
        //     } else {
        //         print!("{}", " ".repeat(e.depth.saturating_sub(anc)),);
        //     }
        // }
        let res: String = e
            .ancestor_has_sib
            .iter()
            .skip(1)
            .map(|has_sib| {
                // let indent = String::from(" ");
                if *has_sib {
                    format!("{}{}", VERT, " ")
                } else {
                    "  ".to_string()
                }
            })
            .collect();

        print!("{}", res)
    }

    fn print_tree_part(&self, e: &TreeEntry) {
        match self {
            TreeBranch => {
                if e.depth > 0 {
                    self.print_ancestors(e);
                    println!(
                        "{}{}{}{}/",
                        " ".repeat(e.depth.saturating_sub(1)),
                        BRANCH,
                        HOR,
                        e.path.display()
                    )
                } else {
                    println!("{}", e.path.display())
                }
            }
            TreeFile => {
                self.print_ancestors(e);
                println!(
                    "{}{}{}{}",
                    " ".repeat(e.depth.saturating_sub(1)),
                    BRANCH,
                    HOR,
                    e.path.display()
                )
            }
            FinalFile => {
                self.print_ancestors(e);
                println!(
                    "{}{}{}{}",
                    " ".repeat(e.depth.saturating_sub(1)),
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
                // part.print_ancestors(&e);
                part.print_tree_part(&e);
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}
