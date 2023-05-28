#!/bin/bash
set -e

brew update
brew install lua@5.1 luarocks@2.4

luarocks install --server=https://luarocks.org/dev luaffi