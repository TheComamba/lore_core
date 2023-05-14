#!/bin/bash

# This script generates a changelog diff between two tags.
# It is used by the release process to generate the changelog for a release.

changelog_tmp_file="changelog_tmp.md"

rm -f $changelog_tmp_file

lastTag=$(git describe --abbrev=0 --tags HEAD^)
currentTag=$(git describe --abbrev=0 --tags HEAD)

diff=$(git diff $lastTag..$currentTag -- CHANGELOG.md)

if [ -z "$diff" ]
then
  echo "Error: Please update CHANGELOG.md before releasing. No changes since last tag."
  exit 1
else
  echo "Changes since last tag:"
  echo "$diff"
  echo "$diff" > changelog_tmp.md
fi
