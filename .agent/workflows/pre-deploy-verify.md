---
description: Verification workflow before deploying changes
---

# Pre-Deploy Verification Workflow

// turbo-all

## 1. Build Check
```bash
cargo build --release -p axur-backend
```

## 2. Run Local Backend (if not running)
```bash
# In separate terminal
cargo run -p axur-backend
```

## 3. Test Critical Endpoints
```bash
curl http://localhost:8080/api/health
curl http://localhost:8080/api/templates -H "Cookie: axur_session=test"
```

## 4. Verify Frontend Builds
```bash
cd crates/frontend && trunk build --release
```

## 5. Browser Test Checklist
- [ ] Open http://localhost:8080 in browser
- [ ] Login flow works
- [ ] Generate a report preview
- [ ] Language selector buttons work (click ES/EN/PT)
- [ ] Download HTML and verify it works standalone
- [ ] Check browser console for errors (F12)

## 6. Pre-flight Security Check
Run the pre-flight workflow:
```bash
git diff HEAD~1 --name-only
```
Verify no secrets or API keys in diff.

## 7. Commit and Push
Only after all checks pass:
```bash
git add .
git commit -m "feat: description"
git push origin main
```

## 8. Monitor Deployment
- Check GitHub Actions for build status
- Wait for GCP deploy (5-8 min)
- Verify prod health: `curl https://axur-backend-844146909418.us-central1.run.app/api/health`
