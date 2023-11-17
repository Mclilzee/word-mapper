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

    args.iter()
        .map(|x| Path::new(x).to_path_buf())
        .flat_map(extract_files)
        .flat_map(TokenFile::from)
        .for_each(|f| {
            println!("{}", f.name);
            f.tokens
                .iter()
                .for_each(|t| println!("\t\t{}: {}", t.0, t.1));
        });
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
