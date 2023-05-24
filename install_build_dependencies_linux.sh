#!/bin/bash
set -e

if ! command -v cargo &> /dev/null
then
    echo Installing rust...
    sudo curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi


rust_rfd_req=libgtk-3-dev
rust_rusqlite_req=libsqlite3-dev

sudo apt-get update
sudo apt-get install $rust_rfd_req $rust_rusqlite_req