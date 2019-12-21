use std::path::Path;
use std::fs;
use std::process;
use std::process::Command;

mod branches;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(".git").exists() {
        println!("Not in a git repository, couldn't find .git directory.");
        process::exit(1);
    }

    // find all local branches
    let local_branches = branches::local::retrieve()?;
    println!("{:?}", local_branches);

    // get all of the remote branches
    let remote_branches = branches::remote::retrieve()?;
    println!("{:?}", remote_branches);

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
