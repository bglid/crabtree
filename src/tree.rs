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
        let tree = Tree::new(root);
        let children = fs::read_dir(&tree.root)?
            .map(|entry| entry.map(|e| Tree::from_pathbuf(e.path())))
            .collect::<std::io::Result<Vec<Tree>>>()?;

        Ok(Self {
            root: tree.root,
            children,
        })
    }
}

impl<'a> IntoIterator for &'a Tree {
    type Item = &'a Tree;
    type IntoIter = std::slice::Iter<'a, Tree>;

    fn into_iter(self) -> Self::IntoIter {
        self.children.iter()
    }
}

/// Builds a path buf until we hit the bottom
pub fn traverse_path(root: PathBuf) -> Result<Tree> {
    // more like build children immediately and recursively
    let children: Result<Vec<Tree>, anyhow::Error> =
        fs::read_dir(&root)?.try_fold(Vec::new(), |mut acc, entry| {
            let path = entry?.path();

            // check if a dir or something else (will want to improve NOTE: )
            if path.is_dir() {
                acc.push(traverse_path(path)?);
            } else {
                acc.push(Tree::from_pathbuf(path));
            }
            Ok(acc)
        });

    // build and return full tree
    Ok(Tree {
        root,
        children: children?,
    })
}

// should this be impl?
pub fn print_tree(tree: &Tree, depth: usize) {
    println!("{}{}", " ".repeat(depth), tree.root.display());

    tree.into_iter()
        .for_each(|child| print_tree(child, depth + 1));
}
