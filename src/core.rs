use std::{fs, io, path::PathBuf};

#[derive(Debug, Clone, Copy)]
pub enum RemoveMode {
    Files,
    FilesAndDirectories,
    Recursive,
}

pub fn remove_files(paths: Vec<PathBuf>) -> io::Result<()> {
    for path in paths {
        fs::remove_file(&path)?;
    }

    Ok(())
}

pub fn remove_files_and_directories(paths: Vec<PathBuf>) -> io::Result<()> {
    for path in paths {
        if fs::symlink_metadata(&path)?.is_dir() {
            fs::remove_dir(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
    }

    Ok(())
}

pub fn remove_recursively(paths: Vec<PathBuf>) -> io::Result<()> {
    for path in paths {
        if fs::symlink_metadata(&path)?.is_dir() {
            fs::remove_dir_all(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
    }

    Ok(())
}
