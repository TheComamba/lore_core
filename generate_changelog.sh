#!/bin/bash
set -e

changelog_tmp_file="changelog_tmp.md"

rm -f $changelog_tmp_file

lastTag=$(git describe --abbrev=0 --tags HEAD^)
currentTag=$(git describe --abbrev=0 --tags HEAD)
date=$(date +%Y-%m-%d)

echo "## [$currentTag] - $date" >> $changelog_tmp_file
echo "" >> $changelog_tmp_file
git log $lastTag..$currentTag --pretty=format:"%h %s" >> $changelog_tmp_file
echo "" >> $changelog_tmp_file
