use std::path::Path;
use std::{
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprint!("No files were provided");
        return;
    }

    let content: String = args
        .iter()
        .flat_map(extract_files)
        .map(|x| read_file_content(&x))
        .collect();

    print!("{content}");
}

fn extract_files(paths: &String) -> Vec<PathBuf> {
    Vec::new()
}

fn read_file_content(path: &PathBuf) -> String {
    if let Ok(content) = read_to_string(path) {
        content
    } else {
        "".to_owned()
    }
}
