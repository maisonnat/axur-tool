---
name: backend_health_check
description: Verifies local backend environment health (DB, Env, Ports).
version: 1.0.0
---

# Backend Health Check

**Objective**: Diagnostic tool to verify that the local development environment is correctly configured and operational.

## Usage
Run this skill if the backend fails to start or behaves unexpectedly.

## Procedure

### 1. Environment Variables
- Check if `.env` exists.
- Check if critical keys (`DATABASE_URL`, `AXUR_API_KEY`, `RUST_LOG`) are set.

### 2. Database Connectivity
- Attempt to connect to `DATABASE_URL` (using simple `psql` or `sqlx` output).

### 3. Port Availability
- Check if port **3001** (Backend default) is free.
```powershell
Test-NetConnection -ComputerName localhost -Port 3001
```

### 4. Recent Logs
- Display the last 20 lines of the latest log file in `debug_logs/`.

## Script
(Future: Implement `check_health.ps1` to automate this)
