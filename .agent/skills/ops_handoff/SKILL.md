---
name: ops_handoff
description: ADHD-friendly session close with auto context generation.
version: 3.0.0
---

# Ops Handoff Protocol

**Objective**: Cleanly close session, save progress, capture lessons.

## Usage
Run the handoff workflow:
`run_workflow handoff`

## What It Does
1. 📝 Checks **git status**
2. 📦 Generates **context_packet.xml** (ACE Protocol)
3. 📊 Updates **DASHBOARD.md**
4. 🧠 **ACE Reflection** (NEW) — Captures lessons learned
5. 🏁 Notifies user

## For AI Agents
At session end:
1. Update `DASHBOARD.md` with completed items
2. Set `FOCUS NOW` to next priority
3. **ACE Step**: Ask yourself:
   - "Did I learn a new constraint or pattern?"
   - "Did I hit a bug that could recur?"
   - "Did I discover dead code or wrong assumptions?"
   If YES → append to `.agent/memory/LESSONS_LEARNED.md` under today's date
4. Run `handoff.ps1` or manually update files

## ACE Protocol (Analyze-Capture-Evolve)
> Inspired by praefectus project governance.

**When triggered:** End of every session, or after resolving a non-trivial bug.

**Format for entries:**
```markdown
### [Short Title]
- **Context:** What happened
- **Lesson:** What to remember next time

## Systematic Debugging Checklist (If bug was fixed)
- [ ] **Reproduce**: Did we have a repro case?
- [ ] **Locate**: Did we find the root cause (not just symptom)?
- [ ] **Fix**: Is the fix robust?
- [ ] **Verify**: Did we verify it's actually fixed?
- [ ] **Resilience**: Did we add a test to prevent regression?
```

## Repomix Omniscient Mode
To ingest all project knowledge (docs + memory) for deep context:
```powershell
npx repomix --style xml --include "docs/**/*.md,.agent/memory/*.md,.agent/skills/*/SKILL.md"
```
Or via MCP: use `pack_codebase` with `includePatterns: "docs/**/*.md,.agent/memory/*.md"`
