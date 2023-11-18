mod args;
mod token_file;

use args::Args;
use clap::Parser;
use std::collections::HashMap;
use std::process::exit;
use std::{fs::read_dir, path::PathBuf};
use token_file::TokenFile;

fn main() {
    let config = Args::parse();
    if config.args.is_empty() {
        eprintln!("No files were provided");
        exit(1);
    }

    let token_files = config
        .args
        .iter()
        .map(|p| p.into())
        .flat_map(extract_paths)
        .flat_map(TokenFile::from_path);

    let token_files: Vec<TokenFile> = match config.search {
        Some(search) => token_files
            .map(|f| {
                let filtered = f
                    .tokens
                    .into_iter()
                    .filter(|t| t.0.contains(&search))
                    .collect();
                TokenFile::from(f.name, filtered)
            })
            .collect(),
        None => token_files.collect(),
    };

    if config.count {
        print_token_summary(token_files);
    } else {
        print_token_files(token_files);
    }
}

fn extract_paths(path: PathBuf) -> Vec<PathBuf> {
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
        .flat_map(extract_paths)
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
