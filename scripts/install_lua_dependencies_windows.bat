@echo off

choco install -y lua
choco install -y luarocks

luarocks install --server=https://luarocks.org/dev luaffi

echo Checking installation...
for /f usebackq %F in (`where /r . ffitest.lua`) do lua %F
