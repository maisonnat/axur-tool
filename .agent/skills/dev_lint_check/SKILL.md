---
name: dev_lint_check
description: Static analysis using Clippy and rustfmt to catch code quality issues.
version: 1.0.0
status: active
---

# Static Analysis Check

**Objective**: Detect code quality issues, warnings, and formatting problems before runtime.

## When to Use
- Before committing code
- As part of VERIFY phase in `ops_local_dev`
- After major refactoring

## Procedure

### 1. Clippy Analysis
```powershell
cargo clippy --workspace -- -D warnings
```

**Exit codes**:
- `0` = ✅ No issues
- `non-zero` = ❌ Warnings or errors found

### 2. Format Check
```powershell
cargo fmt --check
```

**Exit codes**:
- `0` = ✅ Code is formatted
- `non-zero` = ❌ Formatting issues (run `cargo fmt` to fix)

### 3. Documentation Check (Optional)
```powershell
cargo doc --workspace --no-deps
```

## Quick One-Liner
```powershell
cargo clippy --workspace -- -D warnings && cargo fmt --check
```

## Common Issues Detected

| Category | Example |
|----------|---------|
| Unused variables | `warning: unused variable: x` |
| Dead code | `warning: function is never used` |
| Bad patterns | `warning: you should consider using...` |
| Missing docs | `warning: missing documentation` |
| Format issues | Inconsistent indentation/spacing |

## Auto-Fix
```powershell
# Fix formatting automatically
cargo fmt

# Apply clippy suggestions (careful!)
cargo clippy --fix --workspace
```
