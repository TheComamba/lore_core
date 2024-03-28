#!/bin/bash

# Navigate to the root of the git repository
cd "$(git rev-parse --show-toplevel)"

# Check if cargo-tarpaulin is installed
if ! cargo tarpaulin --version &> /dev/null
then
    echo "cargo-tarpaulin could not be found, please install it first using 'cargo install cargo-tarpaulin'"
    exit
fi

# Run tests with coverage
cargo tarpaulin
