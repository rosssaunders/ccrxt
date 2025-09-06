# Coinbase Exchange API Implementation

This module provides a comprehensive implementation of the Coinbase Exchange private REST API, featuring:

## Features

- **Rate Limiter**: Implements Coinbase-specific rate limits (15 req/s for private endpoints, 10 req/s for fills/loans)
- **Shared Enums**: Common trading enums (OrderSide, OrderType, TimeInForce, etc.)
- **Error Modeling**: Comprehensive error handling with Coinbase-specific error mapping
- **Private REST Client**: Full authentication with HMAC SHA256 signatures
- **Get Balances Endpoint**: Implementation of the account balances endpoint with cursor pagination support

## Quick Start

```rust
use venues::coinbase::{PrivateRestClient, RateLimiter, GetAccountBalancesRequest};
use secrets::SecretValue;
use secrecy::SecretString;

// Create the client
let client = PrivateRestClient::new(
    api_key,
    api_secret,      // Base64 encoded
    api_passphrase,
    "https://api.exchange.coinbase.com",
    reqwest::Client::new(),
    RateLimiter::new(),
);

// Get account balances
let request = GetAccountBalancesRequest::default();
let balances = client.get_account_balances(&request).await?;
```

## Authentication

The Coinbase Exchange API requires:

- **CB-ACCESS-KEY**: Your API key
- **CB-ACCESS-SIGN**: HMAC SHA256 signature (base64 encoded)
- **CB-ACCESS-TIMESTAMP**: Unix timestamp
- **CB-ACCESS-PASSPHRASE**: Your API passphrase

The signature is created by:

1. Concatenating: timestamp + method + requestPath + body
2. Signing with HMAC SHA256 using your base64-decoded secret
3. Base64 encoding the result

## Rate Limits

The implementation follows Coinbase's rate limits:

- **Public endpoints**: 10 requests/second (burst: 15)
- **Private endpoints**: 15 requests/second (burst: 30)
- **Private /fills**: 10 requests/second (burst: 20)
- **Private /loans**: 10 requests/second

## Error Handling

The API provides detailed error mapping for common Coinbase errors:

- Invalid price
- Insufficient funds
- Invalid order size
- Authentication errors
- Rate limit exceeded

## Testing

Run the Coinbase tests with:

```bash
cargo test coinbase
```

## API Reference

### Enums

- `OrderSide`: Buy, Sell
- `OrderType`: Limit, Market, Stop
- `TimeInForce`: GTC, GTT, IOC, FOK
- `SelfTradePrevention`: dc, co, cn, cb
- `OrderStatus`: Open, Pending, Rejected, Done, Active, Received, All

### Endpoints

- `get_account_balances()`: Get all account balances with cursor pagination support (before/after cursors)

### Rate Limiter

- `check_limit()`: Check if request can be made
- `wait_for_capacity()`: Wait until request capacity is available

## Security

- API credentials are handled securely using the `secrecy` crate
- All secrets are properly zeroed from memory
- Base64 decoding is handled safely with error checking

## Sandbox Testing

For testing, use the sandbox URLs:

- **REST API**: `https://api-public.sandbox.exchange.coinbase.com`
- **WebSocket**: `wss://ws-feed-public.sandbox.exchange.coinbase.com`

Create sandbox credentials at [Coinbase Pro Sandbox](https://public.sandbox.exchange.coinbase.com).
