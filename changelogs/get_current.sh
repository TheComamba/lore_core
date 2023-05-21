#!/bin/bash

set -e

latest_tag=$(git describe --tags `git rev-list --tags --max-count=1`)
git_workspace=$(git rev-parse --show-toplevel)
cp $git_workspace/changelogs/$latest_tag.md $git_workspace/changelogs/current.md
