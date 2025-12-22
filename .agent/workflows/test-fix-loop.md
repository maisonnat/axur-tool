---
description: Run full-stack test suite (backend + WASM frontend) and auto-fix failures with approval
---

# Full-Stack Test & Fix Loop

This workflow runs the complete test suite for both backend and frontend, analyzes failures, and proposes fixes that require user approval before applying.

## Prerequisites
- Rust toolchain installed
- wasm-pack installed (`cargo install wasm-pack`)
- cargo-audit installed (`cargo install cargo-audit`)

## Test Execution Steps

### 1. Run Backend Tests
```powershell
cargo test -p axur-backend -p axur-core --no-fail-fast
```

### 2. Run Frontend WASM Tests
```powershell
wasm-pack test --headless --chrome crates/frontend
```

### 3. Run Clippy Lints
```powershell
cargo clippy --all-targets --all-features -- -D warnings
```

## On Test Failure

If any test fails:

1. **Read the error output** carefully
2. **Identify the root cause** (assertion failure, panic, type error, etc.)
3. **Create an implementation plan artifact** with:
   - Description of the failure
   - Root cause analysis
   - Proposed fix with code changes
   - Impact assessment
4. **Wait for user approval** before applying the fix
5. After approval, apply the fix and re-run the failing test
6. Repeat until all tests pass

## Success Criteria
- All `cargo test` commands pass
- All `wasm-pack test` commands pass
- No clippy warnings
