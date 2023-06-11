#!/bin/bash
set -e
cd $(git rev-parse --show-toplevel)

dir=changelogs
latest_tag=$(git describe --tags `git rev-list --tags --max-count=1`)
filepath=$dir/$latest_tag.md
if [ -z "$filepath" ]; then
    echo "[No changelog found]" > $dir/current.md
else
    cp $filepath $dir/current.md
fi
