#!/bin/bash
set -e

# Go to the root directory of the Git repository
cd "$(git rev-parse --show-toplevel)"

if ! command -v cbindgen &> /dev/null; then
    cargo install cbindgen
fi

cbindgen --config cbindgen.toml --output lorecore_api.h
