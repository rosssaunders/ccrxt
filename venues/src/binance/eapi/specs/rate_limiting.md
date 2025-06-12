# Binance Options API (EAPI) Rate Limiting

This document describes the rate limiting implementation for Binance Options API endpoints, which use the `/eapi/v1/` prefix.

## Rate Limiting Overview

The Binance Options API implements the same three-bucket rate limiting system as other Binance APIs:

1. **Raw Requests** - Every REST call counts as 1, no exceptions
2. **Request Weight** - Each endpoint has a specific weight value
3. **Orders** - Order creation, cancellation, and modification operations

## Rate Limits

### Raw Requests
- **Limit**: 61,000 requests per 5-minute window
- **Scope**: Per IP address
- **Tracking**: Client-side only (no server header)

### Request Weight  
- **Limit**: 6,000 weight units per 1-minute window
- **Scope**: Per IP address  
- **Header**: `X-MBX-USED-WEIGHT-1M` (and other intervals like 3M, 1H, 1D)
- **Reset**: Rolling window, resets as time passes

### Orders
- **Limits**: 
  - 100 orders per 10-second window
  - 1,200 orders per 1-minute window
- **Scope**: Per account
- **Headers**: `X-MBX-ORDER-COUNT-10S` and `X-MBX-ORDER-COUNT-1M`

## Error Responses

When rate limits are exceeded, the API will return:

- **HTTP 429** - Too Many Requests
- **HTTP 418** - IP Auto-Banned (after repeated 429s without backing off)

## Usage

```rust
use venues::binance::eapi::{RateLimiter, ResponseHeaders};

// Create a rate limiter instance
let limiter = RateLimiter::new();

// Check limits before making a request
match limiter.check_limits(weight, is_order).await {
    Ok(()) => {
        // Safe to make request
        // ... make HTTP request ...
        
        // After successful request, update counters
        limiter.increment_raw_request().await;
        if is_order {
            limiter.increment_order().await;
        }
        
        // Update from response headers (authoritative)
        limiter.update_from_headers(&response_headers).await;
    }
    Err(e) => {
        // Rate limit would be exceeded, handle error
        eprintln!("Rate limit check failed: {}", e);
    }
}
```

## Headers

The following response headers contain authoritative rate limit information:

- `X-MBX-USED-WEIGHT-1M` - Current weight usage in 1-minute window
- `X-MBX-USED-WEIGHT-1H` - Current weight usage in 1-hour window  
- `X-MBX-USED-WEIGHT-1D` - Current weight usage in 1-day window
- `X-MBX-ORDER-COUNT-10S` - Current order count in 10-second window
- `X-MBX-ORDER-COUNT-1M` - Current order count in 1-minute window

## Best Practices

1. **Always check limits** before making requests using `check_limits()`
2. **Update counters** after each request using `increment_raw_request()` and `increment_order()`
3. **Sync with headers** using `update_from_headers()` to stay accurate with server state
4. **Implement exponential backoff** when receiving 429 responses
5. **Use WebSockets** for market data when possible to reduce REST API load

## Notes

- Rate limits are enforced per IP address for weight/raw requests and per account for orders
- The server headers are authoritative - always sync your local counters with them
- Repeated violations can result in temporary IP bans that escalate from 2 minutes to 3 days
- VIP account tiers may have higher limits than the defaults shown here