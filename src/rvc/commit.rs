use std::{collections::HashMap, fmt::Display, path::PathBuf};

use chrono::Utc;
use colored::Colorize;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::utils::{load_file, walk_dir};

use super::repository::Repository;

pub fn str_diff(s1: &str, s2: &str) -> Vec<diff::Result<String>> {
    diff::lines(s1, s2)
        .into_iter()
        .map(|x| match x {
            diff::Result::Left(l) => diff::Result::Left(l.to_owned()),
            diff::Result::Both(l, _) => diff::Result::Both(l.to_owned(), l.to_owned()),
            diff::Result::Right(r) => diff::Result::Right(r.to_owned()),
        })
        .collect()
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug, Default)]
pub struct Commit {
    pub name: String,
    pub timestamp: i64,
    pub filemods: Vec<FileMod>,
    pub created: HashMap<PathBuf, String>,
    pub deleted: Vec<PathBuf>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug, Default)]
pub struct FileMod {
    pub fpath: PathBuf,
    pub changes: Vec<FileChange>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug, Default, PartialOrd)]
pub struct FileChange {
    pub line: usize,
    pub change: Change,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug, Default, PartialOrd)]
pub enum Change {
    Added(String),
    #[default]
    Deleted,
}

impl Commit {
    pub fn from_data(data: HashMap<PathBuf, String>) -> Commit {
        Commit {
            timestamp: Utc::now().timestamp(),
            created: data,
            ..Default::default()
        }
    }
    pub fn create(name: &str, repo: &mut Repository) -> Result<Self, String> {
        let initial_files: Vec<PathBuf> = repo.checkout.keys().map(|f| f.clone()).collect();
        let current_files = walk_dir(repo.path.clone()).unwrap();

        let mut possibly_modified = Vec::new();

        let mut added_files = HashMap::new();
        for cf in &current_files {
            if *cf != repo.rvc_path() {
                if !initial_files.contains(&cf) {
                    added_files.insert(cf.clone(), load_file(&cf));
                } else {
                    possibly_modified.push(cf.clone())
                }
            }
        }

        let mut deleted_files = Vec::new();
        for f in &initial_files {
            if *f != repo.rvc_path() {
                if !current_files.contains(&f) && !added_files.keys().find(|p| p == &f).is_some() {
                    deleted_files.push(f.clone());
                }
            }
        }

        // Check for possibly modified files
        let mut modified = Vec::new();
        for pm in &possibly_modified {
            if *pm != repo.rvc_path() {
                let diffs = str_diff(repo.checkout.get(pm).unwrap(), &load_file(&pm));
                let diffs = FileMod::from_changes(pm.to_path_buf(), diffs);
                if !diffs.is_empty() {
                    modified.push(diffs);
                }
            }
        }

        if added_files.len() == 0 && deleted_files.len() == 0 && modified.len() == 0 {
            Err("There are no changes in the repository".to_owned())
        } else {
            Ok(Commit {
                name: String::from(name),
                timestamp: Utc::now().timestamp(),
                created: added_files,
                deleted: deleted_files,
                filemods: modified,
            })
        }
    }
}

impl FileMod {
    pub fn from_changes(fpath: PathBuf, changes: Vec<diff::Result<String>>) -> Self {
        let mut line = 1;
        let mut vch = Vec::new();
        for change in changes {
            match &change {
                diff::Result::Left(_l) => vch.push(FileChange {
                    line,
                    change: Change::Deleted,
                }), // Add
                diff::Result::Both(_, _) => (), // Nothing
                diff::Result::Right(r) => vch.push(FileChange {
                    line,
                    change: Change::Added(r.clone()),
                }), // Delete
            }
            line += 1;
        }

        FileMod {
            fpath,
            changes: vch,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.changes.len() == 0
    }

    pub fn apply(&self, str: &str) -> String {
        let mut m = self.clone();
        m.changes.sort_by(|a, b| b.line.cmp(&a.line));

        let mut line = 0;
        let mut lines: Vec<&str> = Regex::new("\n").unwrap().split(str).collect();

        for c in &m.changes {
            match &c.change {
                Change::Added(l) => {
                    lines.insert((line + c.line as i32 - 1) as usize, l);
                    line += 1;
                }
                Change::Deleted => {
                    lines.remove((line + c.line as i32 - 1) as usize);
                    line -= 1;
                }
            }
        }

        let mut f = String::new();
        for l in lines {
            f.push_str(&format!("{}\n", l));
        }

        f
    }
}

impl Ord for FileChange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.line.cmp(&other.line)
    }
}

impl Display for FileChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{} [{}] {}",
            self.line,
            match self.change {
                Change::Added(_) => "+",
                Change::Deleted => "-",
            },
            if let Change::Added(l) = &self.change {
                l
            } else {
                "---"
            }
        ))
    }
}

impl Display for Commit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for c in &self.created {
            s.push_str(&format!("{:?}\n", c.0.to_str().unwrap().on_bright_green()));
        }
        s.push('\n');

        for d in &self.deleted {
            s.push_str(&format!("{:?}\n", d.to_str().unwrap().on_bright_red()));
        }
        s.push('\n');

        for m in &self.filemods {
            s.push_str(&format!("{:?}\n", m.fpath));
            for c in &m.changes {
                s.push_str(&format!(
                    "{}",
                    match &c.change {
                        Change::Added(l) => format!(
                            "{}{} {}\n",
                            (c.line - 1).to_string().bold().on_bright_green(),
                            " +".bold().on_bright_green(),
                            l.bright_green()
                        ),
                        Change::Deleted => format!(
                            "{}{}\n",
                            (c.line - 1).to_string().bold().on_bright_red(),
                            " -".bold().on_bright_red()
                        ),
                    }
                ))
            }
        }
        s.push('\n');

        f.write_str(&s)
    }
}
