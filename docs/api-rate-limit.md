# Axur API Rate Limit Rules

Cuando escribas o modifiques código que interactúe con la API de Axur, SIEMPRE aplica estas reglas:

## Regla 1: Delay Entre Requests
- Añade `tokio::time::sleep(Duration::from_secs(1))` entre llamadas consecutivas a la API
- Para operaciones en lotes, considera 2 segundos de delay

## Regla 2: Retry con Backoff para 429
- Implementa retry automático cuando recibas HTTP 429
- Máximo 3 reintentos
- Usa backoff exponencial: 1s → 2s → 4s

## Regla 3: Límites de Requests
- General/TH: máximo 60 requests/minuto
- No más de 5 requests concurrentes a la vez
- Procesar secuencialmente es preferible a paralelo

## Regla 4: Batching
- Limita operaciones bulk a máximo 5-10 items por lote
- Para Threat Hunting Preview: máximo 5 dominios únicos

## Regla 5: Documentar Consumo
- Cuando crees funciones que hacen múltiples llamadas API, documenta el estimado de requests
- Ejemplo: `// Esta función hace ~20 requests (10 POST + 10 GET polling)`

## Código Obligatorio para API Calls

```rust
// Patrón obligatorio para loops con API calls
for item in items {
    let result = api_call(item).await?;
    tokio::time::sleep(std::time::Duration::from_secs(1)).await; // OBLIGATORIO
}
```

```rust
// Patrón obligatorio para manejo de 429
if resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
    tracing::warn!("Rate limited, waiting before retry...");
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    // retry logic...
}
```
