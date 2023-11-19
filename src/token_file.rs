use std::collections::HashMap;
use std::{fs::read_to_string, path::PathBuf};

#[derive(Debug)]
pub struct TokenFile {
    pub name: String,
    pub tokens: Vec<(String, usize)>,
}

impl TokenFile {
    pub fn from_path(path: PathBuf) -> Option<Self> {
        let name = path.to_str().unwrap_or("Unknown").to_owned();
        let content = read_to_string(&path);
        if content.is_err() {
            return None;
        };

        let tokens = extract_tokens(content.unwrap());
        let tokens = count_tokens(tokens);

        Some(TokenFile { name, tokens })
    }

    pub fn from(name: String, tokens: Vec<(String, usize)>) -> TokenFile {
        TokenFile { name, tokens }
    }

    pub fn frequency(&self) -> Vec<(&String, f32)> {
        let total_occurences: usize = self.tokens.iter().map(|f| f.1).sum();
        println!("Sum is: {total_occurences}");

        return self
            .tokens
            .iter()
            .map(|t| (&t.0, (t.1 * 100 / total_occurences) as f32))
            .collect();
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
