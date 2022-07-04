use std::{
    env::current_dir,
    fs::File,
    path::{Path, PathBuf},
};

use directories::UserDirs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Dirs {
    pub working_directory: PathBuf,
    pub rcv: PathBuf,
}

impl Dirs {
    pub fn init() -> Option<Self> {
        if let Some(user_dirs) = UserDirs::new() {
            let working_directory = current_dir().unwrap();
            let rcv_path = {
                let home_directory = user_dirs.home_dir();
                home_directory.join(Path::new(".rcv"))
            };

            if !rcv_path.exists() {
                File::create(&rcv_path).unwrap();
            }

            Some(Dirs {
                working_directory,

                rcv: rcv_path,
            })
        } else {
            None
        }
    }
}
