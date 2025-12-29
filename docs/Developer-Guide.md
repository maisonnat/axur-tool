# Developer Guide

## Tech Stack

| Component | Technology |
|-----------|------------|
| Backend | Rust + Axum |
| Frontend | Rust + Leptos (WASM) |
| Shared | axur-core crate |
| Build | wasm-pack, trunk |
| Deployment | Leapcell (Native Rust) |

## Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install tools
cargo install wasm-pack trunk cargo-audit
```

## Development Setup

### Starting the Backend
```bash
cargo run -p axur-backend
# Server runs at http://localhost:3001
```

### Starting the Frontend
```powershell
# Windows
.\build-frontend.ps1 -Serve
# Frontend runs at http://localhost:8080
```

Or using trunk directly:
```bash
cd crates/frontend
trunk serve
```

## Project Structure

```
axur-web/
├── crates/
│   ├── backend/          # Axum REST API
│   │   ├── src/
│   │   │   ├── main.rs       # Entry point
│   │   │   ├── lib.rs        # App configuration
│   │   │   ├── routes/       # API endpoints
│   │   │   └── middleware/   # Auth, security
│   │   └── Cargo.toml
│   ├── frontend/         # Leptos WASM SPA
│   │   ├── src/
│   │   │   ├── lib.rs        # App state, routing
│   │   │   ├── api.rs        # API client
│   │   │   ├── i18n.rs       # Translations
│   │   │   ├── pages/        # Login, Dashboard
│   │   │   └── components/   # Reusable UI
│   │   ├── index.html
│   │   └── Cargo.toml
│   └── core/             # Shared library
│       ├── src/
│       │   ├── lib.rs        # Public API
│       │   ├── api/          # Axur API client
│       │   └── report/       # HTML generation
│       └── Cargo.toml
├── .agent/workflows/     # Agent automation
├── docs/                 # Documentation
└── Cargo.toml           # Workspace config
```

## Key Files

### Frontend
- `lib.rs` - Application state (`AppState`) and page routing
- `pages/login.rs` - 3-step authentication flow
- `pages/dashboard.rs` - Report generation UI
- `i18n.rs` - Internationalization (ES/EN/PT)
- `api.rs` - HTTP client for backend communication

### Backend
- `routes/auth.rs` - Login, 2FA, session management
- `routes/report.rs` - Report generation endpoint
- `middleware/security.rs` - Security headers, CORS

### Core
- `api/mod.rs` - Axur API client
- `report/html.rs` - HTML report generation

## Adding a New Feature

1. **Plan**: Create an implementation plan artifact
2. **Implement**: 
   - Add types to `axur-core` if shared
   - Add endpoints to `axur-backend`
   - Add UI to `axur-frontend`
3. **Test**: Run `cargo test --workspace`
4. **Audit**: Run `cargo audit`
5. **Document**: Update wiki pages

## Testing

```bash
# Run all tests
cargo test --workspace

# Run clippy lints
cargo clippy --all-targets -- -D warnings

# Run WASM tests (requires Chrome)
wasm-pack test --headless --chrome crates/frontend
```

## Security

Follow the `/security-audit` workflow for comprehensive security checks including:
- RustSec vulnerability scanning
- OWASP 2025 compliance
- Proper error handling verification

## Deployment

Using Leapcell (Native Rust Builder):

1. Push to main branch
2. Leapcell auto-builds and deploys

See [DEPLOY.md](../DEPLOY.md) for full instructions.
