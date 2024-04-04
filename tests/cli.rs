use std::{
    env,
    error::Error,
    fs::{self, File},
    io,
    path::PathBuf,
    process::Command,
};

use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};
use predicates::prelude::predicate;

const BINARY_NAME: &str = env!("CARGO_PKG_NAME");

struct TempDirectory {
    path: PathBuf,
}

impl TempDirectory {
    fn new(test_name: &str) -> io::Result<Self> {
        let mut path = env::temp_dir();
        path.push(BINARY_NAME);
        path.push(test_name);

        fs::create_dir_all(&path)?;

        Ok(Self { path })
    }
}

impl Drop for TempDirectory {
    fn drop(&mut self) {
        if let Err(err) = fs::remove_dir_all(&self.path) {
            eprintln!(
                "Warning: failed to clean up temporary directory '{}' due to {}",
                self.path.display(),
                err
            );
        }
    }
}

#[test]
fn delete_current_dir_fails() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg(".");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Cannot delete '.': Contains the working directory",
    ));

    Ok(())
}

#[test]
fn delete_non_existent_file_fails() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    #[cfg(target_os = "windows")]
    let expected_message = "The system cannot find the path specified";

    #[cfg(not(target_os = "windows"))]
    let expected_message = "No such file or directory";

    cmd.arg("tests/__tmp__/file");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "tests/__tmp__/file: {}",
            expected_message
        )));

    Ok(())
}

#[test]
fn force_delete_non_existent_file_succeeds() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("-f").arg("tests/__tmp__/file");
    cmd.assert().success();

    Ok(())
}

#[test]
fn delete_file_succeeds() -> Result<(), Box<dyn Error>> {
    let temp_dir = TempDirectory::new("delete_file_succeeds")?;

    let file_path = temp_dir.path.join("file");

    File::create(&file_path)?;

    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg(&file_path);
    cmd.assert().success();

    assert!(!file_path.exists());

    Ok(())
}

#[test]
fn delete_directory_fails() -> Result<(), Box<dyn Error>> {
    let temp_dir = TempDirectory::new("delete_directory_fails")?;

    let dir_path = temp_dir.path.join("dir");

    fs::create_dir(&dir_path)?;

    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg(&dir_path);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "{}: Is a directory",
            dir_path.display()
        )));

    assert!(dir_path.exists());

    Ok(())
}

#[test]
fn delete_directory_with_d_succeeds() -> Result<(), Box<dyn Error>> {
    let temp_dir = TempDirectory::new("delete_directory_with_d_succeeds")?;

    let dir_path = temp_dir.path.join("dir");

    fs::create_dir(&dir_path)?;

    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("-d").arg(&dir_path);
    cmd.assert().success();

    assert!(!dir_path.exists());

    Ok(())
}

#[test]
fn delete_non_empty_directory_with_d_fails() -> Result<(), Box<dyn Error>> {
    let temp_dir = TempDirectory::new("delete_non_empty_directory_with_d_fails")?;

    let dir_path = temp_dir.path.join("dir");
    let file_path = dir_path.join("file");

    fs::create_dir(&dir_path)?;
    File::create(&file_path)?;

    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    #[cfg(target_os = "windows")]
    let expected_message = "The directory is not empty";

    #[cfg(not(target_os = "windows"))]
    let expected_message = "Directory not empty";

    cmd.arg("-d").arg(&dir_path);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "{}: {}",
            dir_path.display(),
            expected_message
        )));

    assert!(dir_path.exists());
    assert!(file_path.exists());

    Ok(())
}

#[test]
fn delete_non_empty_directory_with_r_succeeds() -> Result<(), Box<dyn Error>> {
    let temp_dir = TempDirectory::new("delete_non_empty_directory_with_r_succeeds")?;

    let dir_path = temp_dir.path.join("dir");
    let file_path = dir_path.join("file");

    fs::create_dir(&dir_path)?;
    File::create(&file_path)?;

    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("-r").arg(&dir_path);
    cmd.assert().success();

    assert!(!dir_path.exists());
    assert!(!file_path.exists());

    Ok(())
}
