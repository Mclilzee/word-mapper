mod args;
mod token_file;

use args::Args;
use clap::Parser;
use std::collections::HashMap;
use std::fmt::Write;
use std::process::exit;
use std::{fs::read_dir, path::PathBuf};
use token_file::{Token, TokenFile};

fn main() {
    let config = Args::parse();
    if config.path.is_empty() {
        eprintln!("No files were provided");
        exit(1);
    }

    let token_files = config
        .path
        .iter()
        .map(|p| p.into())
        .flat_map(extract_paths)
        .flat_map(TokenFile::from_path)
        .collect();

    // let token_files: Vec<TokenFile> = match config.search {
    //     Some(search) => token_files
    //         .map(|f| {
    //             let filtered = f
    //                 .tokens
    //                 .into_iter()
    //                 .filter(|t| t.0.to_lowercase().contains(&search.to_lowercase()))
    //                 .collect();
    //             TokenFile::from(f.name, filtered)
    //         })
    //         .collect(),
    //     None => token_files.collect(),
    // };

    if config.overall {
        print_token_summary(token_files);
    } else {
        strings_per_file(token_files)
            .iter()
            .for_each(|t| println!("{t}"));
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

fn strings_per_file(token_files: Vec<TokenFile>) -> Vec<String> {
    token_files
        .iter()
        .map(|f| {
            f.tokens.iter().fold(String::new(), |mut str, t| {
                let _ = writeln!(
                    str,
                    "{}: {} ==== {:.3}% <-- {}",
                    t.symbol, t.occurence, t.frequency, f.name
                );
                str
            })
        })
        .collect()
}

fn print_token_summary(token_files: Vec<TokenFile>) {
    let total_occurences: usize = token_files
        .iter()
        .map(|t| &t.tokens)
        .map(|t| t.iter().map(|t| t.occurence).sum::<usize>())
        .sum();

    let mut tokens: Vec<Token> = token_files
        .iter()
        .map(|t| &t.tokens)
        .fold(HashMap::new(), |map, vec| {
            vec.iter().fold(map, |mut map, token| {
                let count = map.get(&token.symbol).unwrap_or(&0) + token.occurence;
                map.insert(token.symbol.to_owned(), count);
                map
            })
        })
        .iter()
        .map(|t| Token {
            symbol: t.0.to_owned(),
            occurence: t.1.to_owned(),
            frequency: (t.1.to_owned() as f32 / total_occurences as f32) * 100.0,
        })
        .collect();

    tokens.sort_by(|a, b| a.occurence.cmp(&b.occurence));
    tokens
        .iter()
        .for_each(|t| println!("{}: {} ==== {:.3}%", t.symbol, t.occurence, t.frequency));
}
