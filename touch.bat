@echo off

rem Batch parameters: https://docs.microsoft.com/en-us/windows-server/administration/windows-commands/call#batch-parameters
set __dirname=%~dp0
powershell -File "%__dirname%\.devcontainer\local\touch.ps1" %*
