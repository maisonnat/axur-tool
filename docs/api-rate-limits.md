# Axur API Rate Limits

Reference documentation for Axur API rate limits to avoid HTTP 429 errors.

## Documented Limits

| Endpoint Category | Rate Limit | Notes |
|------------------|------------|-------|
| **General** (Tickets, TH, etc.) | 60 requests/min | Most common limit |
| **Web Complaints** | 3 requests/second | Strict per-second limit |
| **IoC API** | 120 requests/min | Higher capacity |
| **Feed API** (per feedId) | 1 request/30 seconds | Per-resource limit |
| **Credentials API** | 1,150 requests/min | Highest capacity |

## Best Practices

### 1. Delay Between Requests
```rust
// Add 1 second delay between consecutive API calls
tokio::time::sleep(std::time::Duration::from_secs(1)).await;
```

### 2. Retry with Exponential Backoff (from Axur docs)
```python
MAX_RETRIES = 3
INITIAL_WAIT = 1.0  # seconds
WAIT_INCREMENT = 0.5  # increase per retry

if response.status_code == 429:
    time.sleep(wait_time)
    wait_time += WAIT_INCREMENT
    return make_request(retries + 1, wait_time)
```

### 3. Batch Operations
- For Threat Hunting Preview: Limit to **5 unique domains** max
- Process domains **sequentially**, not in parallel
- Add delay between each domain search

## Current Implementation Concerns

When searching 10 domains × 2 sources (Signal-Lake + Credentials):
- 20 POST requests (start searches)
- ~20 GET requests (poll results)
- Total: ~40+ requests in seconds

**Solution**: Sequential processing with 1-second delays.

## References

- File: `axur-api-doc/doc api axur 15 12 2025 (1).md`
- Rate limit handling example: Lines 3833-3867
- TH API docs: `axur-api-doc/TH API.md`
