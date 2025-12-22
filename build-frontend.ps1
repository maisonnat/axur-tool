# Axur Web Frontend Build Script
# This script builds and serves the frontend with a separate target directory
# to avoid Windows file lock conflicts with rust-analyzer

param(
    [switch]$Build,    # Build only (no serve)
    [switch]$Serve,    # Build and serve
    [switch]$Clean,    # Clean target directory first
    [int]$Port = 8080  # Port for trunk serve
)

$ErrorActionPreference = "Stop"
$FrontendDir = "c:\Users\maiso\.gemini\antigravity\playground\azimuthal-opportunity\axur-web\crates\frontend"
$TargetDir = "c:\Users\maiso\.gemini\antigravity\playground\azimuthal-opportunity\axur-web\target-wasm"

Write-Host "======================================" -ForegroundColor Cyan
Write-Host "  Axur Web Frontend Build Script" -ForegroundColor Cyan
Write-Host "======================================" -ForegroundColor Cyan
Write-Host ""

# Set the custom target directory to avoid conflicts with rust-analyzer
$env:CARGO_TARGET_DIR = $TargetDir
Write-Host "[INFO] Using target directory: $TargetDir" -ForegroundColor Yellow

# Critical Fix for Windows File Locks (os error 32)
# Disable incremental compilation and force single-threaded build
$env:CARGO_INCREMENTAL = "0"
$env:CARGO_BUILD_JOBS = "1"
$env:RUSTFLAGS = "-C codegen-units=1"
Write-Host "[INFO] Robustness: Incremental=0, Jobs=1, CodegenUnits=1" -ForegroundColor Yellow

# Navigate to frontend directory
Set-Location $FrontendDir
Write-Host "[INFO] Working in: $FrontendDir" -ForegroundColor Yellow
Write-Host ""

# Clean if requested
if ($Clean) {
    Write-Host "[CLEAN] Removing target directory..." -ForegroundColor Magenta
    if (Test-Path $TargetDir) {
        Remove-Item -Recurse -Force $TargetDir
        Write-Host "[CLEAN] Target directory removed." -ForegroundColor Green
    }
    else {
        Write-Host "[CLEAN] Target directory doesn't exist, skipping." -ForegroundColor Gray
    }
    Write-Host ""
}

# Verify trunk is installed
try {
    $trunkVersion = trunk --version
    Write-Host "[OK] Trunk found: $trunkVersion" -ForegroundColor Green
}
catch {
    Write-Host "[ERROR] Trunk not found. Install with: cargo install trunk --locked" -ForegroundColor Red
    exit 1
}

# Verify wasm target is installed
$wasmTarget = rustup target list --installed | Select-String "wasm32-unknown-unknown"
if ($wasmTarget) {
    Write-Host "[OK] wasm32-unknown-unknown target installed" -ForegroundColor Green
}
else {
    Write-Host "[WARN] wasm32-unknown-unknown not found. Installing..." -ForegroundColor Yellow
    rustup target add wasm32-unknown-unknown
}

Write-Host ""

if ($Build -or $Serve) {
    Write-Host "[BUILD] Starting WASM build..." -ForegroundColor Cyan
    Write-Host "  This may take a few minutes on first build." -ForegroundColor Gray
    Write-Host ""
    
    if ($Serve) {
        Write-Host "[SERVE] Starting trunk server on port $Port..." -ForegroundColor Cyan
        Write-Host "  Frontend: http://localhost:$Port" -ForegroundColor White
        Write-Host "  Backend:  http://localhost:3001" -ForegroundColor White
        Write-Host ""
        Write-Host "  Press Ctrl+C to stop the server." -ForegroundColor Gray
        Write-Host ""
        trunk serve --port $Port
    }
    else {
        trunk build
        Write-Host ""
        Write-Host "[SUCCESS] Build complete! Output in: $FrontendDir\dist" -ForegroundColor Green
    }
}
else {
    Write-Host "Usage:" -ForegroundColor White
    Write-Host "  .\build-frontend.ps1 -Serve         # Build and serve locally" -ForegroundColor Gray
    Write-Host "  .\build-frontend.ps1 -Build         # Build only (for deployment)" -ForegroundColor Gray
    Write-Host "  .\build-frontend.ps1 -Serve -Clean  # Clean build and serve" -ForegroundColor Gray
    Write-Host "  .\build-frontend.ps1 -Serve -Port 3000  # Use different port" -ForegroundColor Gray
    Write-Host ""
    Write-Host "This script uses a separate target directory (target-wasm)" -ForegroundColor Yellow
    Write-Host "to avoid conflicts with rust-analyzer file locks." -ForegroundColor Yellow
}
