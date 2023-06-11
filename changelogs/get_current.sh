#!/bin/bash

set -e

latest_tag=$(git describe --tags `git rev-list --tags --max-count=1`)
dir=$(dirname $(find . -name $latest_tag.md))
filepath=$dir/$latest_tag.md
if [ -z "$filepath" ]; then
    echo "[No changelog found]" > $dir/current.md
else
    cp $filepath $dir/current.md
fi
