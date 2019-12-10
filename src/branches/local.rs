extern crate regex;

use std::fs;
use regex::Regex;
use std::error;
use std::fmt;
use std::result;

type Result<T> = result::Result<T, Box<dyn error::Error>>;

// Custom error for building 
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

fn ignored_branches(branch_name: &String) -> bool {
    (!branch_name.contains("master"))
}

fn get_branch_name_from_line(re: Regex, line: &str) -> Option<String> {
    re.captures(line)?.get(1).map(|v| v.as_str().to_string())
}

/**
 * Find branches that are tracking a remote
 */
pub async fn retrieve() -> Result<Vec<String>> {
    let git_config = fs::read_to_string(".git/config")?;
    let re = Regex::new(r#"^\[branch "([^"]*)"]$"#)?;
    
    let branch_names: Vec<String> = git_config.lines()
        .map(|line| get_branch_name_from_line(re.clone(), line))
        .filter_map(|line| line) // remove any None objects from the list and return the Some value
        .filter(ignored_branches)
        .collect();

    Ok(branch_names)
}

pub fn get_current() -> Result<String> {
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
        assert!(!ignored_branches(&String::from(test_str)));
    }

    #[test]
    fn ignored_branches_invalid() {
        let test_str = "origin/develop";
        assert!(ignored_branches(&String::from(test_str)));
    }

    #[test]
    fn regex_correctly_returns_value() {
        let test_str = r#"[branch "master"]"#; // correct file line
        let re = Regex::new(r#"^\[branch "([^"]*)"]$"#).unwrap();
        assert_eq!(get_branch_name_from_line(re, test_str), Some(String::from("master")))
    }

    #[test]
    fn regex_correctly_returns_none() {
        let test_str = "	remote = origin"; // incorrect file line
        let re = Regex::new(r#"^\[branch "([^"]*)"]$"#).unwrap();
        assert_eq!(get_branch_name_from_line(re, test_str), None)
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
