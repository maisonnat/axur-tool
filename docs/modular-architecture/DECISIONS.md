# Decision Log

Registro de decisiones arquitecturales importantes para esta iniciativa.

---

## ADR-001: No Migrar a WASI

**Fecha**: 2026-01-05  
**Estado**: Aceptada

### Contexto

Se evaluó migrar a arquitectura WASI con wasmtime y wit-bindgen para sandboxing de componentes.

### Decisión

No implementar WASI ahora.

### Razones

1. **Scope de proyecto**: Web app de reportes no requiere sandboxing de código externo
2. **Limitaciones técnicas**: reqwest y tokio no funcionan nativamente en WASI
3. **Costo/Beneficio**: 6-8 semanas de trabajo sin beneficio directo
4. **Complejidad**: Agrega complejidad operacional significativa

### Alternativa Elegida

Config-driven i18n + Plugin traits (más simple, suficiente para extensibilidad).

### Puerta Abierta

Si en el futuro se necesita sandbox real (plugins de terceros), la arquitectura de plugins permite migrar componentes específicos a WASI.

---

## ADR-002: i18n con JSON Files

**Fecha**: 2026-01-05  
**Estado**: Aceptada

### Contexto

Sistema i18n actual tiene ~2000 líneas de código con traits hardcodeados.

### Decisión

Migrar a archivos JSON con loader en runtime.

### Formato Elegido

```json
{
  "key": "valor",
  "key_with_interpolation": "Hola {name}",
  "_meta": {
    "language": "es",
    "version": "1.0.0"
  }
}
```

### Trade-offs

| Pro | Con |
|-----|-----|
| Fácil agregar idiomas | Errores en runtime vs compile-time |
| Non-devs pueden traducir | Necesita validación de schema |
| Menor código | Slightly más overhead |

### Mitigación

- Schema validation en CI
- Fallback a legacy durante transición
- Tests para verificar todas las keys existen

---

## ADR-003: Plugin Traits vs Dynamic Loading

**Fecha**: 2026-01-05  
**Estado**: Aceptada

### Contexto

Se necesita sistema extensible para agregar slides/features.

### Opciones Evaluadas

1. **Dynamic loading (dlopen)**: Plugins como .so/.dll
2. **Trait objects**: Plugins como `Box<dyn SlidePlugin>`
3. **Feature flags**: Cargo features para habilitar/deshabilitar

### Decisión

Trait objects con registro estático.

### Razones

1. Type-safe en compile time
2. No requiere manejo de FFI
3. Fácil de testear
4. Suficiente para el caso de uso actual

### Futuro

Si se necesita hot-reload de plugins, se puede migrar a dynamic loading o WASI componets.

---

## ADR-004: Migración Incremental con Compatibilidad

**Fecha**: 2026-01-05  
**Estado**: Aceptada

### Contexto

Migración debe permitir trabajo en paralelo con fixes de producción.

### Decisión

1. Trabajar en feature branch
2. Sistema legacy coexiste durante transición
3. Wrapper intenta nuevo sistema → fallback a legacy
4. Merge por milestone completado

### Beneficios

- Producción nunca se rompe
- Rollback fácil si hay problemas
- Testing gradual de cada componente migrado
