use std::path::PathBuf;

use clap::Parser;

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

fn main() {
    let args = Cli::parse();

    println!("{args:#?}");
}
