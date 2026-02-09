# handoff.ps1 - Quick session close
# Run at the end of every work session
param(
    [string]$NextFocus = ""
)

Write-Host "`n🏁 HANDOFF SEQUENCE" -ForegroundColor Yellow
Write-Host ("=" * 40) -ForegroundColor DarkGray

# 1. Check uncommitted work
Write-Host "`n📝 GIT STATUS:" -ForegroundColor Yellow
$gitStatus = git status --porcelain 2>$null
if ($gitStatus) {
    Write-Host "  ⚠️  UNCOMMITTED CHANGES:" -ForegroundColor Red
    $gitStatus | ForEach-Object { Write-Host "    $_" -ForegroundColor DarkYellow }
} else {
    Write-Host "  ✅ Working tree clean" -ForegroundColor Green
}

# 2. Update DASHBOARD timestamp
$dashboardPath = "$PSScriptRoot\.agent\memory\DASHBOARD.md"
$date = Get-Date -Format "yyyy-MM-dd HH:mm"
$content = Get-Content $dashboardPath -Raw

# Update timestamp
$content = $content -replace "Last updated: .*", "Last updated: $date"

# Update focus if provided
if ($NextFocus) {
    $content = $content -replace '(?<=🎯 FOCUS NOW\r?\n> \*\*).*(?=\*\*)', $NextFocus
    Write-Host "`n🎯 Next focus set to: $NextFocus" -ForegroundColor Cyan
}

$content | Set-Content $dashboardPath -NoNewline

# 3. Generate context with code2prompt (if available)
Write-Host "`n📦 CONTEXT GENERATION:" -ForegroundColor Yellow
$c2p = Get-Command code2prompt -ErrorAction SilentlyContinue
if ($c2p) {
    $templatePath = "$PSScriptRoot\.agent\templates\xml_packet.hbs"
    $outputPath = "$PSScriptRoot\.agent\memory\context_packet.xml"
    if (Test-Path $templatePath) {
        code2prompt . --template $templatePath --output $outputPath 2>$null
        if ($?) {
            Write-Host "  ✅ Context packet updated: $outputPath" -ForegroundColor Green
        } else {
            Write-Host "  ⚠️  Context generation failed" -ForegroundColor Yellow
        }
    } else {
        Write-Host "  ⏭️  Template not found, skipping" -ForegroundColor DarkGray
    }
} else {
    Write-Host "  ⏭️  code2prompt not installed, skipping" -ForegroundColor DarkGray
}

Write-Host "`n✅ Handoff complete. See you next time!`n" -ForegroundColor Green
