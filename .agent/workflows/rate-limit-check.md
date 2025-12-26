---
description: Review code for Axur API rate limit compliance
---

# API Rate Limit Compliance Check

Use this workflow to verify that code interacting with Axur APIs respects the documented rate limits.

## Pre-Check: Read Rate Limits

1. **Read the rate limits documentation**
   - View `docs/api-rate-limits.md` to refresh on current limits
   - Key limits to remember:
     - General: **60 requests/min**
     - Credentials: **1,150 requests/min**
     - Feed: **1 request/30s** per feedId

## Step 1: Find API Call Patterns

2. **Search for HTTP client usage**
   ```
   grep -rn "\.post\|\.get" crates/core/src/api/
   grep -rn "reqwest" crates/
   ```

3. **Look for loops making API calls**
   ```
   grep -rn "for.*await" crates/core/src/api/
   ```

## Step 2: Check for Rate Limit Protections

4. **Verify delays exist between requests**
   - Look for `tokio::time::sleep` or similar delays
   - Minimum recommended: 1 second between requests
   - For bulk operations: consider 2 seconds

5. **Check for retry logic on 429 errors**
   - Look for handling of `status_code == 429`
   - Should include exponential backoff

## Step 3: Compliance Checklist

Verify each API interaction has:

- [ ] **Delay between sequential requests** (min 1 second)
- [ ] **Retry logic for 429 errors** (max 3 retries with backoff)
- [ ] **Request batching** where possible (reduce total calls)
- [ ] **Pagination limits** (don't fetch all pages at once)
- [ ] **Concurrent request limits** (max 3-5 parallel requests)

## Step 4: Calculate Request Volume

6. **For preview/batch operations, calculate:**
   - Number of unique items to process
   - Requests per item (POST + GET polling)
   - Total expected requests
   - Time required with delays

   Example for Threat Hunting Preview:
   - 10 domains × 2 sources = 20 POST requests
   - 20 searches × up to 20 polls each = 400 GET requests
   - With 1s delay: ~7 minutes minimum

## Step 5: Report Findings

7. **Create a summary listing:**
   - Functions that make API calls
   - Whether they have rate limit protections
   - Recommendations for improvements

## Code Patterns to Implement

### Good Pattern: Sequential with Delay
```rust
for domain in domains {
    let result = make_api_call(domain).await?;
    tokio::time::sleep(Duration::from_secs(1)).await; // Rate limit protection
}
```

### Good Pattern: Retry with Backoff
```rust
let mut attempts = 0;
let mut wait_time = Duration::from_secs(1);

loop {
    let resp = client.get(&url).send().await?;
    if resp.status() == 429 && attempts < 3 {
        tokio::time::sleep(wait_time).await;
        wait_time *= 2; // Exponential backoff
        attempts += 1;
        continue;
    }
    break;
}
```

### Bad Pattern: Rapid Fire Requests
```rust
// DON'T DO THIS - will trigger 429 errors
for domain in domains {
    tasks.push(tokio::spawn(make_api_call(domain)));
}
futures::future::join_all(tasks).await; // All at once!
```
