# Contributing Guide

## Project Structure

```
axur-web/
├── crates/
│   ├── backend/          # Axum REST API
│   ├── frontend/         # Leptos WASM SPA
│   └── core/             # Shared library (API, Report HTML, i18n)
├── .agent/workflows/     # Agent automation
└── docs/                 # Documentation
```

## Key Files

### Frontend
- `lib.rs`: App state (`AppState`) and routing.
- `pages/dashboard.rs`: Report generation UI.
- `api.rs`: HTTP client.

### Backend
- `routes/report.rs`: Report generation endpoint.
- `services/report_service.rs`: Business logic for reports.

### Core
- `api/report.rs`: Data models (`PocReportData`).
- `report/html.rs`: HTML generation logic.

## Workflow

1.  **Plan**: Create implementation plan.
2.  **Implement**: Changes in `crates/`.
3.  **Test**: `cargo test --workspace`.
4.  **Audit**: `cargo audit`.
5.  **Document**: Update docs.

## Testing

```bash
cargo test --workspace
cargo clippy --all-targets -- -D warnings
wasm-pack test --headless --chrome crates/frontend
```
