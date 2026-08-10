#![allow(unused)]

use anyhow::Result;
use std::{
    error::Error,
    fs::{self, FileType, ReadDir, metadata},
    path::{Path, PathBuf},
};

// use crate::tree::TreeList::{Closed, Opened};

#[derive(Debug)]
pub struct Tree {
    pub root: PathBuf,
    pub etype: EntryType,
    pub children: Vec<Tree>,
}

#[derive(Debug)]
pub enum EntryType {
    Dir,
    File,
    SymL,
    Other,
}

impl From<FileType> for EntryType {
    fn from(value: FileType) -> Self {
        if value.is_dir() {
            Self::Dir
        } else if value.is_file() {
            Self::File
        } else if value.is_symlink() {
            Self::SymL
        } else {
            Self::Other
        }
    }
}

#[derive(Debug)]
pub struct TreeEntry {
    pub path: PathBuf,
    pub entrytype: EntryType,
    pub depth: usize,
    pub dotfile: bool,
}

impl Tree {
    pub fn new<P: AsRef<Path>>(root: P, entry_type: EntryType) -> Self {
        Tree {
            root: root.as_ref().to_path_buf(),
            children: { Vec::new() },
            etype: entry_type,
        }
    }

    pub fn from_pathbuf(root: PathBuf, entry_type: EntryType) -> Self {
        Tree {
            root,
            children: Vec::new(),
            etype: entry_type,
        }
    }

    pub fn build<P: AsRef<Path>>(root: P) -> Result<Self> {
        let metadata = fs::metadata(&root)?;
        let ft = metadata.file_type();
        Self::traverse_build(root.as_ref().to_path_buf(), EntryType::from(ft))
    }

    /// Private tree traversal
    fn traverse_build(root: PathBuf, ty: EntryType) -> Result<Self> {
        let children: Result<Vec<Tree>, anyhow::Error> =
            fs::read_dir(&root)?.try_fold(Vec::new(), |mut acc, entry| {
                let entry = entry?;
                // new Entry type for next level
                let entry_ty = EntryType::from(entry.file_type()?);

                let path = entry.path();

                match entry_ty {
                    EntryType::Dir => {
                        acc.push(Self::traverse_build(path, entry_ty)?);
                    }
                    EntryType::File => {
                        acc.push(Self::from_pathbuf(path, entry_ty));
                    }
                    EntryType::SymL => unimplemented!(),
                    EntryType::Other => unimplemented!(),
                }

                Ok(acc)
            });

        // build and return full tree
        Ok(Self {
            root,
            children: children?,
            etype: ty,
        })
    }

    // need to fix below NOTE:
    // pub fn display_tree(&self, depth: usize) {
    //     println!("{}", self.root.display());
    //     //     match self.into_iter().next() {
    //     //         Some(t) => self.print_tree(t, depth),
    //     //         None => (),
    //     //     }
    //     self.into_iter().for_each(|entry| match entry {
    //         Ok(e) => println!("{}>{}", "-".repeat(e.depth), e.path.display()),
    //         Err(e) => eprintln!("{}", e),
    //     });
    // }

    //
    // // helps for above- basically higher order function part
    // fn print_tree(&self, child: TreeIter, depth: usize) {
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
// should this be impl?
// pub fn print_tree(tree: &Tree, depth: usize) {
//     println!("{}{}", " ".repeat(depth), tree.root.display());
//
//     // tree.into_iter()
//     //     .for_each(|child| print_tree(child, depth + 1));
//
// }

// pub fn print_tree(tree: &Tree, depth: usize) {
//     println!("{}{}", " ".repeat(depth), tree.root.display());
//
//     // tree.into_iter()
//     //     .for_each(|child| print_tree(child, depth + 1));
//
//         self.into_iter().for_each(|entry| match entry {
//             Ok(e) => println!("{}>{}", "-".repeat(e.depth), e.path.display()),
//             Err(e) => eprintln!("{}", e),
//         });
// }

// ----- removing for now because adds a ton of unecessary complexity
// #[derive(Debug)]
// enum TreeList {
//     // Opened(Result<ReadDir, Option<std::io::Error>>),
//     Opened {
//         depth: usize,
//         tre: Result<ReadDir, Option<std::io::Error>>,
//     },
//     // Closed(Vec::TreeIter<Result<TreeEntry>>),
//     // Closed(Vec<TreeIter>),
//     Closed(TreeIter),
// }

// impl Iterator for TreeList {
//     type Item = Result<TreeEntry>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         match self {
//             TreeList::Closed(treeitem) => treeitem.next(),
//             TreeList::Opened { depth, tre } => match tre {
//                 Ok(read_dir) => match read_dir.next() {
//                     Some(dir) => match dir {
//                         Ok(entry) => Some(Ok(TreeEntry {
//                             path: entry.path(),
//                             // NOTE: TODO -> handle getting filetype elsewhere first
//                             filetype: entry.file_type().expect("NEED TO IMPLEMENT"),
//                             depth: *depth,
//                         })),
//                         Err(err) => unimplemented!(),
//                     },
//                     None => None,
//                 },
//                 Err(Some(err)) => unimplemented!(),
//                 Err(None) => None,
//             },
//         }
//     }
// }

// ----------
// iter stuff
impl IntoIterator for Tree {
    type Item = Result<TreeEntry>;
    type IntoIter = TreeIter;

    fn into_iter(self) -> TreeIter {
        TreeIter {
            stack_list: vec![(self, 0)],
        }
    }
}

#[derive(Debug)]
pub struct TreeIter {
    // start: Option<PathBuf>,
    // stack_list: Vec<TreeList>,
    stack_list: Vec<(Tree, usize)>,
}

// Turning treeiter into an actual iterator
impl Iterator for TreeIter {
    type Item = Result<TreeEntry>;

    fn next(&mut self) -> Option<Result<TreeEntry>> {
        let (tree, depth) = self.stack_list.pop()?;

        // skipping if dotfile
        // let dot = match tree.root.to_str()?.chars().next() {
        //     Some(first) => true,
        //     _ => false,
        // };

        // push the children back on the stack
        for child in tree.children.into_iter().rev() {
            self.stack_list.push((child, depth + 1));
        }

        Some(Ok(TreeEntry {
            path: tree.root,
            depth,
            entrytype: tree.etype,
            dotfile: false,
        }))
    }
}

// impl TreeIter {
//     pub fn push(&self) -> Self {
//         self.stack_list
//     }
// }

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
