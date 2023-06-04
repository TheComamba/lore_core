@echo off

rem Lua51 download link is broken.
choco install -y lua52
refreshenv
choco install -y luarocks
refreshenv

luarocks install --server=https://luarocks.org/dev luaffi
refreshenv

echo Checking installation...
for /f usebackq %F in (`where /r . ffitest.lua`) do lua %F
