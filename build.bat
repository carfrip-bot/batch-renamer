@echo off
rem Helper: builds with the Rust toolchain.
rem TC must point to the "bin" folder of your Rust installation.
rem Default rustup location: %USERPROFILE%\.cargo\bin
rem If Rust is installed elsewhere, change the path below.
set TC=%USERPROFILE%\.cargo\bin

if not exist "%TC%\cargo.exe" (
    echo cargo.exe not found in "%TC%".
    echo Install Rust from https://rustup.rs or edit the TC line in build.bat.
    exit /b 1
)

set PATH=%TC%;%PATH%
set RUSTC=%TC%\rustc.exe

rem Always work from the folder that contains this script and Cargo.toml.
cd /d "%~dp0"

rem No arguments: make an optimized release build.
if "%~1"=="" (
    "%TC%\cargo.exe" build --release
) else (
    "%TC%\cargo.exe" %*
)