use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Path of directory file(s)
    pub args: Vec<PathBuf>,

    /// For printing out the total occurcenses across all files / directories
    #[clap(short = 'C', long = "count")]
    pub count: bool,

    /// Searching for specific token
    #[clap(short = 'S', long = "search")]
    pub search: Option<String>,
}
