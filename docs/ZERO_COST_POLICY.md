# ZERO COST & SECURITY POLICY (STRICT)

**Objetivo:** Mantener el costo operativo del proyecto en **$0.00 (Zero Tier)** indefinidamente y proteger las credenciales.
**Violación de esta política:** PROHIBIDA sin autorización explícita y escrita del propeitario.

---

## 1. Reglas de Infraestructura (Google Cloud)
Para mantener el **Always Free Tier**, toda configuración DEBE cumplir:

### A. Cloud Run (Backend)
- **Región:** OBLIGATORIO `us-central1` (Iowa), `us-east1`, o `us-west1`.
  - *Prohibido:* `southamerica-east1` (Brasil) u otras regiones Tier 2.
- **Instancias:**
  - `min-instances`: 0 (Permitir "Scaling to Zero").
  - `max-instances`: 1 (Evita escalado costoso por tráfico inesperado).
- **Recursos:**
  - CPU: 1 vCPU.
  - Memoria: 512 MiB (Suficiente para Rust).

### B. Artifact Registry (Docker Images)
- **Límite Estricto:** Máximo 500 MB de almacenamiento promedio mensual.
- **Política de Limpieza (Lifecycle Policy):**
  - OBLIGATORIO configurar borrado automático.
  - Mantener máximo las últimas **3 imágenes**.
  - Script de deploy debe incluir paso de limpieza (`gcloud artifacts docker images delete ...`).

### C. Base de Datos
- **Prohibido:** Usar Cloud SQL (PostgreSQL/MySQL) de Google (Costo > $15/mes).
- **Permitido:** Bases de datos externas gratuitas (Leapcell, Supabase, Neon) conectadas vía URL.

### D. Seguridad de Presupuesto
- OBLIGATORIO tener activa una **Budget Alert** en GCP.
- Umbral: R$ 5.00 (o $1.00 USD).
- Notificar al 50%, 90% y 100%.

---

## 2. Reglas de Seguridad (Repositorio Público)
El código vive en GitHub público. **NUNCA** subir:

1.  **Credenciales GCP:** Archivos `.json` de Service Accounts (ej: `gcp_key.json`, `adc.json`).
2.  **Tokens:** `AXUR_API_TOKEN` u otros secretos.
3.  **Connection Strings:** `DATABASE_URL` con contraseñas reales.
4.  **Archivos de Config Local:** `.env`, carpetas `.gemini`, logs de debug.
5.  **IPs o Dominios Privados:** No hardcodear IPs de servidores de staging.

**Mecanismo de Protección:**
- Usar **GitHub Secrets** para todo lo sensible.
- Revisar `.gitignore` antes de cada commit masivo.
