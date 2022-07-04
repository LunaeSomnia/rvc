use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Repository {
    pub name: String,
    pub path: PathBuf,
}

impl Repository {
    pub fn new(name: &str, path: &str) -> Self {
        Repository {
            name: name.to_string(),
            path: Path::new(&path).canonicalize().unwrap(),
        }
    }
}
