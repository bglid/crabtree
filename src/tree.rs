use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Tree {
    pub root: PathBuf,
    pub children: Vec<Tree>,
}

impl Tree {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Tree {
            root: root.as_ref().to_path_buf(),
            children: { Vec::new() },
        }
    }

    pub fn from_pathbuf(root: PathBuf) -> Self {
        Tree {
            root,
            children: Vec::new(),
        }
    }

    // NOTE: May want to update this design
    pub fn build<P: AsRef<Path>>(root: P) -> Result<Self> {
        Self::traverse_build(root.as_ref().to_path_buf())
    }

    /// Private tree traversal
    fn traverse_build(root: PathBuf) -> Result<Self> {
        // more like build children immediately and recursively
        let children: Result<Vec<Tree>, anyhow::Error> =
            fs::read_dir(&root)?.try_fold(Vec::new(), |mut acc, entry| {
                let path = entry?.path();

                // check if a dir or something else (will want to improve NOTE: )
                if path.is_dir() {
                    acc.push(Self::traverse_build(path)?);
                } else {
                    acc.push(Tree::from_pathbuf(path));
                }
                Ok(acc)
            });

        // build and return full tree
        Ok(Self {
            root,
            children: children?,
        })
    }

    // need to fix below NOTE:
    // pub fn display_tree(&self, depth: usize) {
    //     println!("{}", self.root.display());
    //     match self.into_iter().next() {
    //         Some(t) => self.print_tree(t, depth),
    //         None => (),
    //     }
    // }
    //
    // // helps for above- basically higher order function part
    // fn print_tree(&self, child: &Self, depth: usize) {
    //     println!("{}{}", " ".repeat(depth), child.root.display());
    //
    //     self.into_iter()
    //         .for_each(|child| self.print_tree(child, depth + 1));
    // }

    // pub fn string_of_paths(self, mut res: Vec<&str>) -> Vec<&str> {
    //     res.push(
    //         (self.into_iter().for_each(|child| {
    //             self.string_of_paths(res);
    //             child.root.to_str();
    //         })),
    //     );
    //     res
    // }
}

impl<'a> IntoIterator for &'a Tree {
    type Item = &'a Tree;
    type IntoIter = std::slice::Iter<'a, Tree>;

    fn into_iter(self) -> Self::IntoIter {
        self.children.iter()
    }
}

// Builds a path buf until we hit the bottom
// pub fn traverse_path(root: PathBuf) -> Result<Tree> {
//     // more like build children immediately and recursively
//     let children: Result<Vec<Tree>, anyhow::Error> =
//         fs::read_dir(&root)?.try_fold(Vec::new(), |mut acc, entry| {
//             let path = entry?.path();
//
//             // check if a dir or something else (will want to improve NOTE: )
//             if path.is_dir() {
//                 acc.push(traverse_path(path)?);
//             } else {
//                 acc.push(Tree::from_pathbuf(path));
//             }
//             Ok(acc)
//         });
//
//     // build and return full tree
//     Ok(Tree {
//         root,
//         children: children?,
//     })
// }

// should this be impl?
pub fn print_tree(tree: &Tree, depth: usize) {
    println!("{}{}", " ".repeat(depth), tree.root.display());

    tree.into_iter()
        .for_each(|child| print_tree(child, depth + 1));
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn traverses_dir() ->  {
//         let expected_tree = Tree {
//             root: PathBuf::from("./")
//             children: vec![Tree]
//         };
//     }
//
//     #[test]
//     fn skips_dot_dirs() {
//     }
// }
