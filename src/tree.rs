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
    pub symlink: bool,
}

#[derive(Debug, PartialEq)]
pub enum EntryType {
    Dir,
    File,
    SymL,
    Other,
}

#[allow(
    clippy::filetype_is_file,
    reason = "Needs to check for file and not rule out symlink"
)]
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
    pub symlink: bool,
    pub ancestor_has_sib: Vec<bool>,
}

impl Tree {
    pub fn new<P>(root: P, entry_type: EntryType) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        // NOTE: DOESNT FOLLOW SYMLINKS
        let metadata = fs::symlink_metadata(&root)?;
        Ok(Tree {
            root: root.as_ref().to_path_buf(),
            children: { Vec::new() },
            etype: entry_type,
            symlink: metadata.file_type().is_symlink(),
        })
    }

    pub fn build<P>(root: P, ignore_flag: Option<&Vec<String>>) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let metadata = fs::symlink_metadata(&root)?;
        let ft = metadata.file_type();
        Self::traverse_build(
            root.as_ref().to_path_buf(),
            EntryType::from(ft),
            ignore_flag,
        )
    }

    /// Handles pathbuf and turns to path, returning the final file.
    pub fn from_pathbuf(root: PathBuf, entry_type: EntryType) -> Self {
        let p = root.file_name();
        let maybe_new_root: PathBuf = if let Some(f) = p {
            PathBuf::from(f)
        } else {
            root
        };
        // Symlink check, all else are good due to check above
        match entry_type {
            EntryType::SymL => Tree {
                root: maybe_new_root,
                etype: entry_type,
                children: Vec::new(),
                symlink: true,
            },
            EntryType::Dir | EntryType::File | EntryType::Other => Tree {
                root: maybe_new_root,
                etype: entry_type,
                children: Vec::new(),
                symlink: false,
            },
        }
    }

    fn is_dot(path: &Path) -> bool {
        path.file_name()
            .and_then(|name| name.to_str())
            .map_or_else(|| false, |s| s.starts_with('.'))
    }

    fn ignore_dir(path: &Path, ignore_flag: &str) -> bool {
        // path.file_name()
        //     .and_then(|name| name.to_str())
        //     .map(|s| s == ignore_flag)
        //     .unwrap_or_else(|| false)
        path.file_name()
            .and_then(|name| name.to_str())
            .map_or_else(|| false, |s| s == ignore_flag)
    }

    fn get_children(root: &PathBuf, ignore_flag: Option<&Vec<String>>) -> Result<Vec<Self>> {
        fs::read_dir(root)?.try_fold(Vec::new(), |mut acc, entry| {
            let entry = entry?;
            // new Entry type for next level
            let entry_ty = EntryType::from(entry.file_type()?);

            let path = entry.path();

            // NOTE: Need to refactor away from clones
            match entry_ty {
                EntryType::Dir => {
                    acc.push(Self::traverse_build(
                        path,
                        entry_ty,
                        ignore_flag.cloned().as_ref(),
                    )?);
                }
                EntryType::File => {
                    acc.push(Self::from_pathbuf(path, entry_ty));
                }
                EntryType::SymL | EntryType::Other => acc.push(Self::new(path, entry_ty)?),
            }

            Ok(acc)
        })
    }

    // Returns just the last file in a path
    fn return_last_path(root: PathBuf, children: Vec<Self>, ty: EntryType) -> Result<Self> {
        let syml = ty == EntryType::SymL;
        let Some(os_root) = root.file_name() else {
            return Ok(Self {
                root,
                etype: ty,
                children,
                symlink: syml,
            });
        };
        Ok(Self {
            root: PathBuf::from(os_root),
            etype: ty,
            children,
            symlink: syml,
        })
    }

    // Private tree traversal
    fn traverse_build(
        root: PathBuf,
        ty: EntryType,
        ignore_flag: Option<&Vec<String>>,
    ) -> Result<Self> {
        // checking for dotfile or dotdir to skip building the tree
        if Self::is_dot(&root) {
            return Self::return_last_path(root, Vec::new(), ty);
        }

        if let Some(flag) = ignore_flag
            && flag.iter().any(|f| Self::ignore_dir(&root, &f.clone()))
        {
            return Self::return_last_path(root, Vec::new(), ty);
        }

        let children = Self::get_children(&root, ignore_flag);

        // handles just returning last dir
        Self::return_last_path(root, children?, ty)
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
            symlink: tree.symlink,
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

    fn create_sl_tree() -> Result<Tree> {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/symls");
        Tree::build(path, None)
    }

    #[test]
    #[allow(clippy::unwrap_used, reason = "test case")]
    fn traverses_dir() {
        let tree = create_tree().unwrap();
        assert!(tree.children.iter().any(|c| c.root.ends_with("hello.rs")));
        assert!(tree.children.iter().any(|c| c.root.ends_with("subdir")));
        assert!(tree.children.iter().any(|c| !c.symlink));
    }

    #[test]
    #[allow(clippy::unwrap_used, reason = "test case")]
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

    #[test]
    #[allow(clippy::unwrap_used, reason = "test case")]
    fn handles_symlinks() {
        let tree = create_sl_tree().unwrap();
        assert!(tree.children.iter().any(|c| c.symlink));
    }
}
