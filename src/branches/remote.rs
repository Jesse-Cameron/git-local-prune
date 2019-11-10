extern crate regex;
extern crate walkdir;


use walkdir::{WalkDir, DirEntry};
use regex::Regex;

fn is_not_head(entry: &DirEntry) -> bool {
    let is_master: bool = entry
        .file_name()
        .to_str()
        .map(|s| s.contains("master") || s.contains("HEAD"))
        .unwrap_or_else(|| false);

    (!is_master)
}

fn is_not_dir(entry: &DirEntry) -> bool {
    let is_dir: bool = entry
        .metadata()
        .unwrap()
        .is_dir();

    (!is_dir)
}

fn remove_prefix(path: String) -> Option<String> {
    if path.len() == 0 {
        return None
    }

    let re;
    match Regex::new(r#"^(.git/refs/remotes/origin/)(.*)"#) {
        Ok(v) => re = v,
        Err(_) => return None
    }
    let replaced_string = re.replace(&path, "$2").into_owned();
    Some(replaced_string)
}

pub fn retrieve() ->  Vec<String> {
    let branches: Vec<String> = WalkDir::new(".git/refs/remotes/origin")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(is_not_dir)
        .filter(is_not_head)
        .map(|e| e.path().to_str().unwrap_or("").to_string())
        .map(remove_prefix)
        .filter_map(|line| line)
        .collect();

    (branches)
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
