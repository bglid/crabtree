use anyhow::Result;
use std::path::{Path, PathBuf};
pub struct Tree {
    pub root: PathBuf,
    pub children: Vec<PathBuf>,
}

impl Tree {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Tree {
            root: root.as_ref().to_path_buf(),
            children: { Vec::new() },
        }
    }
}

impl<'a> IntoIterator for &'a Tree {
    type Item = &'a PathBuf;
    type IntoIter = std::slice::Iter<'a, PathBuf>;

    fn into_iter(self) -> Self::IntoIter {
        self.children.iter()
    }
}
