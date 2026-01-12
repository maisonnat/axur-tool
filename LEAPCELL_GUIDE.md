# 🚀 Deploying to Leapcell (Docker Method)

This guide explains how to deploy the `axur-backend` using pre-built Docker images to **avoid Leapcell build time limits**.

## Architecture

```
GitHub Push
    │
    ├─► deploy-cloudflare.yml ─► Cloudflare Pages (Frontend)
    │
    └─► deploy-backend.yml ─► Docker Hub ─► Leapcell (Backend)
```

## Deployment Order

| Step | Service | Time | Notes |
|------|---------|------|-------|
| 1 | GitHub Actions | ~5-10 min | Builds Rust, pushes Docker image |
| 2 | Docker Hub | Instant | Stores `maisonnat/axur-backend:latest` |
| 3 | Leapcell | ~30s | Pulls and runs pre-built image |
| 4 | Cloudflare | ~2-3 min | Builds WASM frontend |

## Setup Steps

### 1. Docker Hub Secrets

Add these secrets to GitHub repo (Settings → Secrets → Actions):

| Secret | Value | How to Get |
|--------|-------|------------|
| `DOCKERHUB_USERNAME` | `maisonnat` | Your Docker Hub username |
| `DOCKERHUB_TOKEN` | `dckr_pat_...` | Docker Hub → Account Settings → Security → Access Tokens |

### 2. Configure Leapcell for Docker

1. Go to [Leapcell Dashboard](https://leapcell.io/dashboard)
2. Select your service
3. Change **Deployment Method** to **Docker Image**
4. Set:
   - **Image**: `maisonnat/axur-backend:latest`
   - **Port**: `3001`
5. Keep environment variables as before

### 3. Environment Variables (Leapcell)

| Key | Value | Description |
|-----|-------|-------------|
| `RUST_LOG` | `axur_backend=info,tower_http=info` | Logging level |
| `DATABASE_URL` | *Your Supabase/Postgres URL* | Database connection |
| `GITHUB_TOKEN` | *Your GitHub Token* | For feedback issues |
| `AXUR_API_TOKEN` | *Your Axur Token* | For fetching reports |

## Benefits

| Before (Rust Builder) | After (Docker) |
|----------------------|----------------|
| ~30 min build time | 0 min build time |
| Consumes 1h/month limit | No limit consumed |
| Slower deploys | Faster deploys |

## Manual Deploy

If you need to deploy manually:

```bash
# 1. Build locally
cargo build --release --bin axur-backend
cp target/release/axur-backend ./axur-backend

# 2. Build and push Docker
docker build -t maisonnat/axur-backend:latest .
docker push maisonnat/axur-backend:latest

# 3. Redeploy in Leapcell dashboard
```
