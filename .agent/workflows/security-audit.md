---
description: Security audit workflow using RustSec/cargo-audit and OWASP 2025 guidelines
---

# Security Audit Workflow

Run comprehensive security audits aligned with RustSec, OWASP 2025, NIST, and CAST guidelines.

## Quick Audit
// turbo
```powershell
cargo audit
```

## Full Security Audit Steps

### 1. Dependency Vulnerability Check (RustSec)
// turbo
```powershell
cargo audit --deny warnings
```

If vulnerabilities found:
- Check RustSec advisory database: https://rustsec.org/advisories/
- Update dependencies: `cargo update`
- If no patch available, consider alternative crates

### 2. Update Advisory Database
// turbo
```powershell
cargo audit fetch
```

### 3. Check for Outdated Dependencies
```powershell
cargo outdated -R
```

### 4. License Compliance Check
```powershell
cargo deny check licenses
```

---

## OWASP 2025 Security Checklist

### A01: Broken Access Control (Risk #1)
- [ ] All API endpoints have proper authorization middleware
- [ ] Session tokens are validated on every request
- [ ] Role-based access control (RBAC) is enforced
- [ ] No direct object references without access checks
- [ ] Cookie security flags set (HttpOnly, Secure, SameSite)

**Code Review Points:**
- Check `middleware/` for auth guards
- Verify all routes in `routes/` use auth middleware
- Review cookie settings in auth handlers

### A03: Software Supply Chain Failures (New emphasis 2025)
- [ ] All dependencies audited with `cargo audit`
- [ ] Dependency versions pinned in Cargo.lock
- [ ] No `git` dependencies from untrusted sources
- [ ] `cargo-vet` or `cargo-crev` for supply chain verification
- [ ] CI/CD runs security checks on PRs

**Commands:**
```powershell
cargo audit
cargo tree --duplicates
cargo deny check
```

### A10: Mishandling of Exceptional Conditions (New 2025)
- [ ] All `Result` types are properly handled (no `.unwrap()` in production)
- [ ] `Option` types use `.ok_or()` or pattern matching
- [ ] Errors are logged with appropriate detail level
- [ ] User-facing errors don't leak internal details
- [ ] Panic handlers are configured for WASM

**Rust-Specific Patterns:**
```rust
// GOOD: Proper error handling
result.map_err(|e| ApiError::from(e))?;

// BAD: Panics in production
result.unwrap(); // ❌
result.expect("should work"); // ❌
```

---

## NIST Compliance Notes

### NIST SP 800-53 Controls
- **AC-3**: Access Enforcement → Use Axum middleware
- **AU-3**: Audit Logging → Use tracing crate
- **SC-8**: Transmission Confidentiality → HTTPS/TLS
- **SI-10**: Information Input Validation → Validate all API inputs

### NIST SSDF (Secure Software Development Framework)
- PO.1: Define security requirements
- PW.1: Design software to meet security requirements
- PW.6: Use compiler options to improve security
- RV.1: Identify and confirm vulnerabilities

---

## CAST/CISQ Quality Rules

### Reliability
- No unhandled exceptions → Rust's Result/Option
- No null pointer dereferences → Rust's type system

### Security
- No SQL injection → Use parameterized queries
- No hardcoded credentials → Use environment variables
- Input validation on all user data

### Maintainability
- Avoid excessive cyclomatic complexity
- Keep function lengths reasonable
- Document public APIs

---

## Remediation Workflow

When vulnerabilities are found:

1. **Document** the vulnerability in an implementation plan
2. **Assess** impact (CVSS score, exploitability)
3. **Prioritize** based on severity
4. **Propose fix** with code changes
5. **Get approval** before applying
6. **Verify** fix with re-audit
7. **Update** any affected tests
