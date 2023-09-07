#!/bin/bash
set -e

cargo --locked install diesel_cli --no-default-features --features sqlite-bundled