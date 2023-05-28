#!/bin/bash
set -e

sudo apt-get update
sudo apt-get install -y lua5.4 luarocks

luainc=$(pkg-config --cflags lua5.4)
export LUA_INCDIR=$LUA_INCDIR:$luainc

luarocks install --server=https://luarocks.org/dev luaffi