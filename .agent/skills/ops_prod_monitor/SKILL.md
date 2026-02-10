---
name: ops_prod_monitor
description: Production monitoring to verify deployed services are healthy.
version: 1.0.0
status: active
---

# Production Monitor

**Objective**: Verify production deployments are healthy after push to main.

## When to Use
- Immediately after `git push origin main`
- After GitHub Actions complete
- For periodic health checks

## Production URLs

| Service | URL | Platform |
|---------|-----|----------|
| Frontend | https://axtool.pages.dev | Cloudflare Pages |
| Backend | https://axur-backend-*.run.app | GCP Cloud Run |

## Procedure

### 1. Wait for Deploy
After `git push`, wait for GitHub Actions:
```powershell
# Check workflow status (requires gh CLI)
gh run list --limit 5
```

Typical deploy times:
- Frontend (Cloudflare): ~2-3 minutes
- Backend (GCP): ~5-7 minutes

### 2. Frontend Health Check
```
browser_subagent Task:
"Navigate to https://axtool.pages.dev, verify the page loads,
check for any error messages, and confirm the main application is visible.
Return success/failure with details."
```

### 3. Backend Health Check
```powershell
# Health endpoint
curl https://YOUR-CLOUDRUN-URL/api/health

# Or via frontend proxy
curl https://axtool.pages.dev/api/health
```

### 4. Version Verification
Confirm the deployed version matches the pushed commit:
```powershell
# Get deployed version
curl https://axtool.pages.dev/api/version

# Compare with local
git rev-parse --short HEAD
```

## Monitoring Checklist

| Check | Command | Expected |
|-------|---------|----------|
| Frontend loads | browser_subagent | Page visible, no errors |
| API responds | curl /api/health | 200 OK |
| Version matches | curl /api/version | Same as git HEAD |
| SSL valid | browser check | No certificate warnings |

## Rollback Procedure

If production is broken:
```powershell
# Revert last commit
git revert HEAD --no-edit
git push origin main

# Wait for redeploy
gh run watch
```

## Alerts

> [!CAUTION]
> If production is down, prioritize rollback over debugging.
> Debug locally after service is restored.

## Post-Deploy Checklist

- [ ] Frontend loads at axtool.pages.dev
- [ ] API health returns 200
- [ ] Critical user flows work
- [ ] No console errors
- [ ] Version matches pushed commit
