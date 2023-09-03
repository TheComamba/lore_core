#!/bin/bash
set -e

rust_rfd_req=libgtk-3-dev
diesel_req=sqlite3

sudo apt-get update
sudo apt-get install -y "$rust_rfd_req" "$diesel_req"

cargo --locked install diesel_cli --no-default-features --features sqlite