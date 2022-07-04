use std::{
    fmt::Display,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use colored::Colorize;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

use crate::dirs::Dirs;

use self::repository::Repository;

pub mod repository;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rcv {
    // #[serde(skip_serializing)]
    pub dirs: Dirs,
    // #[serde(skip)]
    pub changed_state: bool,
    pub repositories: Vec<Repository>,
}

impl Rcv {
    pub fn new(dirs: Dirs) -> Self {
        Rcv {
            dirs,
            changed_state: Default::default(),
            repositories: Default::default(),
        }
    }

    /// Retreives the current state of the RCV from a local file in the system
    pub fn retreive() -> Self {
        let dirs = Dirs::init().expect("No default directories found!");
        let mut rcv_file = File::open(&dirs.rcv).unwrap();
        let mut str_buf = String::new();
        rcv_file.read_to_string(&mut str_buf).unwrap();

        if let Ok(rcv) = ron::from_str(&str_buf) {
            rcv
        } else {
            Rcv::new(dirs)
        }
    }

    pub fn create_repository(&mut self, name: &str, path: &Option<String>) {
        let path = {
            if let Some(p) = path {
                Path::new(p).canonicalize().unwrap()
            } else {
                self.dirs.working_directory.clone()
            }
        };
        // If there was no repositories with the same working directory as the one you're in... continue
        if !self
            .repositories
            .clone()
            .into_iter()
            .find(|x| x.path.to_str().unwrap() == path.to_str().unwrap())
            .is_some()
        {
            self.changed_state = true;
            self.repositories
                .push(Repository::new(name, path.to_str().unwrap()));

            println!(
                "The repository {} was created successfully\n-> {}",
                format!("{}", "name".bright_green()),
                format!("{}", path.to_str().unwrap().bright_black())
            );
        } else {
            println!(
                "{} There was a repository already created in this directory",
                format!("{}", "error:".bold().bright_red())
            )
        }
    }

    pub fn delete_repository(&mut self, name: &str) {
        if let Some(i) = self
            .repositories
            .clone()
            .into_iter()
            .position(|r| r.name == name)
        {
            self.changed_state = true;
            self.repositories.remove(i);

            println!(
                "The repository {} was deleted successfully",
                format!("{}", "name".bright_red()),
            );
        } else {
            println!(
                "{} There wasn't a repository with name '{}'",
                format!("{}", "error:".bold().bright_red()),
                format!("{}", name.italic())
            );
        }
    }

    pub fn save(&self) {
        let mut rcv_file = File::create(self.dirs.rcv.clone()).unwrap();
        let serialized_state = ron::ser::to_string_pretty(&self, PrettyConfig::new()).unwrap();
        rcv_file.write_all(serialized_state.as_bytes()).unwrap();
    }

    pub fn current_repository(&self) {}
}

impl Display for Rcv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut x = String::new();

        // Header
        x.push_str(&format!(
            "{} {}\n",
            "rvc".green(),
            option_env!("CARGO_PKG_VERSION").unwrap()
        ));

        // Git repositories
        if self.repositories.len() != 0 {
            x.push_str(&format!(
                "\n{}: ({})",
                "Repositories".bright_yellow(),
                self.repositories.len()
            ));
            x.push_str("\n");
            for repo in &self.repositories {
                x.push_str(&format!(
                    " - {} {}\n",
                    format!("{}", repo.name).yellow(),
                    format!("{}", repo.path.to_str().unwrap()).bright_black()
                ))
            }
        } else {
            x.push_str(&format!("{}", "There are no repositories".red()));
        }

        f.write_str(&x)
    }
}
