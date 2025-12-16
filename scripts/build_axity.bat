@echo off
setlocal
set SCRIPT_DIR=%~dp0
pushd "%SCRIPT_DIR%\.."
cargo build --release
if errorlevel 1 exit /b %errorlevel%
echo Built: "%CD%\target\release\axity.exe"
popd
