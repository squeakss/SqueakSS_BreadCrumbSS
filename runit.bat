@echo off

:: Run the Python script
py inandout.py
if %ERRORLEVEL% NEQ 0 (
    echo Python script failed.
    exit /b 1
)

:: Run the Rust program
cargo run
if %ERRORLEVEL% NEQ 0 (
    echo Rust program failed.
    exit /b 1
)

echo Both scripts ran successfully.
