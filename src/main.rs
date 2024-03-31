use core::{remove_files, remove_files_and_directories, remove_recursively, RemoveMode};
use std::{io, path::PathBuf};

use clap::Parser;

mod core;

#[derive(Debug, Parser)]
struct Cli {
    /// Attempt to remove directories as well as other types of files.
    #[clap(short)]
    directories: bool,

    /// Recursively remove directories and the files they contain. This implies the -d option
    #[clap(short)]
    recursive: bool,

    files: Vec<PathBuf>,
}

impl Cli {
    fn get_remove_mode(&self) -> RemoveMode {
        match (self.recursive, self.directories) {
            (false, false) => RemoveMode::Files,
            (false, true) => RemoveMode::FilesAndDirectories,
            (true, _) => RemoveMode::Recursive,
        }
    }
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let mode = args.get_remove_mode();

    match mode {
        RemoveMode::Files => remove_files(args.files),
        RemoveMode::FilesAndDirectories => remove_files_and_directories(args.files),
        RemoveMode::Recursive => remove_recursively(args.files),
    }
}
