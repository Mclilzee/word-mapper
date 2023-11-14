use std::fs::{read_dir, read_to_string};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        eprint!("No files to read were provided");
        return;
    };

    let path = match args.get(1) {
        Some(path) => path,
        None => {
            eprint!("Error reading directory");
            return;
        }
    };

    let content = read_content(path);
    print!("{content}");
}

fn read_content(path: &str) -> String {
    if let Ok(dir) = read_dir(path) {
        return dir
            .filter_map(|x| x.ok())
            .map(|x| x.path())
            .map(|c| read_to_string(c))
            .filter_map(|x| x.ok())
            .collect();
    };

    return "".to_owned();
}
