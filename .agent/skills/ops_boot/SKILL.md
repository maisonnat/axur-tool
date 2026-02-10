---
name: ops_boot
description: ADHD-friendly session start with visual dashboard and health check.
version: 2.0.0
---

# Ops Boot Protocol

**Objective**: Quickly orient to current context with visual feedback.

## Usage
Run the boot workflow:
`run_workflow boot`

## What It Does
1. 📊 Shows **DASHBOARD.md** (status, focus, done today)
2. 🚦 Checks **service health** (ports 3001/8080)
3. 📜 Reads **Constitution**
4. 🎯 Reports status to user

## Alternative: Manual Steps
If script fails:
```powershell
Get-Content .agent/memory/DASHBOARD.md
Test-NetConnection localhost -Port 3001
```

## For AI Agents
Read `.agent/memory/DASHBOARD.md` and `.agent/memory/LESSONS_LEARNED.md` at session start.
