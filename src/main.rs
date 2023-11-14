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
        .map(|x| Path::new(x).to_path_buf())
        .flat_map(extract_files)
        .map(read_file_content)
        .collect();

    print!("{content}");
}

fn extract_files(path: PathBuf) -> Vec<PathBuf> {
    if path.is_file() {
        return vec![path];
    }

    let files = match read_dir(path) {
        Ok(files) => files,
        Err(_) => return Vec::new(),
    };

    files
        .filter(|x| x.is_ok())
        .flatten()
        .map(|x| x.path())
        .flat_map(extract_files)
        .collect()
}

fn read_file_content(path: PathBuf) -> String {
    if let Ok(content) = read_to_string(path) {
        content
    } else {
        "".to_owned()
    }
}
