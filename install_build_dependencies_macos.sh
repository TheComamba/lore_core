#!/bin/bash
set -e

rust_rfd_req=libgtk-3-dev
rust_rusqlite_req=libsqlite3-dev

brew update
brew install $rust_rfd_req $rust_rusqlite_req

if ! command -v cargo &> /dev/null
then
    echo Installing rust...
    brew install curl
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi
