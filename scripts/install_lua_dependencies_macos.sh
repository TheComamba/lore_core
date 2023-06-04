#!/bin/bash
set -e

# Lua5.2 is not a brew package
# Lua5.3 clashes with luaffi
version=5.1
brew update
brew install lua@$version luarocks

luarocks install --lua-dir=/usr/local/opt/lua@$version --server=https://luarocks.org/dev luaffi

echo Checking installation...
path=$(find . -name ffitest.lua)
lua $path