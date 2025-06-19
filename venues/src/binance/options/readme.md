# Binance Options (EAPI) Rust Venue

This crate provides a high-performance, fully typed Rust client for the Binance Options (EAPI) REST API, including robust rate limiting, error handling, and enums for all fixed-value fields.

---

## 📚 Source Documentation

- [Binance Options API Docs](https://binance-docs.github.io/apidocs/voptions/en/)
- [Binance Options API Changelog](https://binance-docs.github.io/apidocs/voptions/en/#change-log)
- [Binance Options Rate Limits](https://binance-docs.github.io/apidocs/voptions/en/#limits)

---

## 🔐 Authentication

- **Public endpoints:** No authentication required.
- **Private endpoints:** (Not yet implemented in this module) will require API Key + Secret, passed as `SecretString` (see project credential handling policy).

---

## 🚦 Rate Limiting

- Implements Binance's three-bucket system: Raw Requests, Request Weight, and Orders.
- All rate limiting logic is separated from endpoint code and uses a venue-specific configuration.
- See [`specs/rate_limiting.md`](specs/rate_limiting.md) for details.

---

## 🏗️ File Structure

- All endpoint files are under `public/` or `private/` subdirectories.
- REST client: [`public/rest/client.rs`](public/rest/client.rs) (struct: `RestClient`)
- Common REST logic: [`rest/common.rs`](rest/common.rs)
- Rate limiting: [`rate_limit.rs`](rate_limit.rs)
- Enums: [`enums.rs`](enums.rs)
- Errors: [`errors.rs`](errors.rs)

---

## ✅ Endpoints Implemented

### Public REST Endpoints

- [`RestClient`](public/rest/client.rs): Generic request support for all public endpoints.
- (Add specific endpoint files as they are implemented.)

### Rate Limiting

- Full support for all Binance Options API rate limits, including header parsing and rolling window logic.

### Error Handling

- All error codes and messages mapped to strongly typed enums.
- See [`errors.rs`](errors.rs) for details.

---

## 🧩 Usage Example

```rust
use venues::binance::options::{PublicRestClient, RateLimiter};
use reqwest::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let rate_limiter = RateLimiter::new();
    let rest = PublicRestClient::new("https://eapi.binance.com", client, rate_limiter);

    // Example: call a public endpoint (replace with actual endpoint)
    // let resp = rest.send_request::<YourResponseType>("/eapi/v1/ping", reqwest::Method::GET, None, None, 1).await.unwrap();
    // println!("{:?}", resp.data);
}
```
