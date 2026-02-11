#!/usr/bin/env pwsh
# ============================================================
# Axur Web - Production Smoke Tests
# ============================================================
# Quick verify that frontend and backend are alive in production.
# Usage: .\smoke_test.ps1
# ============================================================

$ErrorActionPreference = "Continue"
$ProgressPreference = "SilentlyContinue"

# --- Configuration ---
$FrontendUrl  = "https://axtool.pages.dev"
$BackendBase  = "https://axur-backend-844146909418.us-central1.run.app"
$TimeoutSec   = 15

# --- Results ---
$results = @()

function Test-Endpoint {
    param(
        [string]$Name,
        [string]$Url,
        [int]$ExpectedStatus = 200,
        [scriptblock]$BodyCheck = $null
    )

    $result = [PSCustomObject]@{
        Test   = $Name
        URL    = $Url
        Status = "FAIL"
        Code   = "-"
        Detail = ""
    }

    try {
        $resp = Invoke-WebRequest -Uri $Url -UseBasicParsing -TimeoutSec $TimeoutSec -ErrorAction Stop
        $result.Code = $resp.StatusCode

        if ($resp.StatusCode -eq $ExpectedStatus) {
            $result.Status = "PASS"
            $result.Detail = "HTTP $($resp.StatusCode)"

            if ($BodyCheck) {
                $checkResult = & $BodyCheck $resp.Content
                if (-not $checkResult.Pass) {
                    $result.Status = "FAIL"
                    $result.Detail = $checkResult.Detail
                }
                else {
                    $result.Detail = $checkResult.Detail
                }
            }
        }
        else {
            $result.Detail = "Expected $ExpectedStatus, got $($resp.StatusCode)"
        }
    }
    catch {
        $msg = $_.Exception.Message
        if ($msg.Length -gt 80) { $msg = $msg.Substring(0, 80) }
        $result.Detail = $msg
    }

    return $result
}

Write-Host ""
Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  Axur Web - Production Smoke Tests" -ForegroundColor Cyan
$ts = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
Write-Host "  $ts" -ForegroundColor DarkGray
Write-Host "============================================" -ForegroundColor Cyan
Write-Host ""

# --- Test 1: Frontend loads ---
Write-Host "  [1/4] Frontend HTTP 200 ..." -NoNewline
$r = Test-Endpoint -Name "Frontend Load" -Url $FrontendUrl -BodyCheck {
    param($body)
    if ($body -match "<html") {
        @{ Pass = $true; Detail = "HTML response OK" }
    }
    else {
        @{ Pass = $false; Detail = "Response is not HTML" }
    }
}
$results += $r
$fc = if ($r.Status -eq "PASS") { "Green" } else { "Red" }
Write-Host " $($r.Status)" -ForegroundColor $fc

# --- Test 2: Frontend contains expected content ---
Write-Host "  [2/4] Frontend content ..." -NoNewline
$r = Test-Endpoint -Name "Frontend Content" -Url $FrontendUrl -BodyCheck {
    param($body)
    if ($body -match "wasm|Axur|axur|leptos") {
        @{ Pass = $true; Detail = "App markers found" }
    }
    else {
        @{ Pass = $false; Detail = "No app markers in HTML" }
    }
}
$results += $r
$fc = if ($r.Status -eq "PASS") { "Green" } else { "Red" }
Write-Host " $($r.Status)" -ForegroundColor $fc

# --- Test 3: Backend health ---
Write-Host "  [3/4] Backend /api/health ..." -NoNewline
$healthUrl = "$BackendBase/api/health"
$r = Test-Endpoint -Name "Backend Health" -Url $healthUrl -BodyCheck {
    param($body)
    try {
        $json = $body | ConvertFrom-Json
        $s = $json.status
        @{ Pass = $true; Detail = "status=$s" }
    }
    catch {
        @{ Pass = $false; Detail = "Invalid JSON" }
    }
}
$results += $r
$fc = if ($r.Status -eq "PASS") { "Green" } else { "Red" }
Write-Host " $($r.Status)" -ForegroundColor $fc

# --- Test 4: Backend status endpoint ---
Write-Host "  [4/4] Backend /api/status ..." -NoNewline
$statusUrl = "$BackendBase/api/status"
$r = Test-Endpoint -Name "Backend Status" -Url $statusUrl -BodyCheck {
    param($body)
    try {
        $null = $body | ConvertFrom-Json
        @{ Pass = $true; Detail = "Valid JSON response" }
    }
    catch {
        @{ Pass = $false; Detail = "Invalid JSON" }
    }
}
$results += $r
$fc = if ($r.Status -eq "PASS") { "Green" } else { "Red" }
Write-Host " $($r.Status)" -ForegroundColor $fc

# --- Summary ---
Write-Host ""
Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  Results Summary" -ForegroundColor Cyan
Write-Host "============================================" -ForegroundColor Cyan
Write-Host ""

$results | Format-Table Test, Status, Code, Detail -AutoSize

$passed = @($results | Where-Object { $_.Status -eq "PASS" }).Count
$total  = $results.Count
$fc = if ($passed -eq $total) { "Green" } else { "Yellow" }
$summary = "  $passed of $total tests passed"
Write-Host $summary -ForegroundColor $fc
Write-Host ""

# Exit with non-zero if any test failed
if ($passed -lt $total) {
    exit 1
}
