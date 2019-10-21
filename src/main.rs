use std::path::Path;
use std::process;
use std::fs;

mod branches;

fn main() {
    if !Path::new(".git").exists() {
        println!("Not in a git directory, couldn't find .git");
        process::exit(1);
    }

    // find all local branches
    let local_branches = branches::local::retrieve();
    // get all of the remote branches
    let remote_branches = branches::remote::retrieve();
    // find the subset of branches that are tracking a remote that no long exist
    // as in, they are in the in the local but not the remote
    let orphaned_branches = branches::diff::find_orphaned(local_branches, remote_branches);
    if orphaned_branches.len() <= 0 {
        println!("Nothing to delete.");
        process::exit(0);
    }
    
    // delete those branches
    for branch in &orphaned_branches {
        let branch_path = branches::delete::get_path(branch);
        match fs::remove_file(branch_path) {
            Ok(_) => println!("Deleted Branch: {:?}", branch),
            Err(e) => println!("Error Deleting Branch: {:?}. Error: {}", branch, e)
        }
    }
}
