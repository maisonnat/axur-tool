---
description: Verify code changes work correctly and fix issues before moving on
---

# Self-Verification Workflow

This workflow defines how to verify that code changes work correctly and handle failures.

## Verification Strategy

### 1. After Every Code Change

```bash
# turbo
# Step 1: Compile the affected crate
cargo build -p <affected-crate> 2>&1

# turbo
# Step 2: Run tests for the affected module
cargo test -p <affected-crate> <module_path> 2>&1
```

### 2. Interpret Results

| Result | Action |
|--------|--------|
| Build succeeds, tests pass | ✅ Continue to next task |
| Build fails | 🔧 Fix compilation errors immediately |
| Tests fail | 🔧 Analyze failure, fix code, re-run tests |
| Warning only | ⚠️ Note warning, fix if trivial, continue |

### 3. Fix Loop (Max 3 Attempts)

If build or tests fail:

1. **Read the error message carefully**
2. **Identify root cause** (syntax, logic, missing import, wrong path)
3. **Make targeted fix** (smallest change that fixes the issue)
4. **Re-run verification**
5. **If 3 attempts fail**: Stop, document the issue, ask user for guidance

### 4. Verification Levels

| Change Type | Verification Required |
|-------------|----------------------|
| JSON/Config files | Compile only |
| Rust module (no deps) | Compile + unit tests |
| Rust module (with deps) | Compile + unit + integration tests |
| API changes | Compile + tests + manual endpoint test |
| Frontend changes | Compile + trunk build + visual check |

### 5. Documentation of Failures

When encountering persistent failures, document in the relevant MILESTONES.md:

```markdown
### Blockers Encountered

- **Issue**: [description]
- **Error**: [error message]
- **Attempts**: [what was tried]
- **Status**: [resolved/pending user input]
```

## Checklist Before Moving to Next Milestone

- [ ] All new code compiles without errors
- [ ] All new tests pass
- [ ] No new warnings introduced (or documented why acceptable)
- [ ] MILESTONES.md updated with results
- [ ] If integration needed, basic smoke test performed

## Commands Reference

```bash
# Build specific crate
cargo build -p axur-core

# Test specific module
cargo test -p axur-core i18n::loader

# Test all
cargo test --all

# Frontend build
cd crates/frontend && trunk build

# Check for warnings
cargo clippy -p axur-core
```
