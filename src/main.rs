mod token_file;

use std::collections::HashMap;
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

fn print_token_files(files: Vec<TokenFile>) {
    files.iter().for_each(|f| {
        f.tokens
            .iter()
            .for_each(|t| println!("{}: {} <-- {}", t.0, t.1, f.name));
    });
}

fn print_token_summary(files: Vec<TokenFile>) {
    let mut sorted: Vec<(String, usize)> = files
        .iter()
        .map(|t| &t.tokens)
        .fold(HashMap::new(), |map, vec| {
            vec.iter().fold(map, |mut map, tuple| {
                let (token, count) = tuple;
                let count = map.get(token).unwrap_or(&0) + count;
                map.insert(token.to_owned(), count);
                map
            })
        })
        .iter()
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .collect();

    sorted.sort_by(|a, b| a.1.cmp(&b.1));
    sorted.iter().for_each(|(t, c)| println!("{t}: {c}"));
}
