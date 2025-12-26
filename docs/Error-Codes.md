# Códigos de Error - Axur Tool

Esta página documenta todos los códigos de error del sistema para facilitar el diagnóstico de problemas.

## Formato de Códigos

Los códigos siguen el formato `MÓDULO-NNN`:

| Prefijo | Módulo | Descripción |
|---------|--------|-------------|
| `AUTH` | Autenticación | Errores de login, 2FA y sesión |
| `API` | API Axur | Problemas de conexión con la API de Axur |
| `RPT` | Reporte | Errores de generación de reportes |
| `TI` | Threat Intelligence | Búsquedas en Dark Web, viralidad, etc. |
| `NET` | Red | CORS, timeouts, DNS |
| `SYS` | Sistema | Errores internos del servidor |

---

## AUTH - Autenticación

| Código | Nombre | Descripción | Solución |
|--------|--------|-------------|----------|
| `AUTH-001` | Credenciales Inválidas | Email o contraseña incorrectos | Verificar credenciales de Axur |
| `AUTH-002` | 2FA Incorrecto | Código de autenticación de dos factores inválido | Esperar nuevo código o verificar sincronización de tiempo |
| `AUTH-003` | Sesión Expirada | La sesión actual ha expirado | Cerrar sesión y volver a ingresar |
| `AUTH-004` | Token Ausente | No hay token de autenticación en la solicitud | Iniciar sesión nuevamente |

---

## API - Conexión API Axur

| Código | Nombre | Descripción | Solución |
|--------|--------|-------------|----------|
| `API-001` | Token Expirado | El token de Axur ya no es válido | Re-autenticar con Axur |
| `API-002` | Tenant No Encontrado | El tenant seleccionado no existe | Verificar ID de tenant o contactar administrador |
| `API-003` | Rate Limit | Demasiadas solicitudes en poco tiempo | Esperar 5 minutos antes de reintentar |
| `API-004` | Respuesta Inválida | La API retornó datos inesperados | Reportar a soporte con detalles |
| `API-005` | Endpoint No Disponible | El endpoint de la API no responde | Verificar estado de Axur API |

---

## RPT - Generación de Reportes

| Código | Nombre | Descripción | Solución |
|--------|--------|-------------|----------|
| `RPT-001` | Sin Datos | No hay datos en el período seleccionado | Ampliar el rango de fechas |
| `RPT-002` | Sin Evidencias | No hay incidentes para la sección de evidencias | Normal si no hubo incidentes |
| `RPT-003` | Error de Renderizado | Fallo al generar el HTML del reporte | Reportar a soporte |
| `RPT-004` | Configuración Inválida | Falta tenant o rango de fechas | Completar todos los campos requeridos |

---

## TI - Threat Intelligence

| Código | Nombre | Descripción | Solución |
|--------|--------|-------------|----------|
| `TI-001` | Timeout Dark Web | La búsqueda en Dark Web tardó demasiado | Reintentar o reducir rango de fechas |
| `TI-002` | API No Disponible | El servicio de Threat Hunting no responde | Esperar y reintentar |
| `TI-003` | Sin Credenciales | No se encontraron credenciales filtradas | Normal si no hay filtraciones |
| `TI-004` | Búsqueda Cancelada | El usuario canceló la búsqueda | Reiniciar si fue accidental |

---

## NET - Red

| Código | Nombre | Descripción | Solución |
|--------|--------|-------------|----------|
| `NET-001` | CORS Bloqueado | Política de CORS bloquea la solicitud | Contactar administrador para revisar configuración |
| `NET-002` | Timeout | La conexión excedió el tiempo de espera | Verificar conexión a internet |
| `NET-003` | DNS Fallido | No se pudo resolver el dominio | Verificar configuración de red |
| `NET-004` | Error SSL/TLS | Problema con el certificado de seguridad | Verificar fecha/hora del sistema |

---

## SYS - Sistema

| Código | Nombre | Descripción | Solución |
|--------|--------|-------------|----------|
| `SYS-001` | Error Interno | Error inesperado del servidor | Reportar a soporte con código y contexto |
| `SYS-002` | Configuración | Error de configuración del sistema | Contactar administrador |
| `SYS-003` | Recursos Agotados | Memoria o CPU insuficiente | Reiniciar servidor o contactar soporte |

---

## Cómo Reportar un Error

Cuando contactes a soporte, incluye:

1. **Código de error** (ej: `TI-001`)
2. **Fecha y hora** del error
3. **Tenant** que estabas usando
4. **Rango de fechas** del reporte
5. **Pasos** para reproducir el problema

Esto ayuda a identificar rápidamente la causa del problema.

---

## Changelog

| Fecha | Cambio |
|-------|--------|
| 2024-12-24 | Versión inicial de códigos de error |
