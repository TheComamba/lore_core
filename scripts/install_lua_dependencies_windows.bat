choco install -y lua --version=5.1.5.52
choco install -y luarocks --version=2.4.4
choco install -y mingw

luarocks install --server=https://luarocks.org/dev luaffi