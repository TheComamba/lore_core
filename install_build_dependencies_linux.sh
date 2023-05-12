#!/bin/bash
set -e

if ! command -v cargo &> /dev/null
then
    echo Installing rust...
    sudo curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi

rust_rfd_req=libgtk-3-dev

sudo apt install $rust_rfd_req