# Binance Portfolio Margin API Client

This module provides a client implementation for the Binance Portfolio Margin API, following the same patterns and structure as the existing Coin-M Futures client.

## Overview

The Portfolio Margin API allows access to Binance's portfolio margin trading features through REST endpoints at `https://papi.binance.com/`.

## Features

- **Private REST Client**: Authenticated client for Portfolio Margin API endpoints
- **Public REST Client**: Unauthenticated client for public data
- **Rate Limiting**: Built-in rate limiting following Binance's guidelines
- **Error Handling**: Comprehensive error handling for API responses
- **Request Signing**: HMAC-SHA256 request signing for authenticated endpoints

## Usage

### Creating a Private Client

```rust
use venues::binance::portfolio_margin::{PrivateRestClient, RateLimiter};
use rest::secrets::SecretValue;
use secrecy::SecretString;

// Create rate limiter
let rate_limiter = RateLimiter::new();

// Create HTTP client
let http_client = reqwest::Client::new();

// Create Portfolio Margin private client
let client = PrivateRestClient::new(
    Box::new(SecretValue::new(SecretString::from("your_api_key"))),
    Box::new(SecretValue::new(SecretString::from("your_api_secret"))),
    "https://papi.binance.com",
    rate_limiter,
    http_client,
);
```

### Creating a Public Client

```rust
use venues::binance::portfolio_margin::{PublicRestClient, RateLimiter};

// Create rate limiter
let rate_limiter = RateLimiter::new();

// Create HTTP client
let http_client = reqwest::Client::new();

// Create Portfolio Margin public client
let client = PublicRestClient::new(
    "https://papi.binance.com",
    http_client,
    rate_limiter,
);
```

## API Endpoints

The base infrastructure is ready for implementing specific Portfolio Margin endpoints. To add new endpoints, follow the pattern used in the `coinm` module:

1. Create endpoint-specific modules in `private/rest/` or `public/rest/`
2. Define request and response structures
3. Implement methods on the `RestClient`
4. Export the new functionality in the module hierarchy

## Base URL

The Portfolio Margin API uses `https://papi.binance.com` as the base URL, as specified in the [Binance Portfolio Margin API documentation](https://developers.binance.com/docs/derivatives/portfolio-margin/general-info).

## Rate Limiting

The client includes the same rate limiting mechanisms as the Coin-M client:

- Raw requests: 61,000 per 5 minutes
- Request weight: 6,000 per minute
- Order limits: 100 per 10 seconds, 1,200 per minute

## Error Handling

The client reuses the same error types and handling as the Coin-M implementation, providing consistent error reporting across all Binance API clients.

## Examples

See `venues/examples/binanceportfoliomargin/` for a complete working example of creating and using the Portfolio Margin client.
