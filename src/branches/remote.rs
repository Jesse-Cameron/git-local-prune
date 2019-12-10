extern crate regex;
extern crate walkdir;

use std::error;
use std::result;
use regex::Regex;
use walkdir::{WalkDir, DirEntry};

type Result<T> = result::Result<T, Box<dyn error::Error>>;

fn is_not_head(entry: &DirEntry) -> bool {
    let is_master: bool = entry
        .file_name()
        .to_str()
        .map(|s| s.contains("master") || s.contains("HEAD"))
        .unwrap_or(false);

    (!is_master)
}

fn is_not_dir(entry: &DirEntry) -> bool {
    let is_dir: bool = entry
        .metadata()
        .map(|dir| dir.is_dir())
        .unwrap_or(false);

    (!is_dir)
}

fn remove_prefix(path: String) -> Option<String> {
    if path.len() == 0 {
        return None
    }

    let replaced_string = Regex::new(r#"^(.git/refs/remotes/origin/)(.*)"#)
        .ok()?
        .replace(&path, "$2")
        .into_owned();
    Some(replaced_string)
}

pub async fn retrieve() -> Result<Vec<String>> {
    let branches: Vec<String> = WalkDir::new(".git/refs/remotes/origin")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(is_not_dir)
        .filter(is_not_head)
        .map(|e| e.path().to_str().unwrap_or("").to_string())
        .map(remove_prefix)
        .filter_map(|line| line)
        .collect();

    Ok(branches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_removes_prefixes() {
        let test_str = String::from(".git/refs/remotes/origin/master");
        assert_eq!(remove_prefix(test_str), Some(String::from("master")));
    }

    #[test]
    fn correctly_leaves_prefixes() {
        let test_str = String::from(".git/refs/remotes/tracking/master");
        assert_eq!(remove_prefix(test_str), Some(String::from(".git/refs/remotes/tracking/master")))
    }
}
