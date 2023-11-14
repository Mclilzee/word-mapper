use std::fs::read_dir;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        return;
    };

    let filePath = match args.get(1) {
        Some(path) => path,
        None => {
            eprint!("Error reading directory");
            return;
        }
    };
}
