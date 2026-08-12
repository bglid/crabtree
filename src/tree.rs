#![allow(unused)]

use anyhow::{Ok, Result};
use std::{
    error::Error,
    fmt,
    fs::{self, FileType, ReadDir, metadata},
    path::{Path, PathBuf},
};

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

// #[derive(Debug)]
pub struct TreeEntry {
    pub path: PathBuf,
    pub entrytype: EntryType,
    pub depth: usize,
}

impl Tree {
    pub fn new<P: AsRef<Path>>(root: P, entry_type: EntryType) -> Self {
        Tree {
            root: root.as_ref().to_path_buf(),
            children: { Vec::new() },
            etype: entry_type,
        }
    }

    /// Handles pathbuf and turns to path, returning the final file
    pub fn from_pathbuf(root: PathBuf, entry_type: EntryType) -> Self {
        match entry_type {
            EntryType::File => {
                let p = root.file_name();
                match p {
                    Some(f) => {
                        return Tree {
                            root: PathBuf::from(f),
                            children: Vec::new(),
                            etype: entry_type,
                        };
                    }
                    None => {
                        println!("Error getting filename");
                        return Tree {
                            root,
                            children: Vec::new(),
                            etype: entry_type,
                        };
                    }
                };
            }
            EntryType::Dir => println!("Trying to get pathbuf from dir..."),
            EntryType::SymL => println!("Trying to get pathbuf from symlink..."),
            EntryType::Other => println!("Trying to get pathbuf from something else?"),
        }
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

    fn is_dot(path: &PathBuf) -> bool {
        let p = path.as_path();
        p.file_name()
            .and_then(|name| name.to_str())
            .map(|st| st.starts_with("."))
            .unwrap_or_else(|| false)
    }

    fn get_children(root: &PathBuf) -> Result<Vec<Self>> {
        fs::read_dir(root)?.try_fold(Vec::new(), |mut acc, entry| {
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
        })
    }

    /// Private tree traversal
    fn traverse_build(root: PathBuf, ty: EntryType) -> Result<Self> {
        // checking for dotfile or dotdir to skip building the tree
        if Self::is_dot(&root) {
            return Ok(Tree {
                root,
                etype: ty,
                children: Vec::new(),
            });
        };

        let children = Self::get_children(&root);

        // build and return full tree
        Ok(Self {
            root,
            children: children?,
            etype: ty,
        })
    }
}

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
    stack_list: Vec<(Tree, usize)>,
}

// Turning treeiter into an actual iterator
impl Iterator for TreeIter {
    type Item = Result<TreeEntry>;

    fn next(&mut self) -> Option<Result<TreeEntry>> {
        let (tree, depth) = self.stack_list.pop()?;

        // push the children back on the stack
        for child in tree.children.into_iter().rev() {
            self.stack_list.push((child, depth + 1));
        }

        Some(Ok(TreeEntry {
            path: tree.root,
            depth,
            entrytype: tree.etype,
        }))
    }
}

impl fmt::Debug for TreeEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TreeEntry::{:?} => ({:?})", self.entrytype, self.path)
    }
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
