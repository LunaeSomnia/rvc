use std::{
    fs::File,
    io::{Read, Write},
};

use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

use crate::dirs::Dirs;

use self::repository::Repository;

pub mod repository;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rcv {
    pub changed_state: bool,
    pub repositories: Vec<Repository>,
}

impl Default for Rcv {
    fn default() -> Self {
        Self {
            changed_state: false,
            repositories: Default::default(),
        }
    }
}

impl Rcv {
    /// Retreives the current state of the RCV from a local file in the system
    pub fn retreive(dirs: &Dirs) -> Self {
        println!("RCV: Retreive");
        let mut rcv_file = File::open(&dirs.rcv).unwrap();
        let mut str_buf = String::new();
        rcv_file.read_to_string(&mut str_buf).unwrap();

        if let Ok(rcv) = ron::from_str(&str_buf) {
            rcv
        } else {
            Rcv::default()
        }
    }

    pub fn create_repository(&mut self, name: &str, dirs: &Dirs) {
        println!("COMMAND: Create a new repository with name '{}'", name);

        let path = dirs.working_directory.to_str().unwrap();

        // If there was no repositories with the same working directory as the one you're in... continue
        if !self
            .repositories
            .clone()
            .into_iter()
            .find(|x| x.path.to_str().unwrap() == path)
            .is_some()
        {
            self.changed_state = true;
            self.repositories.push(Repository::new(name, path))
        } else {
            println!("RCV: There was a repository already created in this directory")
        }
    }

    pub fn save(&self, dirs: &Dirs) {
        println!("RCV: Save");
        let mut rcv_file = File::create(&dirs.rcv).unwrap();
        let serialized_state = ron::ser::to_string_pretty(&self, PrettyConfig::new()).unwrap();
        rcv_file.write_all(serialized_state.as_bytes()).unwrap();
    }
}
