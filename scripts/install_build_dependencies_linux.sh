#!/bin/bash
set -e

rust_rfd_req=libgtk-3-dev

sudo apt-get update
sudo apt-get install -y "$rust_rfd_req"

cargo install diesel_cli --no-default-features --features sqlite