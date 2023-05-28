choco install -y lua --version=5.1.5.52
choco install -y luarocks --version=2.4.4

rem Add cl.exe to PATH
call "C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvars64.bat"

luarocks install --server=https://luarocks.org/dev luaffi