pub fn get_path(filename: &String) -> String {
    let local_branch_dir: String = String::from("./.git/refs/heads/");
    format!("{}{}", local_branch_dir, filename)
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