Write-Host "Starting Axur Tool Development Environment" -ForegroundColor Cyan

$env:API_BASE_URL = "http://localhost:3001"

# Check backend
$backend = Get-Process -Name "axur-backend" -ErrorAction SilentlyContinue
if ($backend) {
    Write-Host "Backend already running." -ForegroundColor Yellow
} else {
    Write-Host "Starting Backend..." -ForegroundColor Green
    Start-Process -NoNewWindow -FilePath "cargo" -ArgumentList "run", "-p", "axur-backend", "--bin", "axur-backend" -WorkingDirectory $PSScriptRoot
    Start-Sleep -Seconds 2
}

Write-Host "Starting Frontend..." -ForegroundColor Green
$frontendPath = Join-Path $PSScriptRoot "crates\frontend"
Set-Location -Path $frontendPath
trunk serve
