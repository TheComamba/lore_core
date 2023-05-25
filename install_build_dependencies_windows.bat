
if not exist %SystemRoot%\System32\choco.exe (
    @powershell -NoProfile -ExecutionPolicy Bypass -Command "iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))" && SET "PATH=%PATH%;%ALLUSERSPROFILE%\chocolatey\bin"
)

choco install sqlite -y

setx PATH "%PATH%;C:\ProgramData\chocolatey\lib\SQLite\tools" /M