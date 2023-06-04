#!/bin/bash
set -e

version=5.3
brew update
brew install lua@$version luarocks

luarocks install --lua-dir=/usr/local/opt/lua@$version --server=https://luarocks.org/dev luaffi

echo Checking installation...
find . -name ffitest.lua -exec lua {} \;
