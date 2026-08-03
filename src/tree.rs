use anyhow::Result;
use std::path::PathBuf;
pub struct Tree {
    pub root: PathBuf,
    pub children: Vec<PathBuf>,
}

impl IntoIterator for Tree {
    type Item = Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {}
    }
}
