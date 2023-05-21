#!/bin/bash

set -e

latest_tag=$(git describe --tags `git rev-list --tags --max-count=1`)
filepath=$(dirname $(find . -name $latest_tag.md))
cp $filepath/$latest_tag.md $filepath/current.md
