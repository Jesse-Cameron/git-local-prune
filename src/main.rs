extern crate walkdir;
extern crate regex;

use std::path::Path;
use std::process;
use std::fs;
use regex::Regex;

use walkdir::{WalkDir, DirEntry};

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

fn git_get_refs() ->  Vec<String> {
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

fn white_listed_branch(branch_name: &&str) -> bool {
    (!branch_name.contains("master"))
}

/**
 * Find branches that are tracking a remote
 */
fn get_local() -> Vec<String> {
    let git_config = fs::read_to_string(".git/config")
        .unwrap();
    let re = Regex::new(r#"^\[branch "([^"]*)"]$"#).unwrap();
    
    let branch_names: Vec<String> = git_config.lines()
        .map(|line| {
            let trimmed_line: &str = line.trim();
            let branch_capture = re.captures(trimmed_line);
            match branch_capture {
                None => None,
                Some(captures) => Some(captures.get(1).unwrap().as_str())
            }
        })
        .filter_map(|line| line) // remove any None objects from the list and return tge Some value
        .filter(white_listed_branch)
        .map(|line| line.to_string())
        .collect();

    (branch_names)
}

fn main() {
    if !Path::new(".git").exists() {
        println!("Not in a git directory, couldn't find .git");
        process::exit(1);
    }

    // println!("pruning current branches");
    // if let Err(err) = git_prune() {
    //     println!("Error: {}", err);
    //     process::exit(1);
    // }

    // steps
    // find all local branches
    // find the branches that are tracking a remote
    let local_branches = get_local();
    println!("{:?}", local_branches);
    // get all of the remote branches
    let remote_branches = git_get_refs();
    println!("{:?}", remote_branches);
    // find the subset of branches that are tracking a remote that no long exist
    //
    // delete those branches
}
