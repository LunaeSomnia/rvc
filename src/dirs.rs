use std::{
    env::current_dir,
    fmt::Display,
    fs::File,
    path::{Path, PathBuf},
};

use directories::UserDirs;

#[derive(Debug, Clone)]
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

impl Display for Dirs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Working directory: {}\nRcv file path: {} [{}]",
            self.working_directory.to_str().unwrap(),
            self.rcv.to_str().unwrap(),
            self.rcv.exists()
        ))
    }
}
