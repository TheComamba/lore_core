#!/bin/bash
set -e

rust_rfd_req=libgtk-3-dev
diesel_req1=sqlite3
diesel_req2=libsqlite3-dev

sudo apt-get update
sudo apt-get install -y $rust_rfd_req $diesel_req1 $diesel_req2

cargo --locked install diesel_cli --no-default-features --features sqlite