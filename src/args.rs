use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap()]
#[derive(Debug)]
pub struct Args {
    pub args: Vec<PathBuf>,

    #[clap(short = 'C', long = "count")]
    pub count: bool,

    #[clap(short = 'S', long = "search")]
    pub search: Option<String>,
}
