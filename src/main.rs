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

    count_words(content).iter();
    // .for_each(|t| println!("{}: {}", t.0, t.1));
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
    let chars = content.chars().collect();
    let tokens: Vec<String> = extract_tokens(&chars);
    let mut count: HashMap<String, u32> = HashMap::new();

    for word in tokens {
        let i = count.get(&word).unwrap_or(&0);
        count.insert(word.to_owned(), i + 1);
    }

    let mut entries: Vec<(String, u32)> = count
        .iter()
        .map(|t| (t.0.to_owned(), t.1.to_owned()))
        .collect();

    entries.sort_by(|a, b| a.1.cmp(&b.1));

    entries
}

fn extract_tokens(chars: &Vec<char>) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    println!("{:?}", chars);
    let mut start_index = 0;
    let mut end_index = 1;
    for char in chars {
        if char.is_whitespace() && chars[start_index].is_alphabetic() {
            let str: String = chars[start_index..end_index].iter().collect();
            tokens.push(str);
            start_index = end_index;
            end_index += 1;
        } else if char.is_whitespace() {
            start_index += 1;
            end_index += 1;
        } else if char.is_alphabetic() || chars[start_index].is_alphabetic() && char.is_numeric() {
            end_index += 1;
        } else {
            let str: String = chars[start_index..end_index].iter().collect();
            tokens.push(str);
            start_index = end_index;
            end_index += 1;
        }
    }

    tokens
}
