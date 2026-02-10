---
name: ops_handoff
description: ADHD-friendly session close with auto context generation.
version: 2.0.0
---

# Ops Handoff Protocol

**Objective**: Cleanly close session, save progress, generate context.

## Usage
Run the handoff workflow:
`run_workflow handoff`

## What It Does
1. 📝 Checks **git status**
2. 📦 Generates **context_packet.xml** (ACE Protocol)
3. 📊 Updates **DASHBOARD.md**
4. 🏁 Notifies user

## For AI Agents
At session end:
1. Update `DASHBOARD.md` with completed items
2. Set `FOCUS NOW` to next priority
3. Run `handoff.ps1` or manually update files
