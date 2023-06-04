#!/bin/bash
set -e

# Lua5.3 clashes with luaffi
version=5.1
sudo apt-get update
sudo apt-get install -y lua$version liblua$version-dev luarocks

luarocks install --local --server=https://luarocks.org/dev luaffi

echo Checking installation...
path=$(find . -name ffitest.lua)
echo "Executing $path" 
lua "$path"
