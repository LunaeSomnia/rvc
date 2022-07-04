use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

///A structure that defines a Repository.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Repository {
    pub name: String,
    pub path: PathBuf,
}

impl Repository {
    /// Creates a new repository given a name and a path where that repository is stored.
    ///
    /// The path *can* be relative, as it is going to be converted to absolute for it's storage.
    pub fn new(name: &str, path: &str) -> Self {
        Repository {
            name: name.to_string(),
            path: Path::new(&path).canonicalize().unwrap(),
        }
    }
}
