---
applyTo: "venues/src/**"
---

# Rate Limiting Implementation

**REFERENCE IMPLEMENTATION**: See `venues/src/gateio/rate_limit.rs` and `venues/src/gateio/rate_limiter_trait.rs` for the required pattern.

## Required Files and Structure

Each venue MUST implement rate limiting with these exact files at the venue root level:

### 1. `rate_limit.rs` - Rate Limiting Implementation

MUST contain:
- **`RateLimitHeader`** struct for parsing response headers
- **`RateLimitStatus`** struct for tracking rate limit state  
- **`RateLimiter`** struct implementing the venue's rate limiting logic
- **`UsageInfo`** struct for usage statistics
- Header parsing logic for venue-specific rate limit headers
- Endpoint categorization and limits

### 2. `rate_limiter_trait.rs` - Rate Limiter Trait

MUST define:
- **`<Venue>RateLimiter`** trait (e.g., `GateIoRateLimiter`) 
- Async methods for permit acquisition and header updates
- Usage statistics and warning methods
- Venue-specific rate limiting interface

## Implementation Requirements

### Rate Limiting Logic
- **Rate limiting logic MUST be completely separated from endpoint implementations**
- **Each venue MUST define its own rate limiting configuration** based on exchange specifications
- **Use tokio::sync::Semaphore** for permit-based rate limiting
- **Implement sliding window or token bucket algorithms** as appropriate for the venue

### Header Processing
- **Parse venue-specific rate limit headers** from API responses
- **Update internal rate limiting state** based on response headers
- **Handle rate limit resets and window boundaries** correctly

### Endpoint Categorization  
- **Categorize endpoints by rate limit groups** as defined by the exchange
- **Apply different limits per category** (e.g., public vs private endpoints)
- **Track usage per category** independently

### Performance Requirements
- **Rate limiting MUST NOT block request processing** unnecessarily
- **Use efficient permit acquisition** with minimal contention
- **Provide usage statistics and warnings** for monitoring

## Integration Pattern

```rust
// In endpoint implementations - acquire permit before request
let _permit = self.rate_limiter.get_permit(endpoint).await?;
let response = self.http_client.send(request).await?;

// Update rate limiter with response headers
self.rate_limiter.update_from_headers(&response.headers(), endpoint);
```

## Export Requirements

Rate limiting types MUST be exported from venue `mod.rs`:
```rust
pub use rate_limit::{RateLimitHeader, RateLimitStatus, RateLimiter, UsageInfo};
pub use rate_limiter_trait::<Venue>RateLimiter;
```
