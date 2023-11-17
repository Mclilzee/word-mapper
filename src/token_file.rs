use std::{fs::read_to_string, os, path::PathBuf};

pub struct TokenFile {
    name: PathBuf,
    tokens: Vec<(String, usize)>,
}

impl TokenFile {
    fn from(path: PathBuf) -> Option<TokenFile> {
        if let Ok(content) = read_to_string(&path) {
            return Some(TokenFile {
                name: path,
                tokens: Vec::new(),
            });
        }
        return None;
    }
}
