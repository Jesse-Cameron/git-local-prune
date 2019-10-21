#!/usr/bin/env bash

set -e

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
    git checkout --track origin/"branch_$i"
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
  rm -rf remote
  rm -rf local
}

run () {
  number_of_branches=10
  cleanup
  create_repo "remote"
  setup_remote_repo "$number_of_branches"
  create_repo "local"
  setup_local_repo "$number_of_branches"
  prune_remote_branches "$number_of_branches"
}

run
