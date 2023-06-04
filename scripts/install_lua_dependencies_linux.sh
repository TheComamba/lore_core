#!/bin/bash
set -e

version=5.3
sudo apt-get update
sudo apt-get install -y lua$version liblua$version-dev luarocks

luarocks install --local --server=https://luarocks.org/dev luaffi

echo Checking installation...
find . -name ffitest.lua -exec lua {} \;
