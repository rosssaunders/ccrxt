# Binance Spot API Rate Limiting

This document outlines the rate limiting implementation for the Binance Spot API.

## Rate Limit Overview

The Binance Spot API uses three independent rate limiting "buckets":

| Limiter Type | What it measures | Spot API Limits | Where you see the meter |
|--------------|------------------|-----------------|-------------------------|
| `raw_request` | Every single REST call, weight = 1, no exceptions | 6,000 requests per 5 minutes | No header – you must count client-side |
| `request_weight` | The weighted sum shown in each endpoint's "Request Weight" line | 1,200 weight units per minute per-IP | X-MBX-USED-WEIGHT-1M |
| `orders` | Number of orders created / canceled / amended | 100 per 10 seconds, 1,000 per 24 hours | X-MBX-ORDER-COUNT-10S and -24H |

## Key Differences from COIN-M Futures

| Limiter | Spot API | COIN-M Futures |
|---------|----------|----------------|
| Raw Requests | 6,000 / 5 min | 61,000 / 5 min |
| Request Weight | 1,200 / 1 min | 6,000 / 1 min |
| Orders | 100 / 10s, 1,000 / 24h | 100 / 10s, 1,200 / 1 min |

## How Requests are Tallied

Each endpoint has different weights and affects different buckets:

| Example endpoint | Adds to raw_request | Adds to request_weight | Adds to orders |
|------------------|--------------------|-----------------------|----------------|
| GET /api/v3/time (weight = 1) | +1 | +1 | +0 |
| GET /api/v3/depth?limit=1000 (weight = 10) | +1 | +10 | +0 |
| POST /api/v3/order (place 1 order, weight = 1) | +1 | +1 | +1 |
| DELETE /api/v3/order (cancel) | +1 | +1 | +1 |

## Error Handling

When rate limits are exceeded:
- **HTTP 429**: Rate limit exceeded (includes Retry-After header when applicable)
- **HTTP 418**: IP ban for repeated violations (escalates from 2 minutes to 3 days)

## Headers

The API returns these headers to track usage:
- `X-MBX-USED-WEIGHT-(intervalNum)(intervalLetter)`: Current weight usage
- `X-MBX-ORDER-COUNT-(intervalNum)(intervalLetter)`: Current order count
- `Retry-After`: Seconds to wait (in 429/418 responses)

## Implementation Notes

The rate limiter tracks usage in rolling time windows and provides:
- `check_limits(weight, is_order)`: Validates before making requests
- `increment_raw_request()`: Updates raw request counter
- `increment_order()`: Updates order counters
- `update_from_headers()`: Syncs with authoritative server headers

## Usage Example

```rust
use venues::binance::spot::RateLimiter;

let limiter = RateLimiter::new();

// Before making a market data request (weight = 5)
limiter.check_limits(5, false).await?;

// Before placing an order (weight = 1)
limiter.check_limits(1, true).await?;

// After each request
limiter.increment_raw_request().await;

// After order requests
limiter.increment_order().await;

// After receiving response headers
limiter.update_from_headers(&response_headers).await;
```