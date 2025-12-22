# Axur Web - Reference Guide from CLI Project

> Este archivo contiene todas las referencias al proyecto CLI para desarrollar la versión web.

---

## 🎯 Prompt para el Otro Proyecto

Copia y pega esto en la otra conversación:

```
Estoy desarrollando Axur Web, una versión web Full Rust del CLI de Axur.

## Stack
- Backend: Axum (Rust) → Shuttle.rs (free tier)
- Frontend: Leptos (Rust→WASM) → Cloudflare Pages (free)
- Core compartido: Library crate con lógica reutilizable

## Referencia del CLI
El proyecto CLI de referencia está en:
c:\Users\maiso\.gemini\antigravity\playground\azimuthal-opportunity\axur_cli_rust\

### Archivos a reutilizar (copiar y adaptar):

| Archivo | Ruta en CLI | Descripción |
|---------|-------------|-------------|
| API principal | src/api/report.rs (73KB) | fetch_available_tenants(), fetch_full_report(), TenantInfo, PocReportData |
| Retry logic | src/api/retry.rs (2KB) | Backoff exponencial |
| i18n | src/i18n.rs (44KB) | Dictionary trait, EN/ES/PT traducciones |
| HTML templates | src/report/html.rs (71KB) | generate_full_report_html() |
| Errors | src/errors.rs (2KB) | CliError con recovery hints |

### API Axur
- Base URL: https://api.axur.com/gateway/1.0/api
- Auth: Bearer token
- Endpoints usados:
  - GET /customers/customers → lista de tenants
  - POST /customerx/auth → login
  - GET /stats/threats → métricas
  - GET /tickets → tickets con paginación

### Patrones clave del CLI:
1. `Dictionary` trait para i18n → usar igual en web
2. `TenantInfo { key, name }` → selector de tenants
3. `FullReport` struct → datos para HTML
4. Token validation via API call antes de usar session

### OWASP:
- JWT con httpOnly cookies (no localStorage)
- CSP headers
- Input validation con validator crate
- CORS configurado

### Diseño (axur.com):
- Colors: #EF4043 (red), #0F172A (dark)
- Fonts: Inter, DM Mono
- Zero friction UX

Continúa desde: crear el backend con Axum, endpoints de auth y report.
```

---

## 📂 Rutas Absolutas de Referencia

### Proyecto CLI (fuente)
```
c:\Users\maiso\.gemini\antigravity\playground\azimuthal-opportunity\axur_cli_rust\
├── Cargo.toml              # v0.4.0
├── src\
│   ├── main.rs             # Entry point, tenant picker flow
│   ├── i18n.rs             # ⭐ 44KB - Traducciones completas
│   ├── config.rs           # Config struct
│   ├── errors.rs           # ⭐ CliError enum
│   ├── api\
│   │   ├── mod.rs          # API_URL, create_client()
│   │   ├── report.rs       # ⭐ 73KB - Toda la lógica API
│   │   └── retry.rs        # Retry con backoff
│   ├── auth\
│   │   ├── mod.rs          # Login flow, token validation
│   │   └── storage.rs      # Keyring (NO usar en web)
│   └── report\
│       ├── mod.rs
│       └── html.rs         # ⭐ 71KB - Templates HTML
```

### Proyecto Web (destino)
```
c:\Users\maiso\.gemini\antigravity\playground\azimuthal-opportunity\axur-web\
├── Cargo.toml              # Workspace
├── crates\
│   ├── core\               # Código compartido
│   │   └── src\
│   │       ├── lib.rs
│   │       ├── api\        # Copiar de CLI
│   │       ├── report\     # Copiar de CLI
│   │       ├── i18n.rs     # Copiar de CLI
│   │       └── errors.rs   # Copiar de CLI
│   ├── backend\            # Axum server
│   └── frontend\           # Leptos app
```

---

## 🔑 Estructuras de Datos Importantes

### TenantInfo (api/report.rs)
```rust
pub struct TenantInfo {
    pub key: String,   // ID del tenant (ej: "MGLZ")
    pub name: String,  // Nombre (ej: "Magazine Luiza")
}
```

### Language (i18n.rs)
```rust
pub enum Language { En, Es, PtBr }
```

### Dictionary trait (i18n.rs)
```rust
pub trait Dictionary {
    fn threats_title(&self) -> String;
    fn credentials_title(&self) -> String;
    // ... 100+ métodos de traducción
}
```

---

## 🌐 Endpoints de la API Axur

| Endpoint | Método | Uso |
|----------|--------|-----|
| `/customerx/auth` | POST | Login (email, password) |
| `/customerx/auth/confirmation` | POST | 2FA confirmation |
| `/customers/customers` | GET | Lista de tenants |
| `/stats/threats` | GET | Métricas de amenazas |
| `/tickets?limit=X` | GET | Lista de tickets |

---

## ⚠️ Dependencias a Remover para Web

| Dependencia CLI | Reemplazar con |
|-----------------|----------------|
| `dialoguer` | Leptos components |
| `indicatif` (spinners) | Loading states CSS |
| `console` (colors) | CSS styling |
| `keyring` | httpOnly cookies |
| `dirs` | Browser storage/server config |
