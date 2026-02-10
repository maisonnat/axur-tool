---
name: ops_security_audit
description: Automated security compliance check for Rust/Axum stack.
version: 1.0.0
---

# Ops Security Audit

**Objective**: Verify codebase security posture against OWASP and RustSec standards.

## Usage
When the user requests a security check, a "pre-flight" check, or before a major release, execute this skill.

## Procedure

### 1. Dependency Analysis
Run:
```powershell
cargo audit --deny warnings
```
- **If it fails**: Analyze the vulnerability. If it's a dev-dependency, note it. If it's a production dependency, STOP and propose a fix (update or patch).

### 2. License Compliance
Run:
```powershell
cargo deny check licenses
```
- Ensure no copyleft logic is accidentally introduced.

### 3. Safety Check (Memory)
Run:
```powershell
cargo geiger
```
- If unavailable, skip.
- Inspect `unsafe` blocks.

### 4. Hygiene Check (Panics)
Search for forbidden unwrap calls in the backend:
```powershell
rg "unwrap\(\)" crates/backend/src
rg "expect\(\)" crates/backend/src
```
- **Rule**: `unwrap()` is FORBIDDEN in `crates/backend` (except tests).
- **Action**: If found, flag them as critical issues to be refactored into `?` (Result propagation).

## Output
Generate a summary report:
- **Status**: RED/GREEN
- **Vulnerabilities**: Count
- **Licensing**: OK/FAIL
- **Hygiene**: Count of unwraps
