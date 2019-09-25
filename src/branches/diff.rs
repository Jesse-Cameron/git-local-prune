use std::collections::HashSet;

pub fn find_orphaned(local: Vec<String>, remote: Vec<String>) -> Vec<String> {
    let remote_set = remote.into_iter().collect::<HashSet<String>>();
    
    let mut orphaned: Vec<String> = Vec::new();
    for local_branch in &local {
        if !remote_set.contains(local_branch) {
            orphaned.push(local_branch.clone());
        }
    }

    (orphaned)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_diffs_branches() {
        let local_branches = vec![String::from("branch_1"), String::from("branch_2")];
        let remote_branches = vec![String::from("branch_2"), String::from("branch_3")];
        assert_eq!(find_orphaned(local_branches, remote_branches), vec!["branch_1"]);
    }

    #[test]
    fn returns_empty_vec() {
        let local_branches = vec![String::from("branch_1"), String::from("branch_2")];
        let remote_branches = vec![String::from("branch_1"), String::from("branch_2")];
        let expected: Vec<String> = Vec::new();
        assert_eq!(find_orphaned(local_branches, remote_branches), expected);
    }
}