---
name: ops_local_dev
description: Local development workflow with controlled promotion to production.
version: 1.0.0
status: active
---

# Local Development → Production Workflow

**Objective**: Desarrollar localmente y promover cambios a producción de forma controlada.

## Architecture

```
LOCAL                                    PRODUCTION
┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
│  START   │───►│   WORK   │───►│  VERIFY  │───►│  PROMOTE │
│          │    │          │    │          │    │          │
│ dev.ps1  │    │ commits  │    │ tests +  │    │ dry-run  │
│ backend  │    │ locales  │    │ pre-fly  │    │ → push   │
│ frontend │    │ (no push)│    │          │    │          │
└──────────┘    └──────────┘    └──────────┘    └──────────┘
                                                     │
                                              ┌──────┴──────┐
                                              ▼             ▼
                                         Cloudflare    GCP Cloud
                                          (frontend)   (backend)
```

---

## PHASE 1: START

Levantar el entorno de desarrollo local.

```powershell
.\dev.ps1
```

**Resultado esperado**:
- Backend en `http://localhost:3001`
- Frontend en `http://localhost:8080`

---

## PHASE 2: WORK

Desarrollar features/fixes con commits locales.

```bash
# Hacer cambios en el código
# Commit local (SIN PUSH)
git add -A
git commit -m "feat: descripción del cambio"
```

> [!WARNING]
> **NO hacer `git push`** hasta completar PHASE 3 y 4.

---

## PHASE 3: VERIFY

Antes de promover, verificar que todo funciona.

### 3.1 Tests
```powershell
cargo test --workspace
```

### 3.2 Pre-flight Check
Ejecutar el workflow de validación:
```powershell
# Verificar que no hay tokens/secrets expuestos
# Ver .agent/workflows/pre-flight-check.md
```

### 3.3 Build Check
```powershell
cargo build --release --workspace
```

---

## PHASE 4: PROMOTE (Production Gate)

### 4.1 Dry-Run (OBLIGATORIO)
Ver qué commits se subirán:

```powershell
# Commits pendientes de push
git log origin/main..HEAD --oneline

# Archivos modificados vs producción
git diff --stat origin/main
```

### 4.2 Confirmación Visual
Revisar el output del dry-run. Preguntar:
- ¿Los commits reflejan el trabajo de la sesión?
- ¿Hay archivos inesperados (tokens, .env, etc.)?

### 4.3 Push a Producción
Solo si el usuario confirma:

```powershell
git push origin main
```

**Resultado**: GitHub Actions despliega automáticamente:
- Frontend → Cloudflare Pages
- Backend → GCP Cloud Run

---

## Quick Reference

| Fase | Comando | Descripción |
|------|---------|-------------|
| START | `.\dev.ps1` | Levantar entorno local |
| WORK | `git commit` (sin push) | Guardar cambios localmente |
| VERIFY | `cargo test --workspace` | Ejecutar tests |
| DRY-RUN | `git log origin/main..HEAD --oneline` | Ver commits pendientes |
| PROMOTE | `git push origin main` | Desplegar a producción |

---

## Rollback

Si algo sale mal después del push:

```powershell
# Revertir último commit en producción
git revert HEAD
git push origin main
```
