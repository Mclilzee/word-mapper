mod token_file;

use std::path::Path;
use std::{fs::read_dir, path::PathBuf};
use token_file::TokenFile;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprint!("No files were provided");
        return;
    }

    let print_summary = args.len() >= 2 && args.get(0).unwrap() == "-S";
    let token_files = args
        .iter()
        .skip(if print_summary { 1 } else { 0 })
        .map(|x| Path::new(x).to_path_buf())
        .flat_map(extract_files)
        .flat_map(TokenFile::from)
        .collect();

    if print_summary {
        print_token_summary(token_files);
    } else {
        print_token_files(token_files);
    }
}

fn print_token_files(files: Vec<TokenFile>) {
    files.iter().for_each(|f| {
        println!("{}", f.name);
        f.tokens
            .iter()
            .for_each(|t| println!("---- {}: {}", t.0, t.1));
    });
}

fn print_token_summary(files: Vec<TokenFile>) {
    files
        .iter()
        .map(|f| &f.tokens)
        .flat_map(sum_count)
        .for_each(|f| println!("{}: {}", f.0, f.1));
}

fn sum_count(token_file: &Vec<(String, usize)>) -> Vec<(String, usize)> {
    return vec![];
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
