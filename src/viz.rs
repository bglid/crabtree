use crate::tree::{EntryType, Tree};

// better visualizing chars
const BRANCH: char = '├';
const LEAF: char = '└';
const HOR: char = '─';
const VERT: char = '│';

pub fn visualize_tree(tree: Tree) {
    tree.into_iter().for_each(|entry| match entry {
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
                        BRANCH,
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
                        LEAF,
                        HOR,
                        e.path.display()
                    )
                }
            }
            _ => unimplemented!(),
        },
        Err(e) => eprintln!("{}", e),
    });
}
