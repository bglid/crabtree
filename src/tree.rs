use anyhow::{Ok, Result};
use std::{
    fmt,
    fs::{self, FileType},
    path::{Path, PathBuf},
};

#[derive(Debug, PartialEq)]
pub struct Tree {
    pub root: PathBuf,
    pub etype: EntryType,
    pub children: Vec<Tree>,
}

#[derive(Debug, PartialEq)]
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

pub struct TreeEntry {
    pub path: PathBuf,
    pub entrytype: EntryType,
    pub depth: usize,
    pub last_entry: bool,
    pub ancestor_has_sib: Vec<bool>, // pub ancestor: Vec<Ancestor>,
                                     // pub ancestor: Ancestor,
}

impl Tree {
    #[allow(unused)]
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

    pub fn build<P: AsRef<Path>>(root: P, ignore_flag: Option<Vec<String>>) -> Result<Self> {
        let metadata = fs::metadata(&root)?;
        let ft = metadata.file_type();
        Self::traverse_build(
            root.as_ref().to_path_buf(),
            EntryType::from(ft),
            ignore_flag,
        )
    }

    fn is_dot(path: &Path) -> bool {
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.starts_with("."))
            .unwrap_or_else(|| false)
    }

    fn ignore_dir(path: &Path, ignore_flag: String) -> bool {
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|s| s == ignore_flag)
            .unwrap_or_else(|| false)
    }

    fn get_children(root: &PathBuf, ignore_flag: Option<Vec<String>>) -> Result<Vec<Self>> {
        fs::read_dir(root)?.try_fold(Vec::new(), |mut acc, entry| {
            let entry = entry?;
            // new Entry type for next level
            let entry_ty = EntryType::from(entry.file_type()?);

            let path = entry.path();

            // NOTE: Need to refactor away from clones
            match entry_ty {
                EntryType::Dir => {
                    acc.push(Self::traverse_build(path, entry_ty, ignore_flag.clone())?);
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

    // Returns just the last file in a path
    fn return_last_path(root: PathBuf, children: Vec<Self>, ty: EntryType) -> Result<Self> {
        let Some(os_root) = root.file_name() else {
            return Ok(Self {
                root,
                etype: ty,
                children,
            });
        };
        Ok(Self {
            root: PathBuf::from(os_root),
            etype: ty,
            children,
        })
    }

    // Private tree traversal
    fn traverse_build(
        root: PathBuf,
        ty: EntryType,
        ignore_flag: Option<Vec<String>>,
    ) -> Result<Self> {
        // checking for dotfile or dotdir to skip building the tree
        if Self::is_dot(&root) {
            return Self::return_last_path(root, Vec::new(), ty);
        };

        if let Some(ref flag) = ignore_flag
            && flag.iter().any(|f| Self::ignore_dir(&root, f.to_string()))
        {
            return Self::return_last_path(root, Vec::new(), ty);
        }

        let children = Self::get_children(&root, ignore_flag);

        // handles just returning last dir
        if ty == EntryType::Dir {
            return Self::return_last_path(root, children?, ty);
        }

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
            stack_list: vec![(self, 0, false, vec![])],
            // ancestor_sibling: vec![],
            // min_depth: self.min_depth
            // max_depth: self.max_depth
        }
    }
}

#[derive(Debug)]
pub struct TreeIter {
    stack_list: Vec<(Tree, usize, bool, Vec<bool>)>,
}

// Turning treeiter into an actual iterator
impl Iterator for TreeIter {
    type Item = Result<TreeEntry>;

    fn next(&mut self) -> Option<Result<TreeEntry>> {
        let (tree, depth, is_last, anc_sib) = self.stack_list.pop()?;

        // push the children back on the stack
        for (i, child) in tree.children.into_iter().rev().enumerate() {
            let last: bool = i == 0;
            // creating new sibling vector to chain down
            let new_anc_sib = anc_sib
                .iter()
                .copied()
                .chain(std::iter::once(!is_last))
                .collect();
            self.stack_list.push((child, depth + 1, last, new_anc_sib));
        }

        Some(Ok(TreeEntry {
            path: tree.root,
            depth,
            entrytype: tree.etype,
            last_entry: is_last,
            ancestor_has_sib: anc_sib,
        }))
    }
}

impl fmt::Debug for TreeEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TreeEntry::{:?} => ({:?})", self.entrytype, self.path)
    }
}

// these tests are trash atm
#[cfg(test)]
mod tests {
    use super::*;

    fn create_tree() -> Result<Tree> {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/tree");
        Tree::build(path, None)
    }

    #[test]
    fn traverses_dir() {
        let tree = create_tree().unwrap();
        assert!(tree.children.iter().any(|c| c.root.ends_with("hello.rs")));
        assert!(tree.children.iter().any(|c| c.root.ends_with("subdir")));
    }

    #[test]
    fn skips_trav_dot_dirs() {
        let tree = create_tree().unwrap();
        assert!(tree.children.iter().any(|c| c.root.ends_with(".im_hiding")));
        assert!(
            !tree
                .children
                .iter()
                .any(|c| c.root.ends_with("dont_look_at_me.rs"))
        );
    }
}
