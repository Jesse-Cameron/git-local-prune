use std::process;
use std::path::Path;

fn main() {

    if !Path::new(".git").exists() {
        println!("Not in a git directory, couldn't find .git");
        process::exit(1);
    }
}
