#!/bin/bash
set -e

rust_rfd_req=libgtk-3-dev

sudo apt-get update
sudo apt-get install -y $rust_rfd_req

if ! command -v cargo &> /dev/null
then
    echo Installing rust...
    sudo apt-get install -y curl
    sudo curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi
