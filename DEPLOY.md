# Deployment Guide

## Overview

This project uses automated CI/CD deployment:
- **Backend**: Deployed to [Koyeb](https://koyeb.com) (Free tier)
- **Frontend**: Deployed to [Cloudflare Pages](https://pages.cloudflare.com) (Free)

## Prerequisites

Before deployment works, you need to configure GitHub Secrets.

### 1. Koyeb Setup

1. Create account at [Koyeb](https://app.koyeb.com) (no credit card needed)
2. Create a new App:
   - Name: `axur-backend`
   - Source: GitHub → `maisonnat/axur-tool`
   - Builder: Dockerfile
   - Port: 3001
   - Instance: Free (0.1 vCPU, 512MB RAM)
3. Get your API Token from Settings → API
4. Add to GitHub: Settings → Secrets → Actions → New secret
   - Name: `KOYEB_TOKEN`
   - Value: Your Koyeb API token

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
| Backend (Koyeb) | https://axur-backend-USERNAME.koyeb.app |
| Frontend (Cloudflare) | https://axtool.pages.dev |

## Manual Deployment

### Backend to Koyeb
The easiest way is via Koyeb Dashboard:
1. Connect GitHub repo
2. Select Dockerfile as builder
3. Deploy

### Frontend to Cloudflare
```bash
cd crates/frontend
trunk build --release
npx wrangler pages deploy dist --project-name=axtool
```

## Workflow Triggers

- **Backend deploy**: Triggers on push to `main` when `crates/backend/`, `crates/core/`, or `Dockerfile` changes
- **Frontend deploy**: Triggers on push to `main` when `crates/frontend/` or `crates/core/` changes

## Environment Variables

### Build Time (Frontend)
- `API_BASE_URL`: Set to production backend URL (e.g., `https://axur-backend-USERNAME.koyeb.app`)

### Runtime (Backend)
- `RUST_LOG`: Logging level (default: `axur_backend=info`)

