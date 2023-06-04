#!/bin/bash
set -e

sudo apt-get update
sudo apt-get install -y lua5.2 luarocks

# luainc=$(pkg-config --cflags lua)
export LUA_INCDIR=/usr/lib/:/usr/local/lib/:$luainc

ls -R /usr/lib/
ls -R /usr/local/lib/

luarocks install --local --server=https://luarocks.org/dev luaffi

echo Checking installation...
lua ffitest.lua
