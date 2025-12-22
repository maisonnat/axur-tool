# Deployment Guide

## Overview

This project uses automated CI/CD deployment:
- **Backend**: Deployed to [Shuttle.rs](https://shuttle.rs)
- **Frontend**: Deployed to [Cloudflare Pages](https://pages.cloudflare.com)

## Prerequisites

Before deployment works, you need to configure GitHub Secrets.

### 1. Shuttle API Key

1. Install Shuttle CLI: `cargo install cargo-shuttle`
2. Login: `cargo shuttle login`
3. Get your API key from [Shuttle Console](https://console.shuttle.rs)
4. Add to GitHub: Settings → Secrets → Actions → New secret
   - Name: `SHUTTLE_API_KEY`
   - Value: Your Shuttle API key

### 2. Cloudflare Credentials

1. Go to [Cloudflare Dashboard](https://dash.cloudflare.com)
2. Create a Cloudflare Pages project named `axur-tool`
3. Get your Account ID from the dashboard URL or Overview page
4. Create an API Token with "Cloudflare Pages" permissions
5. Add to GitHub: Settings → Secrets → Actions → New secrets
   - `CLOUDFLARE_ACCOUNT_ID`: Your account ID
   - `CLOUDFLARE_API_TOKEN`: Your API token

## URLs After Deployment

| Service | URL |
|---------|-----|
| Backend (Shuttle) | https://axur-web.shuttle.app |
| Frontend (Cloudflare) | https://axur-tool.pages.dev |

## Manual Deployment

### Backend to Shuttle
```bash
cargo shuttle login
cargo shuttle deploy
```

### Frontend to Cloudflare
```bash
cd crates/frontend
trunk build --release
npx wrangler pages deploy dist --project-name=axur-tool
```

## Workflow Triggers

- **Backend deploy**: Triggers on push to `main` when `crates/backend/`, `crates/core/`, or `Cargo.toml` changes
- **Frontend deploy**: Triggers on push to `main` when `crates/frontend/` or `crates/core/` changes

## Environment Variables

### Build Time (Frontend)
- `API_BASE_URL`: Set to production backend URL (e.g., `https://axur-web.shuttle.app`)

### Runtime (Backend)
- No special environment variables required
- Shuttle handles secrets via `Secrets.toml` (local) or console
