use std::process;
use std::path::Path;

/**
 * Remove the local references to the remote branches that are stale
 */
fn git_prune() -> Result<std::process::ExitStatus, std::io::Error>{
    process::Command::new("git")
        .args(&["remote", "prune", "origin"])
        .status()
}

fn main() {
    if !Path::new(".git").exists() {
        println!("Not in a git directory, couldn't find .git");
        process::exit(1);
    }

    println!("pruning current branches");
    if let Err(err) = git_prune() {
        println!("Error: {}", err);
        process::exit(1);
    }
}
