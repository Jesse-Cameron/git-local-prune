use std::path::Path;
use std::process;
use std::fs;
use std::error;
use std::process::Command;
use futures::{executor, try_join};

mod branches;

async fn get_branches() -> Result<(Vec<String>, Vec<String>), Box<dyn error::Error>> {
    // find all local branches
    let get_local_fut = branches::local::retrieve();

    // get all of the remote branches
    let get_remote_fut = branches::remote::retrieve();

    try_join!(get_local_fut, get_remote_fut)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(".git").exists() {
        println!("Not in a git repository, couldn't find .git directory.");
        process::exit(1);
    }

    let (local_branches, remote_branches) = executor::block_on(get_branches())?;

    // find the subset of branches that are tracking a remote that no long exist
    // as in, they are in the in the local but not the remote
    let orphaned_branches = branches::diff::find_orphaned(local_branches, remote_branches);
    if orphaned_branches.len() <= 0 {
        println!("No local branches to prune.");
        process::exit(0);
    }

    // get the current branch and see if it up for deletion
    let current_branch = branches::local::get_current()?;
    if orphaned_branches.contains(&current_branch) {
        let status = Command::new("git")
            .args(&["checkout", "master"])
            .status()
            .expect("failed to execute git checkout");
        if !status.success() {
            process::exit(1);
        }
    }

    // delete those branches
    for branch in &orphaned_branches {
        let branch_path = branches::delete::get_path(branch);
        fs::remove_file(branch_path)?;
        println!("Deleted Branch: {:?}", branch);
    }
    Ok(())
}
