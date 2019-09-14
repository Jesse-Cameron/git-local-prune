extern crate regex;

use std::fs;
use regex::Regex;

fn white_listed_branch(branch_name: &&str) -> bool {
    (!branch_name.contains("master"))
}

/**
 * Find branches that are tracking a remote
 */
pub fn retrieve() -> Vec<String> {
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
