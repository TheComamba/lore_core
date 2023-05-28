#!/bin/bash
set -e

sudo apt-get update
sudo apt-get install -y lua5.3 luarocks

luarocks install --server=https://luarocks.org/dev luaffi