# Portfolio Margin Rate Limiting

Portfolio Margin trading uses **identical rate limits** to COIN-M Futures:

## Rate Limits

### IP Limits

- **Portfolio Margin IP Limit**: 6000/min (same as COIN-M)
- **Raw requests**: 61,000 per 5 min
- **Request weight**: 6,000 per 1 min per-IP

### Order Limits

- **Portfolio Margin Order Limits**: 1200/min (same as COIN-M)
- **Orders**: 100 per 10s, 1,200 per 1m

## Implementation

The Portfolio Margin rate limiter reuses the COIN-M rate limiting implementation since the limits are identical:

```rust
use venues::binance::portfolio_margin::PortfolioMarginRateLimiter;

// Create a rate limiter
let limiter = PortfolioMarginRateLimiter::new();

// Check limits before making a request
limiter.check_limits(weight, is_order).await?;

// Increment counters after requests
limiter.increment_raw_request().await;
if is_order {
    limiter.increment_order().await;
}

// Update from response headers (authoritative)
limiter.update_from_headers(&response.headers).await;
```

## Error Handling

Portfolio Margin rate limiting returns the same error types as COIN-M:

- `TooManyRequests`: When request weight limit (6000/min) is exceeded
- `TooManyOrders`: When order limits (100/10s or 1200/min) are exceeded
- HTTP 429: Rate limit exceeded
- HTTP 418: IP banned for continued violations

## Headers

Portfolio Margin uses the same rate limiting headers as COIN-M:

- `X-MBX-USED-WEIGHT-1M`: Current used weight for 1 minute window
- `X-MBX-ORDER-COUNT-10S`: Order count for 10 second window
- `X-MBX-ORDER-COUNT-1M`: Order count for 1 minute window

## Best Practices

1. **Pre-check limits**: Always call `check_limits()` before making requests
2. **Track usage**: Increment counters after each request
3. **Respect headers**: Update counters from response headers (authoritative)
4. **Use WebSockets**: For market data to avoid REST rate limits
5. **Implement backoff**: When receiving 429 errors

See the [COIN-M rate limiting documentation](../coinm/specs/rate_limiting.md) for detailed explanations of the rate limiting system.
