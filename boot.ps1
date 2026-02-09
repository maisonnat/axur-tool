# boot.ps1 - ADHD-friendly session start
# Run at the beginning of every work session

Write-Host "`n🚀 AXUR BOOT SEQUENCE" -ForegroundColor Cyan
Write-Host ("=" * 40) -ForegroundColor DarkGray

# 1. Show Dashboard
Write-Host "`n📊 DASHBOARD:" -ForegroundColor Yellow
Get-Content "$PSScriptRoot\.agent\memory\DASHBOARD.md" | Select-Object -First 25

# 2. Quick Health Check
Write-Host "`n🚦 SERVICES:" -ForegroundColor Yellow
$backend = Test-NetConnection -ComputerName localhost -Port 3001 -WarningAction SilentlyContinue -ErrorAction SilentlyContinue
$frontend = Test-NetConnection -ComputerName localhost -Port 8080 -WarningAction SilentlyContinue -ErrorAction SilentlyContinue

Write-Host "  Backend (3001):  $(if($backend.TcpTestSucceeded){'✅ UP'}else{'❌ DOWN'})"
Write-Host "  Frontend (8080): $(if($frontend.TcpTestSucceeded){'✅ UP'}else{'❌ DOWN'})"

# 3. Show FOCUS NOW
Write-Host "`n🎯 FOCUS NOW:" -ForegroundColor Green
$focus = (Get-Content "$PSScriptRoot\.agent\memory\DASHBOARD.md" | Select-String -Pattern "^>" | Select-Object -First 1).Line
if ($focus) { Write-Host "  $focus" -ForegroundColor White }

Write-Host "`n✅ Boot complete. Ready to work!`n" -ForegroundColor Green
