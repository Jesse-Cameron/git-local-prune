extern crate regex;

use regex::Regex;
use std::io::{self,BufRead,Write};
use std::fs::File;
use std::collections::HashMap;

pub fn get_path(filename: &String) -> String {
    let local_branch_dir: String = String::from("./.git/refs/heads/");
    format!("{}{}", local_branch_dir, filename)
}

fn get_branch_name_from_line(re: Regex, line: &str) -> Option<String> {
    re.captures(line)?.get(1).map(|res| res.as_str().to_string())
}

fn read_branches() -> HashMap<String, String> {
    let file = File::open(".git/info/refs").unwrap();
    let reader = io::BufReader::new(file);
    
    reader.lines()
        .filter_map(|line| line.ok())
        .fold(HashMap::new(), |mut acc, branch| {
            let re = Regex::new(r#"^.*\trefs/remotes/origin/(.*)$"#).unwrap();
            let branch_name = get_branch_name_from_line(re, &branch.as_str()).unwrap();
            acc.insert(branch_name, branch);
            acc
        })
}

fn remove_branches(branch_map: HashMap<String, String>, branches_to_delete: Vec<String>) -> Vec<String> {
    let mut cloned_branches = branch_map.clone();
    for branch in &branches_to_delete {
        cloned_branches.remove(branch);
    }
    cloned_branches.values().map(|x| x.clone()).collect()
}

pub fn delete_branches(branch_names: Vec<String>) {
    let branch_map = read_branches();
    let remaining_branches = remove_branches(branch_map, branch_names);
    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_gets_path() {
        let filename = String::from("feat/branchname");
        assert_eq!(get_path(&filename), "./.git/refs/heads/feat/branchname");
    }
}