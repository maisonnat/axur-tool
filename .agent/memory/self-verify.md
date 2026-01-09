

# Self-Verification Protocol

Cada vez que programes o modifiques código, SIEMPRE:

## Después de cambios de código
1. **Compilar** el crate afectado: `cargo build -p <crate>`
2. **Ejecutar tests** del módulo: `cargo test -p <crate> <module>`
3. Si falla, **corregir** (máximo 3 intentos) antes de continuar
4. Si no puedes resolver en 3 intentos, **documentar** y preguntar al usuario

## Ciclo de Fix
```
Error → Analizar → Fix mínimo → Re-verificar → Continuar o Re-intentar
```

## Niveles de Verificación
- JSON/Config: Solo compilar
- Rust module: Compilar + tests
- Frontend: Compilar + `trunk build`
- API changes: Tests + smoke test manual

## Documentar Fallos Persistentes
Si algo no funciona después de 3 intentos:
1. Documentar en MILESTONES.md o notificar al usuario
2. No continuar al siguiente milestone hasta resolver o recibir guía

