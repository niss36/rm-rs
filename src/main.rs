use core::{remove, RemoveMode};
use std::{path::PathBuf, process::ExitCode};

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

    /// Ignore "file not found" errors
    #[clap(short = 'f')]
    ignore_not_found: bool,

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

fn main() -> ExitCode {
    let args = Cli::parse();

    let mode = args.get_remove_mode();

    let result = remove(mode, args.files, args.ignore_not_found);

    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
