# Deployment Guide

## Overview

This project uses automated CI/CD deployment:
- **Backend**: Deployed to [Leapcell](https://leapcell.io) (Native Rust builder)
- **Frontend**: Deployed to [Cloudflare Pages](https://pages.cloudflare.com) (Free)

## Prerequisites

Before deployment works, you need to configure GitHub Secrets.

### 1. Leapcell Setup

1. Create account at [Leapcell](https://leapcell.io) (free tier available)
2. Create a new Service:
   - Connect your GitHub account
   - Select the `axur-web` repository
   - Language: Rust
   - Build Command: `cargo build --release --bin axur-backend`
   - Start Command: `./target/release/axur-backend`
   - Port: Auto-detected (uses `PORT` env var)
3. Add Environment Variables in Leapcell Dashboard:
   - `RUST_LOG`: `axur_backend=info,tower_http=info`
   - `GITHUB_TOKEN`: Your GitHub token
   - `AXUR_API_TOKEN`: Your Axur API token
   - `DATABASE_URL`: Connection string (e.g., `postgresql://user:pass@host:port/dbname`)
   - `GH_PAT`: GitHub Personal Access Token (for logs)
   - `GH_OWNER`: GitHub username/org (e.g., `maisonnat`)
   - `GH_LOGS_REPO`: Log repository (e.g., `axur-logs-private`)
   - `AXUR_ADMIN_EMAIL`: Your email (e.g., `your_email@example.com`)

### 2. Cloudflare Credentials

1. Go to [Cloudflare Dashboard](https://dash.cloudflare.com)
2. Create a Cloudflare Pages project named `axtool`
3. Get your Account ID from the dashboard URL or Overview page
4. Create an API Token with "Cloudflare Pages" permissions
5. Add to GitHub: Settings → Secrets → Actions → New secrets
   - `CLOUDFLARE_ACCOUNT_ID`: Your account ID
   - `CLOUDFLARE_API_TOKEN`: Your API token

## URLs After Deployment

| Service | URL |
|---------|-----|
| Backend (Leapcell) | https://axur-tool-maisonnat2655-5j70lozi.leapcell.dev |
| Frontend (Cloudflare) | https://axtool.pages.dev |

## Manual Deployment

### Backend to Leapcell
Leapcell auto-deploys on push to main. Manual redeploy:
1. Go to Leapcell Dashboard
2. Click "Redeploy" on your service

### Frontend to Cloudflare
```bash
cd crates/frontend
trunk build --release
npx wrangler pages deploy dist --project-name=axtool
```

## Workflow Triggers

- **Backend deploy**: Leapcell watches GitHub repo, auto-deploys on push to `main`
- **Frontend deploy**: Triggers on push to `main` when `crates/frontend/` or `crates/core/` changes

## Environment Variables

### Build Time (Frontend)
- Uses Cloudflare Functions proxy (`functions/api/[[path]].js`) to forward requests to Leapcell

### Runtime (Backend - Leapcell Dashboard)
- `RUST_LOG`: Logging level (default: `axur_backend=info`)
- `GITHUB_TOKEN`: For feedback/issues
- `AXUR_API_TOKEN`: For report generation
