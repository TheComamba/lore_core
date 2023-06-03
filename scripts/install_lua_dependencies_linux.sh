#!/bin/bash
set -e

sudo apt-get update
sudo apt-get install -y lua5.1 luarocks

luainc=$(pkg-config --cflags lua5.1)
export LUA_INCDIR=$LUA_INCDIR:$luainc

luarocks install --local --server=https://luarocks.org/dev luaffi

# Test luaFFi
echo 'local ffi = require("ffi")' >ffitest.lua
echo 'ffi.cdef[[void Sleep(int ms); int poll(struct pollfd *fds, unsigned long nfds, int timeout);]]' >>ffitest.lua
echo 'return function(s) ffi.C.poll(nil, 0, s*1000) end' >>ffitest.lua
lua ffitest.lua
rm ffitest.lua
