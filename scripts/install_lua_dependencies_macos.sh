#!/bin/bash
set -e

brew update
brew install lua@5.1
brew install luarocks

luarocks install --lua-dir=/usr/local/opt/lua@5.1 --server=https://luarocks.org/dev luaffi
