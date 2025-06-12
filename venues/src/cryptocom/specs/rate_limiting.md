# Crypto.com Rate Limiting

Implementation of rate limiting for the crypto.com exchange API based on their documented rate limits.

## Rate Limits

### REST API - Authenticated Calls (per API method, per API key)

| Method | Limit |
|--------|-------|
| `private/create-order` | 15 requests per 100ms |
| `private/cancel-order` | 15 requests per 100ms |
| `private/cancel-all-orders` | 15 requests per 100ms |
| `private/get-order-detail` | 30 requests per 100ms |
| `private/get-trades` | 1 request per second |
| `private/get-order-history` | 1 request per second |
| All others | 3 requests per 100ms |

### REST API - Public Market Data (per API method, per IP address)

| Method | Limit |
|--------|-------|
| `public/get-book` | 100 requests per second |
| `public/get-ticker` | 100 requests per second |
| `public/get-trades` | 100 requests per second |
| `public/get-valuations` | 100 requests per second |
| `public/get-candlestick` | 100 requests per second |
| `public/get-insurance` | 100 requests per second |

### Staking

| Method | Limit |
|--------|-------|
| `public/staking/*` | 50 requests per second |
| `private/staking/*` | 50 requests per second |

### WebSocket

| Type | Limit |
|------|-------|
| User API | 150 requests per second |
| Market Data | 100 requests per second |

## Implementation

The rate limiter tracks requests per endpoint type using rolling time windows. Each endpoint type has its own independent rate limit.

### Usage Example

```rust
use venues::cryptocom::{RateLimiter, EndpointType};

// Create a rate limiter
let limiter = RateLimiter::new();

// Before making a request, check if it's allowed
limiter.check_limits(EndpointType::PrivateCreateOrder).await?;

// Make your API request here...

// After making the request, increment the counter
limiter.increment_request(EndpointType::PrivateCreateOrder).await;
```

### Error Handling

The rate limiter returns `RateLimitError::RateLimitExceeded` when limits are exceeded:

```rust
match limiter.check_limits(endpoint_type).await {
    Ok(()) => {
        // Proceed with request
    },
    Err(RateLimitError::RateLimitExceeded { endpoint, current, max, window }) => {
        // Handle rate limit exceeded
        eprintln!("Rate limit exceeded for {:?}: {}/{} requests in {:?}", 
                  endpoint, current, max, window);
    }
}
```

### Cleanup

The rate limiter automatically cleans up old timestamps when new requests are made. For long-running applications, you may want to periodically call `cleanup_old_timestamps()`:

```rust
// Clean up old timestamps for all endpoints
limiter.cleanup_old_timestamps().await;
```

## Best Practices

1. **Check before request**: Always call `check_limits()` before making API requests
2. **Increment after request**: Call `increment_request()` after successful API calls
3. **Handle errors gracefully**: Implement exponential backoff when rate limits are exceeded
4. **Per-endpoint tracking**: Different endpoints have different rate limits - track them independently
5. **WebSocket considerations**: For WebSocket connections, crypto.com recommends adding a 1-second sleep after establishing the connection before sending requests

## Important Notes

- Private REST endpoints are rate limited per API key
- Public REST endpoints are rate limited per IP address
- WebSocket rate limits are pro-rated based on the calendar-second that the connection was opened
- The implementation uses rolling time windows, not fixed windows
- Rate limits are enforced independently for each endpoint type

## References

- [Crypto.com Exchange API Documentation](https://exchange-docs.crypto.com/exchange/v1/rest-ws/index.html#rate-limits)