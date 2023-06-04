#!/bin/bash
set -e

brew update
brew install lua@5.1
brew install luarocks

luarocks install --lua-dir=/usr/local/opt/lua@5.1 --server=https://luarocks.org/dev luaffi

export LUA_PATH=".luarocks/lib/lua/5.1/":$LUA_PATH

echo Checking installation...
lua ffitest.lua
