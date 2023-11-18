use clap::Parser;

#[derive(Parser)]
#[clap()]
#[derive(Debug)]
pub struct Args {
    pub args: Vec<String>,

    #[clap(short = 'C', long = "count")]
    pub count: bool,

    #[clap(short = 'S', long = "search")]
    pub search: Option<Vec<String>>,
}
