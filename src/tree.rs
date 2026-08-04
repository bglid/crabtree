use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};
// pub struct Tree {
//     pub root: PathBuf,
//     pub children: Vec<PathBuf>,
// }

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
        let tree = Tree::new(root);
        let children = fs::read_dir(&tree.root)?
            // .map(|entry| entry.map(|e| e.path()))
            .map(|entry| entry.map(|e| Tree::from_pathbuf(e.path())))
            // .collect::<std::io::Result<Vec<PathBuf>>>()?;
            .collect::<std::io::Result<Vec<Tree>>>()?;

        Ok(Self {
            root: tree.root,
            children,
        })
    }
}

impl<'a> IntoIterator for &'a Tree {
    // type Item = &'a PathBuf;
    type Item = &'a Tree;
    // type IntoIter = std::slice::Iter<'a, PathBuf>;
    type IntoIter = std::slice::Iter<'a, Tree>;

    fn into_iter(self) -> Self::IntoIter {
        self.children.iter()
    }
}
