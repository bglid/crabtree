use crate::{
    tree::{EntryType, Tree, TreeEntry, TreeIter},
    viz::TreePart::{FinalFile, LeafFile, TreeBranch},
};

// better visualizing chars
const BRANCH: char = '├';
const LEAF: char = '└';
const HOR: char = '─';
const VERT: char = '│';

// enum for which point in the tree it is?
enum TreePart {
    TreeBranch,
    LeafFile,
    FinalFile,
}

impl TreePart {
    fn resolve_part(entry: TreeEntry) -> Self {
        if entry.last_entry {
            return FinalFile;
        }

        // placeholder
        match entry.entrytype {
            EntryType::Dir => TreeBranch,
            EntryType::File => LeafFile,
            EntryType::SymL => LeafFile,
            EntryType::Other => unimplemented!(),
        }
    }
}

pub fn visualize_tree(tree: Tree) {
    let mut t_iter: TreeIter = tree.into_iter();
    while let Some(t_ent) = t_iter.next() {
        match t_ent {
            Ok(e) => match e.entrytype {
                EntryType::Dir => {
                    if e.depth == 1 {
                        println!(
                            "{}{}{}{}/",
                            "  ".repeat(e.depth - 1),
                            BRANCH,
                            HOR,
                            e.path.display()
                        )
                    } else if e.depth > 1 {
                        println!(
                            "{}{}{}{}{}/",
                            VERT,
                            "  ".repeat(e.depth - 1),
                            LEAF,
                            HOR,
                            e.path.display()
                        )
                    } else {
                        println!("{}", e.path.display())
                    }
                }
                EntryType::File => {
                    if e.depth - 1 == 0 {
                        println!(
                            "{}{}{}{}",
                            "  ".repeat(e.depth - 1),
                            BRANCH,
                            HOR,
                            e.path.display()
                        )
                    } else {
                        println!(
                            "{}{}{}{}{}",
                            VERT,
                            "  ".repeat(e.depth - 1),
                            BRANCH,
                            HOR,
                            e.path.display()
                        )
                    }
                }
                _ => unimplemented!(),
            },
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
