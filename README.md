# git-local-prune

> ⚠️ **Note:** this tool is currently in beta. As such I can't guarantee that this won't nuke your git history. Use with care.

This is a utility to help with the cleaning up of any local copies of remote deleted branches.

Often a local repository will have checked out a branch that is tracking a remote. What happens when that remote branch is deleted? Typically, it is just orphaned locally. That's where `git-local-prune` comes in. Running this utility will delete any branches that have been abandoned.

### Building

- This is a rust project. So ensure you have rust and cargo installed. You can find instructions [here.](https://www.rust-lang.org/tools/install)
- run `cargo build`

### Usage

- Make sure you are in a `git` repository
- Update your remotes: `git fetch --prune`
- Make sure that you don't have any work in any deleted remotes locally that you need
- Execute: `./git-local-prune`

> ⚠️ **Note:** this tool alters the contents of the `.git` folder directly. So I would recommend running a [`git gc`](https://git-scm.com/docs/git-gc) afterwards. Fair warning, this is a destructive action.

### Testing

- unit tests: `cargo test`
- functional tests: `./test/end-to-end/test.sh` - this requires `bats` to be [installed locally](https://github.com/sstephenson/bats/wiki/Install-Bats-Using-a-Package)

### FAQ

- ***Q: Will this delete all my local branches?***  
Nope. Just the branches that previously tracked a remote. If you create an entirely fresh branch then run this utility. It will ignore this branch.
- ***Q: Does this not just make a bunch of `git` commands?***  
Nope. It will go straight to the relevant files in you `.git` directory and make the changes there. `git` is just distributed files and folders!
- ***Q: HECK, I just deleted something I should not have! What can I do?***  
Ouch! If you haven't run `git gc` or `git prune`, you **may** be able to dig old commits out using `git reflog`. Otherwise, your code probably was not that good anyway. 🤷
- ***Q: Why not use `<insert-custom-script-here>`?***  
Because Rust. gottagofast.
