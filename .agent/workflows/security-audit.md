---
description: Security audit workflow using RustSec/cargo-audit and OWASP 2025 guidelines
---

```markdown
# Security Audit Workflow (Rust 2025 Edition)

Run comprehensive security audits aligned with RustSec, OWASP Top 10 2025, NIST SP 800-63B Rev 4, and SSDF v1.2.

## Quick Audit (Local Dev)
// turbo
```powershell
cargo audit
cargo deny check licenses

```

## Full Security Audit Steps

### 1. Dependency Vulnerability Check (RustSec)

**Objective:** Detect known vulnerabilities in the dependency tree.

// turbo

```powershell
cargo audit --deny warnings

```

If vulnerabilities found:

* Check RustSec advisory database: https://rustsec.org/advisories/
* Update dependencies: `cargo update` or replace the crate if unmaintained.

### 2. Supply Chain Integrity (OWASP A03:2025)

**Objective:** Verify the integrity of the software supply chain beyond just vulnerabilities.

**Audit Trusted Publishers:**

```powershell
cargo vet

```

*Use cargo-vet to ensure dependencies have been audited by trusted entities (Google, Mozilla, or internal team).*

**Source & License Control:**

```powershell
cargo deny check sources

```

*Ensure no dependencies are pulled from untrusted git registries or unverified crates.io mirrors.*

### 3. Production Build with SBOM (Evidence Collection)

**Objective:** Embed dependency data into the binary for runtime auditing (Compliance with NIST SSDF 1.2).

```powershell
cargo auditable build --release

```

*Uses cargo-auditable to embed the dependency tree into the compiled binary.*

### 4. Unsafe Code Audit (Memory Safety)

**Objective:** Audit blocks that bypass Rust's memory safety guarantees (NIST/NSA Focus).

```powershell
cargo geiger

```

*Rule: Any use of `unsafe` must be documented and justified. Ideally, replace with safe abstractions.*

---

## OWASP 2025 Security Checklist

### A01: Broken Access Control (Risk #1) & SSRF

* [ ] **Middleware:** Ensure axum or actix-web middleware enforces authentication on all protected routes
* [ ] **IDOR Prevention:** Use typed identifiers (e.g., `struct UserId(Uuid)`) instead of raw integers
* [ ] **SSRF Check:** Validate all user-supplied URLs if the server makes outbound requests (block localhost/metadata services)

### A02: Security Misconfiguration (Risk #2)

* [ ] **Headers:** Ensure security headers (HSTS, Content-Security-Policy) are set
* [ ] **Debug Mode:** Verify debug assertions are disabled in release builds (`[profile.release] debug = false`)
* [ ] **Panic Strategy:** Set `panic = "abort"` in Cargo.toml for production release profiles

### A03: Software Supply Chain Failures (New Emphasis 2025)

* [ ] **Binary Auditing:** Verify production binaries contain embedded SBOMs (`cargo auditable`)
* [ ] **CI/CD Integrity:** Ensure CI pipelines use immutable tags for actions/containers
* [ ] **Dependency Pinning:** `Cargo.lock` must be committed to version control

### A07: Authentication Failures (NIST 800-63B Rev 4 Alignment)

* [ ] **Password Length:** Enforce minimum 15 characters for single-factor passwords
* [ ] **Composition Rules:** REMOVE requirements for special characters/numbers. Allow spaces (Unicode 64+ chars)
* [ ] **Blocklist:** Check new passwords against a "banned list" (e.g., "Password123")
* [ ] **MFA:** Offer Phishing-Resistant MFA (Passkeys/FIDO2) using `webauthn-rs`. Avoid SMS/Email OTP

### A10: Mishandling of Exceptional Conditions (New 2025)

* [ ] **No Panics:** Prohibit `.unwrap()` and `.expect()` in production code
* [ ] **Fail Closed:** Ensure auth logic fails to a "deny" state if errors occur (e.g., DB unreachable)
* [ ] **Error Leakage:** Ensure API error responses do not leak internal stack traces

**Rust-Specific Patterns:**

```rust
// Use clippy to enforce strict error handling
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

// GOOD: Mapped errors for external consumers
result.map_err(|e| ApiError::from(e))?;

// BAD: Leaking internal DB errors / Panics
result.unwrap(); // ❌

```

---

## NIST & Regulatory Compliance

### NIST SP 800-218 (SSDF 1.2)

* **PO.6 (Continuous Improvement):** Document audit results over time to show trend improvement
* **PS.4 (Robust Updates):** Use `cargo-auditable` to prove exactly what is running in production

### NIST SP 800-53 Rev 5

* **SI-7 (Software Integrity):** Use cryptographic hashes (SHA-256) for all release artifacts
* **SC-8 (Transmission Confidentiality):** Enforce TLS 1.3 via `rustls` (avoid OpenSSL where possible)

---

## Remediation Workflow

When vulnerabilities are found:

1. **Identify**: Run `cargo audit` and `cargo geiger`
2. **Assess**:
* Is it a Supply Chain issue (A03)? → Check `cargo-vet`
* Is it a Logic issue (A10)? → Review error handling paths


3. **Fix**:
* Dependencies: `cargo update` or patch via `[patch.crates-io]`
* Code: Refactor unsafe blocks or replace `.unwrap()` with Result propagation


4. **Verify**: Re-run the audit suite
5. **Document**: Update the Digital Identity Acceptance Statement (if applicable)

```

```