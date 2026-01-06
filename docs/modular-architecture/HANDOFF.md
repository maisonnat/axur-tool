# Handoff Document

Instrucciones para continuar este trabajo desde cualquier punto.

---

## Estado Actual

**Fecha**: 2026-01-05  
**Milestone**: M1 - i18n Loader + 5 keys  
**Branch**: `main` (crear `feature/modular-architecture` al iniciar trabajo)

---

## Para Continuar el Trabajo

### 1. Leer Documentación

```
docs/modular-architecture/
├── README.md       ← Overview y estado actual
├── DESIGN.md       ← Diseño técnico detallado
├── MILESTONES.md   ← Tracking de progreso (ACTUALIZAR AQUÍ)
├── DECISIONS.md    ← Por qué tomamos ciertas decisiones
└── HANDOFF.md      ← Este documento
```

### 2. Verificar Estado Actual

```bash
# Ver qué milestone está en progreso
cat docs/modular-architecture/MILESTONES.md | head -20

# Ver si hay branch de trabajo
git branch -a | grep modular

# Ver últimos cambios relacionados
git log --oneline --all | head -20
```

### 3. Crear/Cambiar a Branch de Trabajo

```bash
git checkout -b feature/modular-architecture
# o si ya existe:
git checkout feature/modular-architecture
git pull origin main  # sincronizar con main
```

### 4. Continuar Según Milestone

Ver [MILESTONES.md](./MILESTONES.md) para el checklist del milestone actual.

---

## Archivos Clave

| Archivo | Propósito |
|---------|-----------|
| `crates/core/src/i18n.rs` | Sistema legacy (a migrar) |
| `crates/core/src/i18n/mod.rs` | Nuevo loader (a crear/continuar) |
| `crates/core/translations/*.json` | Archivos de traducción (a crear) |
| `crates/core/src/plugins/` | Sistema de plugins (futuro) |
| `crates/core/src/report/html.rs` | Generación de slides (a refactorizar) |

---

## Patrones a Seguir

### Para i18n

```rust
// Nuevo sistema
let trans = Translations::load("en")?;
let text = trans.get("metrics_title");
let formatted = trans.format("threats_desc", &[("total", "42")]);
```

### Para Plugins

```rust
// Registrar plugin
registry.register_slide(Box::new(MyCustomPlugin));

// Implementar plugin
impl SlidePlugin for MyCustomPlugin {
    fn id(&self) -> &'static str { "custom.my-plugin" }
    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        // ...
    }
}
```

---

## Comandos Útiles

```bash
# Build
cargo build -p axur-core

# Test específico
cargo test -p axur-core i18n

# Verificar que no rompimos nada
cargo test --all

# Frontend (para probar reportes)
cd crates/frontend && trunk serve
```

---

## Preguntas Frecuentes

### ¿Puedo hacer fixes de producción mientras trabajo en esto?

Sí. Ver sección "Branching Strategy" en DECISIONS.md (ADR-004).

```bash
git stash
git checkout main
# ... hacer fix ...
git commit && git push
git checkout feature/modular-architecture
git cherry-pick <commit>
git stash pop
```

### ¿Cómo sé si un milestone está completo?

Ver "Definición de Completado" en cada milestone en MILESTONES.md.

### ¿Debo actualizar esta documentación?

**Sí.** Después de cada sesión de trabajo:
1. Actualizar estado en `README.md`
2. Marcar items completados en `MILESTONES.md`
3. Agregar decisiones importantes a `DECISIONS.md`
4. Actualizar `HANDOFF.md` si hay info nueva relevante

---

## Contacto/Contexto

Esta iniciativa fue diseñada para:
- Permitir agregar idiomas sin código Rust
- Permitir agregar features como plugins sin modificar core
- Mantener compatibilidad con producción durante migración

El objetivo final es que **cualquier desarrollador** (incluso no-Rust) pueda:
- Agregar un nuevo idioma copiando un JSON
- Agregar una slide nueva implementando un trait simple
