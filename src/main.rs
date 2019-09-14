extern crate walkdir;
extern crate regex;

use std::path::Path;
use std::process;

mod branches;

fn main() {
    if !Path::new(".git").exists() {
        println!("Not in a git directory, couldn't find .git");
        process::exit(1);
    }

    // println!("pruning current branches");
    // if let Err(err) = git_prune() {
    //     println!("Error: {}", err);
    //     process::exit(1);
    // }

    // steps
    // find all local branches
    // find the branches that are tracking a remote
    let local_branches = branches::local::retrieve();
    println!("{:?}", local_branches);
    // get all of the remote branches
    let remote_branches = branches::remote::retrieve();
    println!("{:?}", remote_branches);
    // find the subset of branches that are tracking a remote that no long exist
    //
    // delete those branches
}
