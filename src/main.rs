use std::collections::HashMap;
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

    count_words(content)
        .iter()
        .for_each(|t| println!("{}: {}", t.0, t.1));
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

fn count_words(content: String) -> Vec<(String, u32)> {
    let content: String = content
        .to_lowercase()
        .chars()
        .filter(|c| c == &' ' || c == &'\n' || c == &'\'' || c.is_alphabetic())
        .collect();

    let content: Vec<&str> = content.split_whitespace().collect();
    let mut count: HashMap<String, u32> = HashMap::new();

    for word in content {
        let i = count.get(word).unwrap_or(&0);
        count.insert(word.to_owned(), i + 1);
    }

    let mut entries: Vec<(String, u32)> = count
        .iter()
        .map(|t| (t.0.to_owned(), t.1.to_owned()))
        .collect();

    entries.sort_by(|a, b| b.1.cmp(&a.1));

    entries
}
