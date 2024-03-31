use std::{
    fs,
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Copy)]
pub enum RemoveMode {
    Files,
    FilesAndDirectories,
    Recursive,
}

fn remove_file(path: &Path) -> io::Result<()> {
    if fs::symlink_metadata(path)?.is_dir() {
        Err(io::Error::new(ErrorKind::Other, "Is a directory"))
    } else {
        fs::remove_file(path)
    }
}

fn remove_file_or_directory(path: &Path) -> io::Result<()> {
    if fs::symlink_metadata(path)?.is_dir() {
        fs::remove_dir(path)
    } else {
        fs::remove_file(path)
    }
}

fn remove_recursively(path: &Path) -> io::Result<()> {
    if fs::symlink_metadata(&path)?.is_dir() {
        fs::remove_dir_all(&path)
    } else {
        fs::remove_file(&path)
    }
}

fn remove_aux(paths: Vec<PathBuf>, operation: impl Fn(&Path) -> io::Result<()>) -> Result<(), ()> {
    let mut result = Ok(());

    for path in paths {
        if let Err(error) = operation(&path) {
            result = Err(());
            eprintln!("{}: {}", path.display(), error);
        }
    }

    result
}

pub fn remove(mode: RemoveMode, paths: Vec<PathBuf>) -> Result<(), ()> {
    match mode {
        RemoveMode::Files => remove_aux(paths, remove_file),
        RemoveMode::FilesAndDirectories => remove_aux(paths, remove_file_or_directory),
        RemoveMode::Recursive => remove_aux(paths, remove_recursively),
    }
}
