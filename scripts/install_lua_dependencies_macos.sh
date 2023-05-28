#!/bin/bash
set -e

brew update
brew install lua luarocks

luarocks install --server=https://luarocks.org/dev luaffi