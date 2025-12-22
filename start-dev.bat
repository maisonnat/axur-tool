@echo off
REM Axur Web - Start Both Backend and Frontend
REM Run this script from the axur-web directory

echo ======================================
echo   Axur Web - Development Start
echo ======================================
echo.

set AXUR_WEB_DIR=%~dp0
set CARGO_TARGET_DIR=%AXUR_WEB_DIR%target-wasm

echo [1/2] Starting Backend on port 3001...
start "Axur Backend" cmd /k "cd /d %AXUR_WEB_DIR% && cargo run -p axur-backend"

echo [2/2] Waiting 5 seconds for backend to start...
timeout /t 5 /nobreak > NUL

echo [2/2] Starting Frontend on port 8080...
REM Using single thread and no incremental to avoid file locks
start "Axur Frontend" cmd /k "cd /d %AXUR_WEB_DIR%crates\frontend && set CARGO_TARGET_DIR=%CARGO_TARGET_DIR% && set CARGO_INCREMENTAL=0 && set CARGO_BUILD_JOBS=1 && trunk serve --port 8080"

echo.
echo ======================================
echo   Axur Web Started!
echo ======================================
echo.
echo   Backend:  http://localhost:3001
echo   Frontend: http://localhost:8080
echo.
echo   Two terminal windows opened.
echo   Close them to stop the servers.
echo ======================================
pause
