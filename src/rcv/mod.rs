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

/// The `Rcv` structure describes the state of the version control program each time it's executed.
/// This state is loaded locally from a file called `.rcv` in the home directory.
/// Depending on the operating system, this file can be found under:
///
/// | Platform | Value                     | Example            |
/// | -------- | ------------------------- | ------------------ |
/// | Linux    | `$HOME/.rcv`              | /home/user/.rcv    |
/// | macOS    | `$HOME/.rcv`              | /Users//user/.rcv  |
/// | Windows  | `{FOLDERID_Profile}\.rcv` | C:\Users\user\.rcv |
///
/// If the file wasn't created by the time it is needed, it will be created automatically.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rcv {
    // #[serde(skip_serializing)]
    pub dirs: Dirs,
    // #[serde(skip)]
    pub changed_state: bool,
    pub repositories: Vec<Repository>,
}

impl Rcv {
    /// Creates an empty state given a set of directories provided by the OS.
    pub fn new(dirs: Dirs) -> Self {
        Rcv {
            dirs,
            changed_state: Default::default(),
            repositories: Default::default(),
        }
    }

    /// Returns the state saved locally.
    ///
    /// If the file doesn't exist, it creates it.
    pub fn retreive() -> Self {
        let dirs = Dirs::init().expect("No default directories found!");

        // Open and read the file
        let mut rcv_file = File::open(&dirs.rcv).unwrap();
        let mut str_buf = String::new();
        rcv_file.read_to_string(&mut str_buf).unwrap();

        // If there is any state over there, return it
        if let Ok(rcv) = ron::from_str(&str_buf) {
            rcv
        } else {
            Rcv::new(dirs)
        }
    }

    /// Attempts to create a new repository given a name and an optional path to it.
    ///
    /// If the path is omitted, the current working directory will be used as it's path.
    ///
    /// If the path used for the repository's creation is currently being occupied by other repository,
    /// the creation will be stopped.
    pub fn create_repository(&mut self, name: &str, path: &Option<String>) {
        // Choose the path if given or the default path
        let path = {
            if let Some(p) = path {
                Path::new(p).canonicalize().unwrap()
            } else {
                self.dirs.working_directory.clone()
            }
        };
        // If there was no repositories with the same working directory as the one you're in then continue
        if !self
            .repositories
            .clone()
            .into_iter()
            .find(|x| x.path == path)
            .is_some()
        {
            self.changed_state = true;
            self.repositories
                .push(Repository::new(name, path.to_str().unwrap()));

            println!(
                "The repository {} was created successfully\n-> {}",
                format!("{}", name.bright_green()),
                format!("{}", path.to_str().unwrap().bright_black())
            );
        }
        // There was a repository in the given directory
        else {
            println!(
                "{} There was a repository already created in this directory",
                format!("{}", "error:".bold().bright_red())
            )
        }
    }

    /// Attempts to delete a repository given a name.
    ///
    /// The given repository must exist for it to be deleted.
    ///
    /// TODO: Erase all the files within the repository directory recursively and itself.
    pub fn delete_repository(&mut self, name: &str) {
        // If it exists, delete it
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
                format!("{}", name.bright_red()),
            );
        } else {
            println!(
                "{} There wasn't a repository with name {}",
                format!("{}", "error:".bold().bright_red()),
                format!("{}", name.bright_red())
            );
        }
    }

    /// Saves the current state locally
    pub fn save(&self) {
        let mut rcv_file = File::create(self.dirs.rcv.clone()).unwrap();
        let serialized_state = ron::ser::to_string_pretty(&self, PrettyConfig::new()).unwrap();
        rcv_file.write_all(serialized_state.as_bytes()).unwrap();
    }

    /// Returns a copy of the repository that matches the currently working directory if any.
    pub fn current_repository(&self) -> Option<Repository> {
        self.repositories
            .clone()
            .into_iter()
            .find(|x| x.path == self.dirs.working_directory)
    }
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

        if let Some(current) = self.current_repository() {
            x.push_str(&format!(
                "\nCurrent: {}\n",
                format!("{}", current.name).bright_green()
            ));
        }

        // Git repositories
        if self.repositories.len() != 0 {
            x.push_str(&format!(
                "\n{}: ({})",
                "All Repositories".bright_yellow(),
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
