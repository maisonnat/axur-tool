---
name: qa_api_smoke
description: API endpoint health checks to verify backend functionality.
version: 1.0.0
status: active
---

# API Smoke Test

**Objective**: Verify backend API endpoints respond correctly and handle requests properly.

## When to Use
- After backend changes
- Before promoting to production
- When debugging frontend-backend integration

## Prerequisites
- Backend running at `http://localhost:3001` (via `.\dev.ps1`)

## Procedure

### 1. Health Check
```powershell
# Basic health endpoint
curl http://localhost:3001/api/health
# Expected: 200 OK
```

### 2. Version/Info Endpoint
```powershell
curl http://localhost:3001/api/version
# Expected: JSON with version info
```

### 3. Protected Endpoint Test
```powershell
# Without auth (should fail gracefully)
curl http://localhost:3001/api/protected-resource
# Expected: 401 or appropriate error

# With auth
curl -H "Authorization: Bearer TOKEN" http://localhost:3001/api/protected-resource
```

## Endpoint Checklist

| Endpoint | Method | Expected Status | Purpose |
|----------|--------|-----------------|---------|
| `/api/health` | GET | 200 | Server alive check |
| `/api/version` | GET | 200 | Build/version info |
| `/api/beta-access` | POST | 200/400 | Beta signup |

## PowerShell Test Script
```powershell
$base = "http://localhost:3001"

# Health check
$health = Invoke-RestMethod "$base/api/health" -Method Get
Write-Host "Health: $($health | ConvertTo-Json)"

# Test endpoints
$endpoints = @(
    @{Path="/api/health"; Method="GET"; Expected=200},
    @{Path="/api/version"; Method="GET"; Expected=200}
)

foreach ($ep in $endpoints) {
    try {
        $resp = Invoke-WebRequest "$base$($ep.Path)" -Method $ep.Method
        if ($resp.StatusCode -eq $ep.Expected) {
            Write-Host "✅ $($ep.Path): OK" -ForegroundColor Green
        }
    } catch {
        Write-Host "❌ $($ep.Path): FAILED" -ForegroundColor Red
    }
}
```

## Common Issues Detected

| Issue | Status Code | Cause |
|-------|-------------|-------|
| Server down | Connection refused | Backend not running |
| Config error | 500 | Missing env vars |
| Auth failure | 401/403 | Token issues |
| Bad route | 404 | Endpoint not defined |
| Rate limit | 429 | Too many requests |

## Integration with Axur API

> [!WARNING]
> When testing Axur API endpoints, respect rate limits per `api-rate-limit.md` rules.
