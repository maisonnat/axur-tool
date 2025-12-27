# Axur Tool - Development Script
# Run this to start both backend and frontend for local development

Write-Host "🚀 Starting Axur Tool Development Environment" -ForegroundColor Cyan
Write-Host ""

# Configuration
$env:API_BASE_URL = "http://localhost:3001"

# Check if backend is already running
$backend = Get-Process -Name "axur-backend" -ErrorAction SilentlyContinue
if ($backend) {
    Write-Host "⚠️  Backend already running (PID: $($backend.Id))" -ForegroundColor Yellow
} else {
    Write-Host "📦 Starting Backend on http://localhost:3001..." -ForegroundColor Green
    Start-Process -NoNewWindow -FilePath "cargo" -ArgumentList "run", "-p", "axur-backend" -WorkingDirectory $PSScriptRoot
    Start-Sleep -Seconds 2
}

# Start frontend with correct API URL
Write-Host "🌐 Starting Frontend on http://localhost:8080..." -ForegroundColor Green
Write-Host "   (API_BASE_URL = $env:API_BASE_URL)" -ForegroundColor DarkGray
Write-Host ""
Write-Host "Press Ctrl+C to stop" -ForegroundColor DarkGray
Write-Host ""

Set-Location "$PSScriptRoot\crates\frontend"
trunk serve
