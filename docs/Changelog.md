# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2024-12-22

### Added
- **Full-stack Rust application** with Axum backend and Leptos/WASM frontend
- **3-step authentication flow**: Login → 2FA → Finalization
- **Multi-language support**: Spanish, English, Portuguese (UI and reports)
- **Tenant selection** with autocomplete/search functionality
- **Report generation** with HTML output and download
- **Security features**: 
  - OWASP 2025 compliance checklist
  - RustSec/cargo-audit integration
  - Proper error handling patterns
- **Agent workflows**:
  - `/test-fix-loop` - Automated testing with fix proposals
  - `/security-audit` - Security audit workflow
  - `/fullstack-sync` - Full-stack synchronization
  - `/update-wiki` - Wiki documentation maintenance

### Security
- CORS configuration for frontend-backend communication
- HTTP-only session cookies
- Security headers (X-Content-Type-Options, X-Frame-Options, CSP)
- Input validation on all endpoints

### Documentation
- README with badges and quick start
- User Guide for end users
- Developer Guide for contributors
- Architecture documentation with diagrams
- API Reference

---

## [Unreleased]

### Planned
- [ ] Persistent language preference (localStorage)
- [ ] PDF export option
- [ ] Scheduled report generation
- [ ] Email delivery of reports
