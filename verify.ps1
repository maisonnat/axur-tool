# Verification Script (The Golden Command)
# Usage: .\verify.ps1
#
# This script enforces the Definition of Done:
# 1. Formatting (cargo fmt)
# 2. Linting (cargo clippy)
# 3. Tests (cargo test)
# 4. Safety Audit (no unwrap in backend)

$ErrorActionPreference = "Stop"

function Write-Step {
    param($Message)
    Write-Host "👉 $Message..." -ForegroundColor Cyan
}

function Write-Success {
    param($Message)
    Write-Host "✅ $Message" -ForegroundColor Green
}

function Write-Error-Msg {
    param($Message)
    Write-Host "❌ $Message" -ForegroundColor Red
    exit 1
}

Write-Host "🛡️  Starting Axur Web Verification Protocol (Praefectus Standard)" -ForegroundColor Magenta
Write-Host ""

# 1. Format Check
Write-Step "Checking Code Formatting"
cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) { Write-Error-Msg "Formatting check failed. Run 'cargo fmt --all' to fix." }
Write-Success "Formatting Clear"

# 2. Safety Audit (No Unwrap)
Write-Step "Auditing Safe Code (No unwrap in backend)"
# Search for unwrap() in backend src, excluding test files
$matches = Select-String -Path "crates\backend\src\*.rs" -Pattern "\.unwrap\(\)" -Exclude "*test*"
# Filter out known safe/test files if any (naive check for now)
if ($matches) {
    Write-Host "⚠️  Found potential unsafe unwrap() calls:" -ForegroundColor Yellow
    $matches | ForEach-Object { Write-Host "   $($_.Filename):$($_.LineNumber) $($_.Line)" }
    # Intentionally not failing yet until Refactor task is done, but warning loudly
    Write-Host "   [NOTE] This will become a hard failure after the 'No Panic' refactor." -ForegroundColor Gray
} else {
    Write-Success "Safety Audit Clear"
}

# 3. Linting (Clippy)
Write-Step "Running Linter (Clippy)"
cargo clippy --workspace -- -D warnings
if ($LASTEXITCODE -ne 0) { Write-Error-Msg "Clippy check failed." }
Write-Success "Linter Clear"

# 4. Tests
Write-Step "Running Unit Tests"
cargo test --workspace
if ($LASTEXITCODE -ne 0) { Write-Error-Msg "Tests failed." }
Write-Success "All Tests Passed"

# 5. Documentation Check (Simple existence check)
if (!(Test-Path "knowledge\LESSONS_LEARNED.md")) { Write-Error-Msg "Missing LESSONS_LEARNED.md" }
if (!(Test-Path ".agent\rules\00_CONSTITUTION.md")) { Write-Error-Msg "Missing 00_CONSTITUTION.md" }
Write-Success "Governance Artifacts Present"

Write-Host ""
Write-Host "🎉 VERIFICATION SUCCESSFUL - READY TO COMMIT" -ForegroundColor Green
