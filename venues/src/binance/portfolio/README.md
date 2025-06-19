# Binance Portfolio Margin API Client

This module provides a high-performance, idiomatic Rust client for the [Binance Portfolio Margin API](https://developers.binance.com/docs/derivatives/portfolio-margin/general-info), following the same structure and conventions as the Coin-M Futures client.

---

## 📚 Source Documentation

- [Binance Portfolio Margin API Docs](https://developers.binance.com/docs/derivatives/portfolio-margin/general-info)
- [Binance Portfolio Margin REST Endpoints](https://developers.binance.com/docs/derivatives/portfolio-margin/portfolio-margin-rest-api)
- [Binance Portfolio Margin Rate Limits](https://developers.binance.com/docs/derivatives/portfolio-margin/general-info#rate-limits)

---

## 🔐 Authentication

- **Type:** API Key + Secret (passed as `SecretString`, securely stored)
- **Required for:** All private endpoints (see below)
- **How:** Credentials must be provided as `impl Into<SecretString>` and are never stored as plain strings.

---

## 🏗️ Structure

- **Public endpoints:** `public/rest/` (unauthenticated)
- **Private endpoints:** `private/rest/` (API Key + Secret required)
- **REST client struct:** `RestClient` (in both public and private modules)
- **Rate limiting:** Shared, venue-specific, and separated from endpoint logic
- **Enums:** All fixed-value fields use enums defined in [`enums.rs`](enums.rs)

---

## 🚀 Features

- **Private REST Client:** Authenticated access to all Portfolio Margin endpoints
- **Public REST Client:** Unauthenticated access to public endpoints
- **Rate Limiting:** Built-in, identical to COIN-M, see [`specs/rate_limiting.md`](specs/rate_limiting.md)
- **Error Handling:** Comprehensive, with venue-specific error enums and response structs
- **Request Signing:** HMAC-SHA256 for all signed endpoints
- **Idiomatic Rust:** Uses enums, `Cow<'static, str>`, and structured logging (`tracing`)

---

## ✅ Implemented Endpoints

| Endpoint Type | Path/Module     | REST Client Method | Auth Required |
| ------------- | --------------- | ------------------ | ------------- |
| Public        | `public/rest/`  | `RestClient`       | No            |
| Private       | `private/rest/` | `RestClient`       | Yes           |

> **Note:** To add new endpoints, create a new file in the appropriate `public/rest/` or `private/rest/` directory, define request/response structs and enums, and implement the method on the `RestClient` in that file.

---

## 🛡️ Rate Limiting

- **Raw requests:** 61,000 per 5 minutes
- **Request weight:** 6,000 per minute
- **Order limits:** 100 per 10 seconds, 1,200 per minute
- **Headers:** `X-MBX-USED-WEIGHT-1M`, `X-MBX-ORDER-COUNT-10S`, `X-MBX-ORDER-COUNT-1M`
- **See:** [`specs/rate_limiting.md`](specs/rate_limiting.md) for details

---

## ⚠️ Error Handling

- All errors are mapped to venue-specific enums and structs in [`errors.rs`](errors.rs)
- Each HTTP status code and API error code is mapped to a specific error variant
- Error messages are preserved from the API
- See [`errors.rs`](errors.rs) for details

---

## 📝 Usage

### Creating a Private Client

```rust
use venues::binance::portfolio::{PrivateRestClient, RateLimiter};
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
use venues::binance::portfolio::{PublicRestClient, RateLimiter};

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
