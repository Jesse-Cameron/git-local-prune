#!/usr/bin/env bash

# the utility will remove branches where the remote track has been deleted
testDeletesCorrectBranches () {
  ./setup.sh > /dev/null 2>&1
  cd local || fail
  old_branches=($(git branch | awk -F ' +' '! /\(no branch\)/ {print $2}')) # get all local branches without the leading * or (no branch)
  assertEquals "${#old_branches[@]}" 11 # there should be two branches to begin with
  git fetch --all --prune > /dev/null 2>&1
  ../../../target/debug/git-local-prune
  new_branches=($(git branch | awk -F ' +' '! /\(no branch\)/ {print $2}')) # get all local branches without the leading * or (no branch)
  assertEquals "${#new_branches[@]}" 1 # there should be only one branch remaining
  assertEquals "${new_branches[0]}" "master" # it should leave the master
}

# the utility will not remove branches where the remote still exists
leaveValidRemoteBranches () {
  assertEquals 1 1
}

# utility will not remove branches where it is not tracking a remote
leaveValidLocalBranches () {
  assertEquals 1 1
}

. ./shunit2.sh