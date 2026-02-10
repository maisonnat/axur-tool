# Deployment Guide

## 🌍 Infrastructure Overview
- **Backend:** Google Cloud Run (Docker Container)
    - **Region:** us-central1
    - **Project:** axur-backend-free
- **Frontend:** Cloudflare Pages
    - **Project:** axtool
- **Database:** Firebase/Firestore (NoSQL)

## 🚀 How to Deploy

### Automatic (GitOps)
Just push to the `main` branch. GitHub Actions handles the rest.

```bash
git add .
git commit -m "feat: amazing new feature"
git push origin main
```

- **Backend Workflow:** `.github/workflows/deploy-gcp.yml`
- **Frontend Workflow:** `.github/workflows/deploy-cloudflare.yml`

### Manual Verification (Backend)
To check the status of the backend service:

```powershell
gcloud run services list --platform managed --region us-central1
```

## 🛠️ Diagnostics

### Backend (GCP)
- View logs: `gcloud logging read "resource.type=cloud_run_revision AND resource.labels.service_name=axur-backend" --limit 20`
- Check health: `curl https://<SERVICE_URL>/api/health`

### Frontend (Cloudflare)
- Dashboard: [Cloudflare Dashboard](https://dash.cloudflare.com)
- Production URL: `https://axtool.pages.dev`
