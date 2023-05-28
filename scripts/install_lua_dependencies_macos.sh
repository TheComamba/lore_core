#!/bin/bash
set -e

brew update
brew install lua@5.1
brew install luarocks

luarocks install --server=https://luarocks.org/dev luaffi