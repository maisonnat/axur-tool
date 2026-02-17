# Milestones Tracking

## Overview

| ID | Milestone | Estado | Fecha Inicio | Fecha Fin |
|----|-----------|--------|--------------|-----------|
| M1 | i18n Loader + 5 keys | ✅ Completado | 2026-01-05 | 2026-01-05 |
| M2 | Migrar resto keys EN | ✅ Completado | 2026-01-05 | 2026-01-05 |
| M3 | ES + PT-BR JSONs | ✅ Completado | 2026-01-05 | 2026-01-05 |
| M4 | Plugin traits | ✅ Completado | 2026-01-05 | 2026-01-05 |
| M5 | 1 slide como plugin | ✅ Completado | 2026-01-05 | 2026-01-05 |
| M6 | Resto de slides | ✅ Completado (14/14) | 2026-01-05 | 2026-01-05 |

---

## M1: i18n Loader + 5 Keys Migradas

**Objetivo**: Crear la infraestructura base del nuevo sistema i18n y probar con 5 keys.

**Estado**: ✅ Completado (2026-01-05)

### Checklist

- [x] Crear directorio `crates/core/translations/`
- [x] Crear `translations/en.json` con 5 keys iniciales
- [x] Crear `crates/core/src/i18n/mod.rs` con nuevo loader  
- [x] Crear wrapper de compatibilidad para coexistencia
- [x] Probar que keys migradas funcionan
- [x] Verificar fallback a legacy para keys no migradas

### Keys Migradas en M1

```
welcome_message
login_prompt_email
footer_text
metrics_title
metrics_total_tickets
```

### Resultados

- Build: ✅ Exitoso
- Tests: ✅ 5/5 pasaron
- Legacy compatibility: ✅ Funciona

---

## M2: Migrar Resto de Keys EN

**Objetivo**: Completar migración de todas las keys al JSON de inglés.

### Checklist

- [ ] Identificar todas las keys del trait `Dictionary` (~100)
- [ ] Agregar al `translations/en.json`
- [ ] Actualizar llamadas en código para usar nuevo sistema
- [ ] Eliminar valores hardcoded del trait English

### Definición de Completado

- [ ] `impl Dictionary for English` puede ser eliminado
- [ ] Todas las keys funcionan desde JSON
- [ ] No hay `[MISSING: xxx]` en reportes generados

---

## M3: ES + PT-BR JSONs

**Objetivo**: Crear archivos JSON para español y portugués.

### Checklist

- [ ] Crear `translations/es.json` basado en impl Spanish actual
- [ ] Crear `translations/pt-br.json` basado en impl Portuguese actual
- [ ] Verificar interpolación funciona en todos los idiomas
- [ ] Eliminar `impl Dictionary for Spanish/Portuguese`

### Definición de Completado

- [ ] Reportes generan correctamente en 3 idiomas
- [ ] Código legacy de idiomas eliminado
- [ ] Archivo i18n.rs reducido significativamente

---

## M4: Plugin Traits

**Objetivo**: Implementar sistema de plugins sin migrar slides aún.

### Checklist

- [ ] Crear `crates/core/src/plugins/mod.rs`
- [ ] Crear `traits.rs` con `SlidePlugin`, `DataPlugin`
- [ ] Crear `registry.rs` con `PluginRegistry`
- [ ] Crear directorio `builtin/` vacío
- [ ] Tests unitarios para registry

### Definición de Completado

- [ ] Traits compilando y documentados
- [ ] Registry puede registrar y listar plugins
- [ ] Tests pasan

---

## M5: Primera Slide como Plugin

**Objetivo**: Migrar una slide sencilla para validar el patrón.

### Slide Candidata: Metrics

La slide de métricas es buena candidata porque:
- Estructura simple
- Datos claros (total_tickets, threats_detected, time_saved)
- No tiene lógica compleja

### Checklist

- [ ] Crear `builtin/metrics.rs`
- [ ] Implementar `SlidePlugin` para MetricsSlidePlugin
- [ ] Integrar en report generation
- [ ] Verificar output HTML idéntico al anterior

### Definición de Completado

- [ ] Slide de métricas generada por plugin
- [ ] Output HTML visualmente idéntico
- [ ] Código legacy de métricas comentado/removido

---

## M6: Resto de Slides + Cleanup

**Objetivo**: Migrar slides restantes y eliminar código legacy.

### Slides a Migrar

1. CoverSlidePlugin
2. IntroSlidePlugin
3. ThreatChartPlugin
4. StealerAnalysisPlugin
5. TakedownsPlugin
6. ROIPlugin
7. ClosingPlugin
8. ... (resto según html.rs)

### Checklist

- [ ] Migrar cada slide a su plugin
- [ ] Eliminar generación legacy de html.rs
- [ ] Actualizar imports y exports
- [ ] Documentation final

### Definición de Completado

- [ ] Todas las slides son plugins
- [ ] html.rs usa solo PluginRegistry
- [ ] No hay código legacy duplicado
- [ ] Documentación actualizada
