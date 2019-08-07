extern crate regex;
use std::path::Path;
use std::process;
use std::fs;
use regex::Regex;

/**
 * Find branches that are tracking a remote
 */
fn get_branches() -> Vec<String> {
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
        .filter_map(|line| line)
        .map(|line| line.to_string())
        .collect();

    (branch_names)
}

/**
 * Remove the local references to the remote branches that are stale
 */
fn git_prune() -> Result<std::process::ExitStatus, std::io::Error> {
    process::Command::new("git")
        .args(&["remote", "prune", "origin"])
        .status()
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
    let local_branches = get_branches();
    // find the subset of branches that are tracking a remote that no long exist
    // delete those branches
}
