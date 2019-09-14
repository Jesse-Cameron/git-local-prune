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

fn remove_prefix(path: String) -> String {
    let re = Regex::new(r#"^(.git/refs/remotes/origin/)(.*)"#).unwrap();
    let replaced_string = re.replace(&path, "$2").into_owned();
    (replaced_string)
}

pub fn retrieve() ->  Vec<String> {
    let branches: Vec<String> = WalkDir::new(".git/refs/remotes/origin")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(is_not_dir)
        .filter(is_not_head)
        .map(|e| e.path().to_str().unwrap().to_string())
        .map(remove_prefix)
        .collect();

    (branches)
}

