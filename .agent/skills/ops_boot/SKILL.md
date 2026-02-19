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
## What It Does
1. 📊 Loads **Project Context** (via `task.md` and Repomix)
2. 🚦 Checks **service health** (ports 3001/8080)
3. 🎯 Establishes **Session Goal**

## Alternative: Manual Steps
If script fails:
```powershell
# Read Task Context
cat .agent/tasks/current.md
# Check Ports
Test-NetConnection localhost -Port 3001
```

## For AI Agents
1. Read `task.md` to understand the objective.
2. Read `knowledge/LESSONS_LEARNED.md` to avoid regressions.
3. **DO NOT** look for `DASHBOARD.md` (Deprecated).
