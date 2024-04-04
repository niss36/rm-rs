use std::{
    env, io,
    path::{Path, PathBuf},
};

use path_absolutize::Absolutize;

fn contains_working_directory(path: &Path, working_directory: &Path) -> io::Result<bool> {
    let absolute_path = path.absolutize_from(working_directory)?;

    Ok(working_directory.starts_with(absolute_path))
}

fn get_paths_containing_working_directory(
    paths: &[PathBuf],
    working_directory: &Path,
) -> io::Result<Vec<PathBuf>> {
    let mut results = vec![];

    for path in paths {
        if contains_working_directory(path, working_directory)? {
            results.push(path.clone());
        }
    }

    Ok(results)
}

pub fn does_any_path_contain_working_directory(paths: &[PathBuf]) -> io::Result<bool> {
    let working_directory = env::current_dir()?;

    let offending_paths = get_paths_containing_working_directory(paths, &working_directory)?;

    if offending_paths.is_empty() {
        return Ok(false);
    }

    for path in offending_paths {
        eprintln!(
            "Cannot delete '{}': Contains the working directory",
            path.display()
        );
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn contains_working_directory_returns_true_if_equal() {
        let working_directory = env::current_dir().unwrap();

        let result = contains_working_directory(&working_directory, &working_directory).unwrap();

        assert!(result);
    }

    #[test]
    fn contains_working_directory_returns_true_for_dot() {
        let working_directory = env::current_dir().unwrap();

        let result = contains_working_directory(Path::new("."), &working_directory).unwrap();

        assert!(result);
    }

    #[test]
    fn contains_working_directory_returns_true_for_parent() {
        let working_directory = env::current_dir().unwrap();

        let result =
            contains_working_directory(Path::new("foo/../.."), &working_directory).unwrap();

        assert!(result);
    }

    #[test]
    fn contains_working_directory_returns_false_for_child() {
        let working_directory = env::current_dir().unwrap();

        let result = contains_working_directory(Path::new("./foo"), &working_directory).unwrap();

        assert!(!result);
    }

    #[test]
    fn contains_working_directory_returns_false_for_sibling() {
        let working_directory = env::current_dir().unwrap();

        let result = contains_working_directory(Path::new("../foo"), &working_directory).unwrap();

        assert!(!result);
    }
}
