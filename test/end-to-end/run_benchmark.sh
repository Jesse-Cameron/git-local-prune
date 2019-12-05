#!/usr/bin/env bash

# example
# ./run_benchmark.sh -n <number_of_branches> -c <command_to_delete>

set -ex

CURRENT_DIR="$(pwd)/test/end-to-end"

number_of_branches=0
command=''

while getopts "n:c:" opt; do
    case "$opt" in
    n)
        number_of_branches=${OPTARG%/} # remove trailing args
        ;;
    c)
        command=$OPTARG
        ;;
    esac
done

"$CURRENT_DIR"/setup.sh "$number_of_branches" "$number_of_branches" > /dev/null 2>&1
cd "$CURRENT_DIR"/local || fail
git fetch --all --prune
git checkout master
ts=$(date +%s%N)
eval $command # > /dev/null 2>&1
echo $((($(date +%s%N) - ts)/1000000))