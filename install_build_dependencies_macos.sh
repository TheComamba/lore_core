#!/bin/bash
set -e

brew update

if ! command -v cargo &> /dev/null
then
    echo Installing rust...
    brew install curl
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi
