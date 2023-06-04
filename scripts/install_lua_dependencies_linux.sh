#!/bin/bash
set -e

sudo apt-get update
sudo apt-get install -y lua5.2 luarocks

# luainc=$(pkg-config --cflags lua5.2)
# export LUA_INCDIR=$LUA_INCDIR:$luainc

luarocks install --local --server=https://luarocks.org/dev luaffi

echo Checking installation...
lua ffitest.lua
