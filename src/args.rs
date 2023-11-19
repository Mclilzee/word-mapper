use std::path::PathBuf;

use clap::{command, Parser};

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "Prints the frequencies of words (tokens) that appears in provided files"
)]
pub struct Args {
    /// Path of directory or file(s)
    pub args: Vec<PathBuf>,

    /// For printing out the total occurcenses across all files / directories
    #[clap(short = 'C', long = "count")]
    pub count: bool,

    /// Searching for specific token
    #[clap(short = 'S', long = "search")]
    pub search: Option<String>,
}
