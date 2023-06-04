#!/bin/bash
set -e

version=5.2
sudo apt-get update
sudo apt-get install -y lua$version liblua$version-dev luarocks

# luainc=$(pkg-config --cflags lua)
# export LUA_INCDIR=/usr/lib/x86_64-linux-gnu/lua/

luarocks install --local --server=https://luarocks.org/dev luaffi

echo Checking installation...
lua ffitest.lua
