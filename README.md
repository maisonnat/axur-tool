# Axur Tool

> Full-stack Rust web application for Axur External Threat Intelligence report generation.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=webassembly&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-000000?style=for-the-badge)
![Leptos](https://img.shields.io/badge/Leptos-EF4444?style=for-the-badge)

## 🚀 Overview

A complete Rust-based web tool for authenticating with Axur API and generating threat intelligence reports. Features:

- **Full Rust Stack**: Backend (Axum) + Frontend (Leptos/WASM)
- **3-Step Auth Flow**: Login → 2FA → Finalization
- **Multi-language UI**: Spanish, English, Portuguese
- **Tenant Autocomplete**: Search and filter tenants
- **Report Generation**: Beautiful HTML reports with Axur branding
- **Security First**: OWASP 2025, RustSec audits, proper error handling

## 📁 Project Structure

```
axur-web/
├── crates/
│   ├── backend/      # Axum REST API server
│   ├── frontend/     # Leptos WASM SPA
│   └── core/         # Shared types and API client
├── .agent/
│   └── workflows/    # Agent automation workflows
├── build-frontend.ps1
└── Cargo.toml
```

## 🛠️ Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [trunk](https://trunkrs.dev/) for frontend dev server

```bash
# Install tools
cargo install wasm-pack trunk cargo-audit
rustup target add wasm32-unknown-unknown
```

## 🏃 Quick Start

### Development

```bash
# Terminal 1: Start backend
cargo run -p axur-backend

# Terminal 2: Start frontend (Windows)
.\build-frontend.ps1 -Serve

# Or use trunk directly
cd crates/frontend && trunk serve
```

- Backend: http://localhost:3001
- Frontend: http://localhost:8080

### Production Build

```bash
# Build backend
cargo build -p axur-backend --release

# Build frontend
wasm-pack build crates/frontend --target web --release
```

## 🔐 Security

This project follows security best practices:

- **RustSec**: Run `cargo audit` for vulnerability checks
- **OWASP 2025**: A01 (Access Control), A03 (Supply Chain), A10 (Error Handling)
- **Rust Safety**: Proper `Result`/`Option` handling, no unwrap in production

See `.agent/workflows/security-audit.md` for full audit workflow.

## 🧪 Testing

```bash
# Run all tests
cargo test --workspace

# Frontend WASM tests
wasm-pack test --headless --chrome crates/frontend

# Lint check
cargo clippy --all-targets -- -D warnings
```

## 📚 Documentation

See the [Wiki](../../wiki) for:
- User Guide
- Developer Guide
- Architecture Overview
- API Reference

Supports [Koyeb](https://koyeb.com) for easy deployment (free tier):

See [DEPLOY.md](DEPLOY.md) for full instructions.

## 📄 License

MIT License - See [LICENSE](LICENSE) for details.

---

Built with ❤️ using Rust
