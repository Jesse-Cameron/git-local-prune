#!/usr/bin/env bash

# example
# ./setup.sh <remote> <local> <remote_delete>

# TODO: have better validation around this
if [ -z "$1" ]; then
  number_of_remote_branches=0
else
  number_of_remote_branches=$1
fi

if [ -z "$2" ]; then
  number_of_local_branches=0
else
  number_of_local_branches=$2
fi

if [ -z "$3" ]; then
  number_of_remote_branches_to_delete=number_of_remote_branches
else
  number_of_remote_branches_to_delete=$3
fi

create_repo () {
  repo_name=$1
  if [ ! -d "$repo_name" ]; then
    mkdir "$repo_name"
  fi
  cd "$repo_name"
  git init
  git commit --allow-empty -m "initial commit"
  cd -
}

create_branch () {
  branch_name=$1
  git checkout -b "$branch_name"
  git commit --allow-empty -m "branch commit"
  git checkout master
}

setup_remote_repo () {
  n=$1
  cd remote
  # create testing branches
  for ((i=1; i<=n; i++))
  do
    create_branch "branch_$i"
  done
  cd -
}

setup_local_repo () {
  n=$1
  cd local
  git remote add origin ../remote/.git
  git fetch --all
  # start tracking all the remote branches
  for ((i=1; i<=n; i++))
  do
    git ls-remote origin | grep -c "branch_$i" # check if the remote exists
    ret=$?
    # checkout if it does exist
    if [ ! "$ret" -ne 0 ]; then
      git checkout -B "branch_$i" origin/"branch_$i"
    # create otherwise
    else
      git checkout master
      git branch "branch_$i"
    fi
  done
  cd -
}

prune_remote_branches () {
  n=$1
  cd remote
  for ((i=1; i<=n; i++))
  do
    git branch -D "branch_$i"
  done
}

cleanup () {
  rm -rf remote/
  rm -rf local/
}

run () {
  cleanup
  create_repo "remote"
  setup_remote_repo "$number_of_remote_branches"
  create_repo "local"
  setup_local_repo "$number_of_local_branches"
  prune_remote_branches "$number_of_remote_branches_to_delete"
}

run
