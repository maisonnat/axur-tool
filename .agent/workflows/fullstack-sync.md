---
description: Sync and verify both backend and frontend are running correctly
---

# Full-Stack Sync Workflow

Ensures backend and frontend are synchronized, built, and running together.

## Quick Start (Full Stack)
// turbo-all

### 1. Build Backend
```powershell
cargo build -p axur-backend
```

### 2. Build Frontend WASM
```powershell
wasm-pack build crates/frontend --target web --dev
```

### 3. Start Backend Server
```powershell
cargo run -p axur-backend
```

### 4. Start Frontend Dev Server
```powershell
.\build-frontend.ps1 -Serve
```

---

## Verify Sync Status

### Check Backend Health
```powershell
curl http://localhost:3001/health
```

### Check Frontend
Open browser to: http://localhost:8080

### Check CORS Configuration
Backend must allow origin: http://localhost:8080

---

## Sync Points to Verify

### API Contract
- [ ] Frontend `api.rs` request types match backend handlers
- [ ] Response types in frontend match backend responses
- [ ] API base URL is correct (`http://localhost:3001`)

### Shared Types
- [ ] `axur-core` types used consistently
- [ ] Serialization/deserialization compatible

### Authentication Flow
- [ ] Cookie domain compatible (localhost)
- [ ] CORS credentials: Include
- [ ] Session validation endpoint working

---

## Common Issues

### Frontend Can't Reach Backend
1. Check backend is running on port 3001
2. Check CORS configuration includes frontend origin
3. Check network tab for failed requests

### WASM Build Fails
1. Clean target: `cargo clean -p axur-frontend`
2. Rebuild: `wasm-pack build crates/frontend --target web --dev`

### Type Mismatch
1. Compare `api.rs` types with backend handler responses
2. Run `cargo check` on both crates
