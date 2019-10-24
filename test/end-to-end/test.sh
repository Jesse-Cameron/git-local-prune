#!/usr/bin/env bats

CURRENT_DIR="$(pwd)/test/end-to-end"

# the utility will remove branches where the remote track has been deleted
@test "deletes correct branches" {
  # create and delete 10 branches
  "$CURRENT_DIR"/setup.sh 10 10
  cd "$CURRENT_DIR"/local || fail
  old_branches=($(git branch | awk -F ' +' '! /\(no branch\)/ {print $2}')) # get all local branches without the leading * or (no branch)
  [ "${#old_branches[@]}" -eq 11 ] # there should be ten branches to begin with
  git fetch --all --prune
  "$CURRENT_DIR"/../../target/debug/git-local-prune
  new_branches=($(git branch | awk -F ' +' '! /\(no branch\)/ {print $2}')) # get all remaining local branches without the leading * or (no branch)
  [ "${#new_branches[@]}" -eq 1 ] # there should be only one branch remaining
  [ "${new_branches[0]}" = "master" ] # it should leave the master
}

# the utility will not remove branches where the remote still exists
@test "leaves valid remote branches" {
  # create 10 branches, but only delete 8
  "$CURRENT_DIR"/setup.sh 10 10 8
  cd "$CURRENT_DIR"/local || fail
  old_branches=($(git branch | awk -F ' +' '! /\(no branch\)/ {print $2}')) # get all local branches without the leading * or (no branch)
  [ "${#old_branches[@]}" -eq 11 ] # there should be ten branches to begin with
  git fetch --all --prune
  "$CURRENT_DIR"/../../target/debug/git-local-prune
  new_branches=($(git branch | awk -F ' +' '! /\(no branch\)/ {print $2}')) # get all remaining local branches without the leading * or (no branch)
  printf '%s,' "${new_branches[@]}"
  [ "${#new_branches[@]}" -eq 3 ] # there should be only one branch remaining
  [ "${new_branches[0]}" = "branch_10" ]
  [ "${new_branches[1]}" = "branch_9" ]
  [ "${new_branches[2]}" = "master" ]
}

# utility will not remove branches where it is not tracking a remote
@test 'leave valid local branches' {
  # create 12 local branches, remove 10 remote
  "$CURRENT_DIR"/setup.sh 10 12
  cd "$CURRENT_DIR"/local || fail
  old_branches=($(git branch | awk -F ' +' '! /\(no branch\)/ {print $2}')) # get all local branches without the leading * or (no branch)
  [ "${#old_branches[@]}" -eq 13 ] # there should be ten branches to begin with
  git fetch --all --prune
  "$CURRENT_DIR"/../../target/debug/git-local-prune
  new_branches=($(git branch | awk -F ' +' '! /\(no branch\)/ {print $2}')) # get all remaining local branches without the leading * or (no branch)
  [ "${#new_branches[@]}" -eq 3 ] # there should be only one branch remaining
  [ "${new_branches[0]}" = "branch_11" ]
  [ "${new_branches[1]}" = "branch_12" ]
  [ "${new_branches[2]}" = "master" ]
}
