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

fn is_directory(path: &Path) -> io::Result<bool> {
    Ok(fs::symlink_metadata(path)?.is_dir())
}

fn remove_file(path: &Path) -> io::Result<()> {
    if is_directory(path)? {
        Err(io::Error::new(ErrorKind::Other, "Is a directory"))
    } else {
        fs::remove_file(path)
    }
}

fn remove_file_or_directory(path: &Path) -> io::Result<()> {
    if is_directory(path)? {
        fs::remove_dir(path)
    } else {
        fs::remove_file(path)
    }
}

fn remove_recursively(path: &Path) -> io::Result<()> {
    if is_directory(path)? {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}

fn remove_aux(
    paths: Vec<PathBuf>,
    ignore_not_found: bool,
    operation: impl Fn(&Path) -> io::Result<()>,
) -> Result<(), ()> {
    let mut result = Ok(());

    for path in paths {
        if let Err(error) = operation(&path) {
            if ignore_not_found && matches!(error.kind(), ErrorKind::NotFound) {
                continue;
            }

            result = Err(());
            eprintln!("{}: {}", path.display(), error);
        }
    }

    result
}

pub fn remove(mode: RemoveMode, paths: Vec<PathBuf>, ignore_not_found: bool) -> Result<(), ()> {
    use RemoveMode as Mode;

    match mode {
        Mode::Files => remove_aux(paths, ignore_not_found, remove_file),
        Mode::FilesAndDirectories => remove_aux(paths, ignore_not_found, remove_file_or_directory),
        Mode::Recursive => remove_aux(paths, ignore_not_found, remove_recursively),
    }
}
