# Modular Architecture Initiative

> **Estado**: 🟡 En Progreso  
> **Última Actualización**: 2026-01-05  
> **Milestone Actual**: M1 - i18n Loader + 5 keys

## Objetivo

Refactorizar la arquitectura de axur-web para soportar extensibilidad sin modificar el código core:
- **Config-Driven i18n**: Traducciones en archivos JSON
- **Plugin System**: Slides y features como plugins registrables

## Documentos de Esta Iniciativa

| Documento | Propósito |
|-----------|-----------|
| [README.md](./README.md) | Este archivo - overview y estado |
| [DESIGN.md](./DESIGN.md) | Diseño técnico detallado |
| [MILESTONES.md](./MILESTONES.md) | Tracking de progreso por milestone |
| [DECISIONS.md](./DECISIONS.md) | Log de decisiones arquitecturales |
| [HANDOFF.md](./HANDOFF.md) | Instrucciones para continuar el trabajo |

## Quick Status

```
M1 [✅ Completado] i18n Loader + 5 keys migradas
M2 [✅ Completado] Migrar resto de keys EN (~100 keys)
M3 [✅ Completado] Crear ES + PT-BR JSONs (~100 keys c/u)
M4 [✅ Completado] Plugin traits (SlidePlugin, DataPlugin, ExportPlugin)
M5 [✅ Completado] MetricsSlidePlugin migrada
M6 [✅ Completado] 14 slides migradas a plugins
```

## Contexto de Decisión

### ¿Por qué no WASI?
Se evaluó migración a WASI/wasmtime pero se descartó porque:
- El proyecto es una web app de reportes, no necesita sandboxing de código externo
- reqwest/tokio no funcionan nativamente en WASI
- Overhead de 6-8 semanas sin beneficio directo

### ¿Por qué esta arquitectura?
- Agregar idioma = 1 archivo JSON (vs 600 líneas Rust)
- Agregar feature = 1 plugin file (vs modificar html.rs)
- Compatible con trabajo en paralelo (feature branch)
- Deja puerta abierta a WASI futuro si se necesita

## Cómo Continuar Este Trabajo

Ver [HANDOFF.md](./HANDOFF.md) para instrucciones completas.
