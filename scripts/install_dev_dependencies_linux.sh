#!/bin/bash
set -e

sudo ./install_build_dependencies_linux.sh

cargo install diesel_cli --no-default-features --features sqlite