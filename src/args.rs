use clap::{command, Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "Prints the frequencies of words (tokens) that appears in provided files"
)]
pub struct Args {
    /// Path of directory or file(s)
    pub path: Vec<PathBuf>,

    /// For printing out the total occurcenses across all files / directories
    #[arg(short = 'C', long = "count")]
    pub count: bool,

    /// Searching for specific token
    #[arg(short = 'S', long = "search")]
    pub search: Option<String>,

    ///  Printing format (default, path, frequency, overall)
    /// -- path will include path printout for each occurence
    /// -- frequency will show frequency of items
    /// -- overall will count the overall data of all occurences across mutliple files
    #[arg(short = 'F', long = "format", value_enum)]
    pub format: Option<Format>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Format {}
