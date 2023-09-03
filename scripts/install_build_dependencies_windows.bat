@echo off

vcpkg install sqlite3:x64-windows

cargo install diesel_cli --no-default-features --features sqlite
