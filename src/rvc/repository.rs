use std::{
    collections::HashMap,
    fmt::Display,
    fs::{remove_file, File},
    io::Write,
    path::PathBuf,
};

use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

use crate::utils::load_file;

use super::commit::Commit;

///A structure that defines a Repository.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Repository {
    pub name: String,
    pub path: PathBuf,
    pub commits: Vec<Commit>,
    pub checkout: HashMap<PathBuf, String>,
}

impl Repository {
    pub fn rvc_path(&self) -> PathBuf {
        self.path.as_path().join(".rvc")
    }

    pub fn retreive(path: PathBuf) -> Result<Repository, String> {
        // Choose the path if given or the default path
        let rvc_path = path.as_path().join(".rvc");

        let rvc: Repository = ron::from_str(&load_file(&rvc_path)).unwrap();

        Ok(rvc)
    }

    /// Creates a new repository given a name and a path where that repository is stored.
    pub fn create(name: &str, path: PathBuf) -> Result<Self, String> {
        let path = path.canonicalize().unwrap();
        let rvc_path = path.join(".rvc");

        if rvc_path.exists() {
            return Err("A repository already exists in the given path".to_owned());
        }

        if path.exists() && path.is_dir() {
            let mut repo = Repository {
                name: name.to_string(),
                commits: Vec::new(),
                path,
                checkout: HashMap::new(),
            };

            let c = Commit::create("init", &mut repo).unwrap();
            repo.push_commit(c);

            Ok(repo)
        } else {
            Err(String::from("The path is not valid"))
        }
    }

    pub fn save(&mut self) {
        let rvc_path = self.rvc_path();

        // Make sure the file saved is in a good shape
        // for (_, s) in &mut self.checkout {
        //     if s.len() == 0 || s.chars().last().unwrap() != '\n' {
        //         s.push('\n');
        //     }
        // }

        File::create(&rvc_path)
            .unwrap()
            .write_all(
                ron::ser::to_string_pretty(&self, PrettyConfig::new())
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
    }

    pub fn delete(&self) {
        let rvc_path = self.rvc_path();

        remove_file(rvc_path).unwrap()
    }

    pub fn checkout(&mut self, i: usize) {
        let mut files = HashMap::new();

        for c in &self.commits[0..=i] {
            // Created files
            for (p, d) in &c.created {
                files.insert(p.clone(), d.clone());
            }

            // Deleted files
            for d in &c.deleted {
                files.remove(d);
            }

            for m in &c.filemods {
                let file = files.get(&m.fpath).unwrap();
                *files.get_mut(&m.fpath).unwrap() = m.apply(file);
            }
        }

        self.checkout = files;
    }

    pub fn checkout_lastest(&mut self) {
        self.checkout(self.commits.len() - 1)
    }

    pub fn push_commit(&mut self, commit: Commit) {
        self.commits.push(commit);
    }
}

impl Display for Repository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Name: {}\nPath: {:?}\nCommits: {}",
            self.name,
            self.path,
            self.commits.len()
        ))
    }
}
