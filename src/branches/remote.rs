extern crate regex;

use regex::Regex;
use std::fs;
use std::result;
use std::error;

type Result<T> = result::Result<T, Box<dyn error::Error>>;

fn ignored_branches(branch_name: &String) -> bool {
    (!(branch_name == "master") && !(branch_name == "HEAD"))
}

fn get_branch_name_from_line(re: Regex, line: &str) -> Option<String> {
    re.captures(line)?.get(1).map(|res| res.as_str().to_string())
}

pub fn retrieve() -> Result<Vec<String>> {
    let git_config = fs::read_to_string(".git/info/refs")?;
    let re = Regex::new(r#"^.*\trefs/remotes/origin/(.*)$"#)?;
    let branch_names: Vec<String> = git_config.lines()
        .map(|line| get_branch_name_from_line(re.clone(), line))
        .filter_map(|line| line) // remove any None objects from the list and return the Some value
        .filter(ignored_branches)
        .collect();

    Ok(branch_names)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignored_branches_invalid_master() {
        let test_str = "master";
        assert!(!ignored_branches(&String::from(test_str)));
    }

    #[test]
    fn ignored_branches_invalid_head() {
        let test_str = "HEAD";
        assert!(!ignored_branches(&String::from(test_str)));
    }

    #[test]
    fn ignored_branches_invalid() {
        let test_str = "feature-branch/master";
        assert!(ignored_branches(&String::from(test_str)));
    }

    #[test]
    fn regex_correctly_returns_value() {
        let test_str = "e144fd7196f05d33cd1007eaeb722cf465f8eed9	refs/remotes/origin/master"; // correct file line
        let re = Regex::new(r#"^.*\trefs/remotes/origin/(.*)$"#).unwrap();
        assert_eq!(get_branch_name_from_line(re, test_str), Some(String::from("master")))
    }

    #[test]
    fn regex_correctly_returns_none() {
        let test_str = r#"[branch "master"]"#; // incorrect file line
        let re = Regex::new(r#"^.*\trefs/remotes/origin/(.*)$"#).unwrap();
        assert_eq!(get_branch_name_from_line(re, test_str), None)
    }

}
