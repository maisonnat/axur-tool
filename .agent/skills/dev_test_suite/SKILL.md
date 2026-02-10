---
name: dev_test_suite
description: Run unit and integration tests to catch regressions and logic errors.
version: 1.0.0
status: active
---

# Test Suite Runner

**Objective**: Execute all tests to detect regressions and verify business logic.

## When to Use
- Before committing code
- After any logic changes
- As part of VERIFY phase in `ops_local_dev`

## Procedure

### 1. Run All Tests
```powershell
cargo test --workspace
```

### 2. Run Specific Crate Tests
```powershell
# Backend only
cargo test -p axur-backend

# Core library only
cargo test -p axur-core

# Frontend (if applicable)
cargo test -p axur-frontend
```

### 3. Run with Output (Debug)
```powershell
cargo test --workspace -- --nocapture
```

### 4. Run Specific Test
```powershell
cargo test test_name --workspace
```

## Test Categories

| Category | Location | Purpose |
|----------|----------|---------|
| Unit tests | `src/**/*.rs` | Function-level logic |
| Integration | `tests/` | Cross-module behavior |
| Doc tests | `/// # Examples` | Documentation accuracy |

## Coverage (Optional)
```powershell
# Install coverage tool
cargo install cargo-tarpaulin

# Run with coverage
cargo tarpaulin --workspace --out Html
```

## Common Failures

| Issue | Symptom | Fix |
|-------|---------|-----|
| State leak | Tests pass alone, fail together | Use `#[serial]` or isolate state |
| Async race | Intermittent failures | Add proper awaits/timeouts |
| Missing mock | API calls in tests | Use `mockall` or test doubles |

## Quick Reference
```powershell
# Full test suite
cargo test --workspace

# With verbose output
cargo test --workspace -- --nocapture 2>&1 | tee test_output.log
```
