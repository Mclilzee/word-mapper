use std::collections::HashMap;
use std::{fs::read_to_string, path::PathBuf};

#[derive(Debug)]
pub struct TokenFile {
    pub name: String,
    pub tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct Token {
    pub symbol: String,
    pub occurence: usize,
    pub frequency: f32,
}

impl TokenFile {
    pub fn from_path(path: PathBuf) -> Option<Self> {
        let name = path.to_str().unwrap_or("Unknown").to_owned();
        let content = read_to_string(&path);
        if content.is_err() {
            return None;
        };

        let content = content.expect(&format!(
            "Error: Failed reading content after reading file {name}."
        ));

        let symbols = extract_tokens(content);

        let tokens = count_tokens(symbols);
        let total_occurences: usize = tokens.iter().map(|t| t.1).sum();

        let tokens = tokens
            .iter()
            .map(|t| Token {
                symbol: t.0,
                occurence: t.1,
                frequency: (t.1 as f32 / total_occurences as f32) * 100.0,
            })
            .collect();

        Some(TokenFile { name, tokens })
    }
}

fn extract_tokens(str: String) -> Vec<String> {
    let chars: Vec<char> = str.chars().collect();

    let mut tokens: Vec<String> = Vec::new();
    let mut index = 0;

    while index < chars.len() {
        if chars[index].is_whitespace() {
            index += 1;
        } else if chars[index].is_numeric() {
            let str: String = chars
                .iter()
                .skip(index)
                .take_while(|c| c.is_numeric())
                .collect();
            index += str.len();
            tokens.push(str);
        } else if chars[index].is_alphabetic() {
            let str: String = chars
                .iter()
                .skip(index)
                .take_while(|c| c.is_alphanumeric())
                .collect();
            index += str.len();
            tokens.push(str);
        } else {
            let str: String = chars.iter().skip(index).take(1).collect();
            tokens.push(str);
            index += 1;
        }
    }

    tokens
}

fn count_tokens(tokens: Vec<String>) -> Vec<(String, usize)> {
    let mut count: HashMap<String, usize> = HashMap::new();

    for word in tokens {
        let i = count.get(&word).unwrap_or(&0);
        count.insert(word.to_owned(), i + 1);
    }

    let mut entries: Vec<(String, usize)> = count
        .iter()
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .collect();

    entries.sort_by(|a, b| a.1.cmp(&b.1));

    entries
}
