extern crate regex;

use std::fs;
use regex::Regex;
use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct BranchError;

impl fmt::Display for BranchError {
    fn fmt(&self, f: &mut fmt::Formatter) ->  fmt::Result {
        write!(f, "unable to fetch branch information")
    }
}

impl error::Error for BranchError {
    fn description(&self) -> &str {
        "unable to fetch branch information"
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}


fn ignored_branches(branch_name: &&str) -> bool {
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
        .filter(ignored_branches)
        .map(|line| line.to_string())
        .collect();

    (branch_names)
}

pub fn get_current() -> Result<String, Box<dyn error::Error>> {
    let git_file = fs::read_to_string(".git/HEAD")?;
    let re = Regex::new("^ref: refs/heads/(.*)")?;
    let file_line = git_file.lines().next().ok_or(BranchError)?;
    let branch_capture = re.captures(file_line).ok_or(BranchError)?;
    let branch_name = branch_capture.get(1).ok_or(BranchError)?;

    Ok(branch_name.as_str().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use std::str;

    #[test]
    fn ignored_branches_valid() {
        let test_str = "origin/master";
        assert!(!ignored_branches(&test_str));
    }

    #[test]
    fn ignored_branches_invalid() {
        let test_str = "origin/develop";
        assert!(ignored_branches(&test_str));
    }

    #[test]
    fn gets_current_branch() {
        // get the current branch from the OS
        let output = Command::new("git")
            .args(&["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .unwrap()
            .stdout;
        let expected = str::from_utf8(&output).unwrap().trim();
        let actual = get_current().unwrap();
        assert_eq!(expected, actual)
    }
}
