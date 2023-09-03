@echo off

choco install sqlite

cargo install diesel_cli --no-default-features --features sqlite
