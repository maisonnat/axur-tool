---
description: Update GitHub Wiki with project documentation for users and developers
---

# GitHub Wiki Documentation Workflow

Maintain project documentation in the GitHub Wiki for both end users and developers.

## Wiki Structure

```
wiki/
├── Home.md                    # Main landing page
├── User-Guide.md              # How to use the application
├── Developer-Guide.md         # How to contribute/develop
├── Architecture.md            # System design and components
├── API-Reference.md           # Backend API documentation
└── Changelog.md               # Version history
```

---

## Creating/Updating Wiki Pages

### 1. Clone Wiki Repository
```bash
git clone https://github.com/maisonnat/axur-tool.wiki.git
cd axur-tool.wiki
```

### 2. Home.md Template
```markdown
# Axur Tool Wiki

Welcome to the Axur Tool documentation!

## Quick Links
- [[User Guide]] - How to use the application
- [[Developer Guide]] - Development setup and contribution
- [[Architecture]] - System design overview
- [[API Reference]] - Backend endpoints
- [[Changelog]] - Version history

## About
Axur Tool is a full-stack Rust web application for generating 
Axur External Threat Intelligence reports.
```

### 3. User Guide Template
```markdown
# User Guide

## Getting Started
1. Navigate to the application
2. Select your language (ES/EN/PT)
3. Enter your Axur credentials
4. Complete 2FA verification

## Generating Reports
1. Select a tenant from the dropdown (supports search)
2. Choose date range
3. Select report language
4. Click "Generate Report"
5. Download the HTML report

## Troubleshooting
- **Login fails**: Check your Axur credentials
- **No tenants**: Contact your Axur administrator
- **Report error**: Check date range validity
```

### 4. Developer Guide Template
```markdown
# Developer Guide

## Tech Stack
- **Backend**: Rust + Axum
- **Frontend**: Rust + Leptos (WASM)
- **Shared**: axur-core crate

## Setup
\`\`\`bash
# Prerequisites
cargo install wasm-pack trunk cargo-audit
rustup target add wasm32-unknown-unknown

# Run development
cargo run -p axur-backend  # Backend on :3001
.\build-frontend.ps1 -Serve  # Frontend on :8080
\`\`\`

## Project Structure
- `crates/backend/` - Axum REST API
- `crates/frontend/` - Leptos WASM SPA
- `crates/core/` - Shared types and API client

## Key Files
- `lib.rs` - Application state and routing
- `pages/login.rs` - Authentication flow
- `pages/dashboard.rs` - Report generation
- `i18n.rs` - Internationalization

## Adding Features
1. Create implementation plan
2. Add to appropriate crate
3. Run tests: `cargo test --workspace`
4. Run security audit: `cargo audit`
```

### 5. Architecture Template
```markdown
# Architecture

## System Overview
\`\`\`
┌─────────────────┐     ┌─────────────────┐
│   Frontend      │────▶│    Backend      │
│   (Leptos/WASM) │     │    (Axum)       │
│   :8080         │     │    :3001        │
└─────────────────┘     └────────┬────────┘
                                 │
                        ┌────────▼────────┐
                        │   Axur API      │
                        │   (External)    │
                        └─────────────────┘
\`\`\`

## Authentication Flow
1. User submits email/password
2. Backend proxies to Axur `/auth/login`
3. Axur returns token + requires 2FA
4. User submits 2FA code
5. Backend proxies to Axur `/auth/2fa`
6. Backend calls `/auth/finalize`
7. Session cookie set, user redirected to dashboard

## Report Generation
1. Frontend requests tenant list
2. User selects tenant + date range
3. Backend fetches data from Axur API
4. axur-core generates HTML report
5. Frontend displays preview + download
```

### 6. Push Updates
```bash
git add .
git commit -m "Update wiki documentation"
git push origin master
```

---

## When to Update Wiki

Update documentation when:
- [ ] New features are added
- [ ] API endpoints change
- [ ] Authentication flow changes
- [ ] Major bug fixes that affect usage
- [ ] New configuration options added

## Documentation Checklist

Before release, verify:
- [ ] User Guide reflects current UI
- [ ] Developer Guide has accurate setup steps
- [ ] Architecture diagram is current
- [ ] API Reference matches implementation
- [ ] Changelog updated with version
