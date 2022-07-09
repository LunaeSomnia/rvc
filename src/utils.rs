use std::{
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

pub fn load_file(path: &PathBuf) -> String {
    let mut read = read_to_string(path).unwrap();
    if read.len() == 0 || read.chars().last().unwrap() != '\n' {
        read.push('\n');
    }
    read
}

fn w(path: PathBuf, level: usize, files: &mut Vec<PathBuf>) {
    for dir_entry in read_dir(&path).unwrap() {
        let dir_entry = dir_entry.unwrap();
        let file_name = dir_entry.file_name();

        if dir_entry.file_type().unwrap().is_dir() {
            w(path.clone().join(file_name), level + 1, files);
        } else {
            files.push(dir_entry.path())
        }
    }
}

pub fn walk_dir(dir: PathBuf) -> Option<Vec<PathBuf>> {
    if dir.is_dir() {
        let mut files = Vec::new();
        w(dir, 0, &mut files);

        Some(files)
    } else {
        None
    }
}
