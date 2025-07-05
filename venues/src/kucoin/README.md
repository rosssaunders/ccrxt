# KuCoin Venue Implementation

This module provides a Rust implementation for interacting with the KuCoin exchange API.

## Documentation

- **Source Documentation**: [KuCoin API Documentation](https://docs.kucoin.com/)
- **Authentication Type**: API Key + Secret + Passphrase (HMAC-SHA256)
- **API Version**: v1 and v2

## Authentication

KuCoin requires three components for authentication:

- **API Key**: Your API key from KuCoin
- **API Secret**: Your secret key for signing requests
- **API Passphrase**: Your passphrase (encrypted with the secret)

Authentication headers required:

- `KC-API-KEY`: Your API key
- `KC-API-SIGN`: HMAC-SHA256 signature
- `KC-API-TIMESTAMP`: Unix timestamp in milliseconds
- `KC-API-PASSPHRASE`: HMAC-SHA256 encrypted passphrase
- `KC-API-KEY-VERSION`: Version 2

## Implemented Endpoints

### Public REST API

- ✅ **Server Time** - `GET /api/v1/timestamp`
- ✅ **Currencies** - `GET /api/v1/currencies` and `GET /api/v1/currencies/{currency}`
- ✅ **Symbols** - `GET /api/v1/symbols`
- ✅ **Ticker Statistics** - `GET /api/v1/market/orderbook/level1` and `GET /api/v1/market/allTickers`
- ✅ **Order Book** - `GET /api/v1/market/orderbook/level2_{20|100}` and `GET /api/v1/market/orderbook/level2`
- ✅ **Trade History** - `GET /api/v1/market/histories`
- ✅ **Klines/Candlesticks** - `GET /api/v1/market/candles`

### Private REST API

- ✅ **Place Order** - `POST /api/v1/hf/orders`
- ✅ **Cancel Order** - `DELETE /api/v1/hf/orders`
- ✅ **Cancel All Orders** - `DELETE /api/v1/hf/orders` (with filters)

## Usage Examples

### Public API

```rust
use venues::kucoin::public::{RestClient, GetServerTimeRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RestClient::new_default();
    
    // Get server time
    let (response, _) = client.get_server_time(GetServerTimeRequest::default()).await?;
    println!("Server time: {}", response.timestamp);
    
    Ok(())
}
```

### Private API

```rust
use venues::kucoin::private::{RestClient, PlaceOrderRequest};
use venues::kucoin::{OrderSide, OrderType};
use rest::secrets::SecretValue;
use secrecy::SecretString;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = SecretValue::new(SecretString::new("your_api_key".to_string()));
    let api_secret = SecretValue::new(SecretString::new("your_api_secret".to_string()));
    let passphrase = SecretValue::new(SecretString::new("your_passphrase".to_string()));
    
    let client = RestClient::new_with_credentials(api_key, api_secret, passphrase);
    
    let request = PlaceOrderRequest {
        side: OrderSide::Buy,
        symbol: "BTC-USDT".to_string(),
        order_type: OrderType::Limit,
        price: Some("50000.0".to_string()),
        size: Some("0.001".to_string()),
        ..Default::default()
    };
    
    let (response, _) = client.place_order(request).await?;
    println!("Order placed: {}", response.order_id);
    
    Ok(())
}
```

## Rate Limiting

KuCoin has various rate limits:

- Public endpoints: Generally more lenient
- Private endpoints: More restrictive

The rate limiter is built into the client and will prevent requests when limits are approached.

## Error Handling

All methods return a `Result<T, KucoinError>` where `KucoinError` includes:

- HTTP errors
- API errors with specific error codes
- Authentication failures
- Rate limit violations
- JSON parsing errors

## Sandbox Environment

For testing, you can use the sandbox environment:

```rust
let client = RestClient::new_sandbox(api_key, api_secret, passphrase);
```

## Feature Completeness

This implementation covers the most commonly used endpoints for spot trading. Additional endpoints can be added as needed following the same patterns established in this codebase.

# KuCoin Rate Limiter

A comprehensive rate limiting implementation for the KuCoin API that follows KuCoin's resource pool-based rate limiting system.

## Overview

KuCoin uses a sophisticated rate limiting system with different resource pools based on endpoint types and VIP levels. This implementation accurately models KuCoin's rate limiting behavior as documented in their API specifications.

## Key Features

- **VIP Level Support**: All VIP levels (0-12) with accurate rate limits
- **Resource Pool Management**: 6 different resource pools (Spot, Futures, Management, Earn, CopyTrading, Public)
- **Automatic Endpoint Classification**: Determines the correct resource pool based on endpoint path
- **30-Second Windows**: Follows KuCoin's 30-second rate limit windows
- **Concurrent Safe**: Thread-safe implementation using async RwLock
- **Header Parsing**: Extracts rate limit information from KuCoin response headers

## Resource Pools and Limits

| VIP Level | Spot (incl. Margin) | Futures | Management | Earn | CopyTrading | Public |
|-----------|--------------------:|--------:|-----------:|-----:|------------:|-------:|
| VIP0      | 4,000/30s          | 2,000/30s | 2,000/30s | 2,000/30s | 2,000/30s | 2,000/30s |
| VIP1      | 6,000/30s          | 2,000/30s | 2,000/30s | 2,000/30s | 2,000/30s | 2,000/30s |
| VIP2      | 8,000/30s          | 4,000/30s | 4,000/30s | 2,000/30s | 2,000/30s | 2,000/30s |
| VIP3      | 10,000/30s         | 5,000/30s | 5,000/30s | 2,000/30s | 2,000/30s | 2,000/30s |
| VIP4      | 13,000/30s         | 6,000/30s | 6,000/30s | 2,000/30s | 2,000/30s | 2,000/30s |
| VIP5      | 16,000/30s         | 7,000/30s | 7,000/30s | 2,000/30s | 2,000/30s | 2,000/30s |
| VIP6      | 20,000/30s         | 8,000/30s | 8,000/30s | 2,000/30s | 2,000/30s | 2,000/30s |
| VIP7      | 23,000/30s         | 10,000/30s | 10,000/30s | 2,000/30s | 2,000/30s | 2,000/30s |
| VIP8      | 26,000/30s         | 12,000/30s | 12,000/30s | 2,000/30s | 2,000/30s | 2,000/30s |
| VIP9      | 30,000/30s         | 14,000/30s | 14,000/30s | 2,000/30s | 2,000/30s | 2,000/30s |
| VIP10     | 33,000/30s         | 16,000/30s | 16,000/30s | 2,000/30s | 2,000/30s | 2,000/30s |
| VIP11     | 36,000/30s         | 18,000/30s | 18,000/30s | 2,000/30s | 2,000/30s | 2,000/30s |
| VIP12     | 40,000/30s         | 20,000/30s | 20,000/30s | 2,000/30s | 2,000/30s | 2,000/30s |

## Usage

### Basic Usage

```rust
use venues::kucoin::rate_limit::{RateLimiter, ResourcePool, VipLevel};

// Create rate limiter for VIP 5
let mut rate_limiter = RateLimiter::new_with_vip(VipLevel::Vip5);

// Check if we can make a spot trading request with weight 2
match rate_limiter.check_limits(ResourcePool::Spot, 2).await {
    Ok(()) => {
        // Proceed with API request
        println!("Request approved");
    }
    Err(e) => {
        println!("Rate limit exceeded: {}", e);
    }
}
```

### Automatic Endpoint Classification

```rust
// The rate limiter can automatically determine the resource pool
let pool = ResourcePool::from_endpoint_path("/api/v1/orders");
assert_eq!(pool, ResourcePool::Spot);

let pool = ResourcePool::from_endpoint_path("/api/v1/contracts/XBTUSDM");
assert_eq!(pool, ResourcePool::Futures);
```

### Checking Rate Limit Status

```rust
// Get status for a specific resource pool
if let Some(status) = rate_limiter.get_status(ResourcePool::Spot).await {
    println!("Spot: {}/{} used, {} remaining, reset in {}ms",
             status.used, status.limit, status.remaining, status.reset_time_ms);
}

// Get all statuses
let all_statuses = rate_limiter.get_all_statuses().await;
for (pool, status) in all_statuses {
    println!("{:?}: {}/{} requests", pool, status.used, status.limit);
}
```

### VIP Level Management

```rust
// Update VIP level (preserves current usage)
rate_limiter.update_vip_level(VipLevel::Vip12).await;

// Check current VIP level
let current_vip = rate_limiter.vip_level();
```

### Advanced Features

```rust
// Check if request can proceed without consuming quota
let can_proceed = rate_limiter.check_can_proceed(ResourcePool::Spot, 100).await;

// Wait for rate limit if needed (with backoff)
rate_limiter.wait_if_needed(ResourcePool::Spot, 50).await?;

// Parse rate limit headers from KuCoin response
let headers = response.headers();
let rate_limit_info = RateLimitHeader::from_headers(headers);
```

## Error Handling

```rust
use venues::kucoin::rate_limit::RateLimitError;

match rate_limiter.check_limits(ResourcePool::Spot, weight).await {
    Ok(()) => {
        // Request approved
    }
    Err(RateLimitError::Exceeded { pool, used, limit }) => {
        println!("Rate limit exceeded for {:?}: {}/{}", pool, used, limit);
        // Wait or retry later
    }
    Err(RateLimitError::ServerOverload) => {
        println!("Server overload, retry later");
        // Exponential backoff recommended
    }
}
```

## Integration with HTTP Clients

```rust
async fn make_kucoin_request(
    client: &reqwest::Client,
    rate_limiter: &RateLimiter,
    endpoint: &str,
    weight: u32,
) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    // Determine resource pool from endpoint
    let pool = ResourcePool::from_endpoint_path(endpoint);
    
    // Check rate limits before making request
    rate_limiter.check_limits(pool, weight).await?;
    
    // Make the HTTP request
    let response = client.get(&format!("https://api.kucoin.com{}", endpoint))
        .send()
        .await?;
    
    // Parse rate limit headers for monitoring
    let rate_limit_info = RateLimitHeader::from_headers(response.headers());
    println!("Rate limit remaining: {:?}", rate_limit_info.remaining);
    
    Ok(response)
}
```

## Testing

The rate limiter includes comprehensive tests:

```bash
cargo test kucoin::rate_limit
```

## Resource Pool Classification

The rate limiter automatically classifies endpoints into resource pools:

- **Spot**: Trading endpoints including orders, margin, isolated margin, HF orders, OCO orders
- **Futures**: Contracts, positions, trade history, funding history
- **Management**: Accounts, sub-accounts, user info, deposits, withdrawals, transfers
- **Earn**: Earn products, lending, margin lending
- **CopyTrading**: Copy trading related endpoints
- **Public**: All other endpoints (market data, symbols, etc.)

## Performance

- Lock-free quota checking for read operations
- Efficient window-based reset mechanism
- Minimal memory footprint per resource pool
- Async-friendly design with no blocking operations

## Compliance

This implementation follows KuCoin's official rate limiting documentation:
- [KuCoin Rate Limit Documentation](https://www.kucoin.com/docs-new/rate-limit)
- Resource pool based quotas
- 30-second sliding windows
- VIP level specific limits
- Proper error codes (429000)
