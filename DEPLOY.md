# Axur Web - Deployment Guide

## Prerequisites

1. **Rust toolchain**: `rustup update stable`
2. **Shuttle CLI**: `cargo install cargo-shuttle`
3. **Trunk** (for frontend): `cargo install trunk`
4. **wasm32 target**: `rustup target add wasm32-unknown-unknown`

---

## Backend Deployment (Shuttle.rs)

### 1. Login to Shuttle
```bash
cargo shuttle login
```

### 2. Initialize project (first time only)
```bash
cd crates/backend
cargo shuttle project start
```

### 3. Deploy
```bash
cargo shuttle deploy --features shuttle
```

Your backend will be available at:
`https://axur-web.shuttleapp.rs`

### Environment Variables
Currently none required - the backend proxies to Axur API directly.

---

## Frontend Deployment (Cloudflare Pages)

### 1. Build WASM
```bash
cd crates/frontend
trunk build --release
```

This creates a `dist/` folder with:
- `index.html`
- `pkg/axur_frontend_bg.wasm`
- `pkg/axur_frontend.js`

### 2. Deploy to Cloudflare Pages

**Option A: Via Dashboard**
1. Go to [Cloudflare Pages](https://pages.cloudflare.com)
2. Create new project
3. Upload the `dist/` folder

**Option B: Via Wrangler CLI**
```bash
npm install -g wrangler
wrangler pages publish dist --project-name=axur-web
```

### 3. Configure Frontend API URL
Before building for production, update `src/api.rs`:
```rust
const API_BASE: &str = "https://axur-web.shuttleapp.rs";
```

---

## Local Development

### Backend
```bash
cargo run -p axur-backend
# Runs on http://localhost:3001
```

### Frontend
```bash
cd crates/frontend
trunk serve --open
# Runs on http://localhost:8080
```

---

## URLs Summary

| Environment | Backend | Frontend |
|-------------|---------|----------|
| **Local** | http://localhost:3001 | http://localhost:8080 |
| **Production** | https://axur-web.shuttleapp.rs | https://axur-web.pages.dev |

---

## Troubleshooting

### CORS Issues
If you get CORS errors, ensure the frontend URL is in the backend's CORS config:
```rust
// In main.rs
.allow_origin([
    "https://axur-web.pages.dev".parse().unwrap(),
])
```

### Cookie Issues (Production)
Ensure cookies work across domains:
- Backend and frontend should share same-site or use `SameSite=None; Secure`
- Consider using a custom domain for both
