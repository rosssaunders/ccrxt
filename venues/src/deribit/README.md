# Deribit Venue API

This venue implements **Deribit's public API** (REST and WebSocket) for use in the `venues` crate, following project conventions for structure, error handling, and rate limiting.

---

## 📚 Documentation

- [Deribit API Reference](https://docs.deribit.com/)
- [Deribit WebSocket API](https://docs.deribit.com/#websocket-api)
- [Deribit REST API](https://docs.deribit.com/#rest-api)

---

## 🔐 Authentication

- **Public endpoints:** No authentication required.
- **Private endpoints:** _Not implemented in this venue module._  
  If implemented, authentication would use API Key + Secret (see [Deribit authentication docs](https://docs.deribit.com/#authentication)).

---

## 🗂️ Implemented Endpoints

### REST (public/rest/)

- `/public/get_time` – Get server time
- `/public/test` – Test connection, returns API version
- `/public/status` – Platform lock status and locked currencies
- `/public/get_combo_ids` – List of combo instrument IDs
- `/public/get_combos` – Detailed combo information

### WebSocket (public/websocket/)

- `/public/hello` – Introduce client software to the platform

---

## 🚫 Private Endpoints

**Private endpoints are implemented in this venue.**

You can find private REST endpoints under `venues/src/deribit/private/rest/` and private WebSocket endpoints under `venues/src/deribit/private/websocket/`.

### Implemented Private REST Endpoints (`private/rest/`)

- `/private/get_account_summary` – Get account summary
- `/private/buy` – Place a buy order
- `/private/sell` – Place a sell order
- `/private/cancel` – Cancel an order
- `/private/get_open_orders_by_currency` – List open orders by currency
- `/private/get_order_state` – Get order state
- `/private/get_positions` – Get open positions
- `/private/get_account_settings` – Get account settings
- `/private/change_account_settings` – Change account settings
- `/private/get_subaccounts` – List subaccounts
- `/private/transfer_to_subaccount` – Transfer to subaccount
- `/private/transfer_to_main` – Transfer to main account
- `/private/get_deposits` – List deposits
- `/private/get_withdrawals` – List withdrawals
- `/private/withdraw` – Withdraw funds
- `/private/get_transaction_log` – Get transaction log

### Implemented Private WebSocket Endpoints (`private/websocket/`)

- `private/buy` – Place a buy order via WebSocket
- `private/sell` – Place a sell order via WebSocket
- `private/cancel` – Cancel an order via WebSocket
- `private/get_account_summary` – Get account summary via WebSocket
- `private/get_positions` – Get open positions via WebSocket
- `private/get_open_orders_by_currency` – List open orders by currency via WebSocket
- `private/get_order_state` – Get order state via WebSocket
- `private/get_account_settings` – Get account settings via WebSocket
- `private/change_account_settings` – Change account settings via WebSocket
- `private/get_subaccounts` – List subaccounts via WebSocket
- `private/transfer_to_subaccount` – Transfer to subaccount via WebSocket
- `private/transfer_to_main` – Transfer to main account via WebSocket
- `private/get_deposits` – List deposits via WebSocket
- `private/get_withdrawals` – List withdrawals via WebSocket
- `private/withdraw` – Withdraw funds via WebSocket
- `private/get_transaction_log` – Get transaction log via WebSocket

See the [Deribit API documentation](https://docs.deribit.com/#private-get_account_summary) for the full list of private endpoints.

**Authentication:**  
Private endpoints require API Key + Secret. See the authentication section above for

---

## 📁 File Structure

- `public/rest/` – REST endpoints (one file per endpoint)
- `public/websocket/` – WebSocket endpoints (one file per endpoint)
- `rate_limit.rs` – Rate limiting configuration and logic
- `enums.rs` – All enums for fixed-value fields
- `error.rs` – Error types and error response mapping

---

## 🚀 Usage Examples

### Get Server Time (REST)

```rust
use venues::deribit::{
    public::rest::{RestClient, GetTimeRequest},
    AccountTier, RateLimiter
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let rate_limiter = RateLimiter::new(AccountTier::Tier4);
    let rest_client = RestClient::new("https://www.deribit.com", client, rate_limiter);

    let response = rest_client.get_time(GetTimeRequest {}).await?;
    tracing::info!("Server time: {} ms", response.result);
    Ok(())
}
```

### Test Connection (REST)

```rust
use venues::deribit::{
    public::rest::{RestClient, TestRequest},
    AccountTier, RateLimiter
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let rate_limiter = RateLimiter::new(AccountTier::Tier4);
    let rest_client = RestClient::new("https://test.deribit.com", client, rate_limiter);

    let response = rest_client.test(TestRequest::new()).await?;
    tracing::info!("API Version: {}", response.result.version);

    // Exception test
    let exception_request = TestRequest::new_exception();
    match rest_client.test(exception_request).await {
        Ok(_) => tracing::warn!("Unexpected success"),
        Err(e) => tracing::info!("Expected error: {}", e),
    }
    Ok(())
}
```

### Get Platform Status (REST)

```rust
use venues::deribit::{
    public::rest::{RestClient, GetStatusRequest},
    AccountTier, RateLimiter
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let rate_limiter = RateLimiter::new(AccountTier::Tier4);
    let rest_client = RestClient::new("https://www.deribit.com", client, rate_limiter);

    let response = rest_client.get_status(GetStatusRequest {}).await?;
    tracing::info!("Platform locked: {}", response.result.locked);
    tracing::info!("Locked indices: {:?}", response.result.locked_indices);
    Ok(())
}
```

### WebSocket Hello Example

```rust
use venues::deribit::{AccountTier, DeribitWebSocketClient, RateLimiter};
use websockets::WebSocketConnection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rate_limiter = RateLimiter::new(AccountTier::Tier4);
    let mut client = DeribitWebSocketClient::new(None, rate_limiter);

    client.connect().await?;
    let response = client.send_hello("my_client".to_string(), "1.0.0".to_string()).await?;
    tracing::info!("API Version: {}", response.result.version);
    client.disconnect().await?;
    Ok(())
}
```

---

## 🧪 Testing

- **Unit tests:** In each endpoint file, do not require credentials or network.
- **Integration tests:** In `tests/` directory at repo root.
- Run all tests:
  ```bash
  cargo test deribit
  ```
- _All 249 tests pass covering serialization, rate limiting, error handling, and integration._

---

## 🛡️ Error Handling

- All errors map to a venue-specific error enum with code/message.
- HTTP status codes and API error codes are mapped to error variants.
- Error messages are preserved from the API.
- See `error.rs` for details.

---

## 🏗️ Design Principles

- **Low latency:** Minimal allocations, async everywhere.
- **Exact rate limiting:** Credit-based, per endpoint.
- **Pure wrappers:** No helpers, only endpoint logic.
- **Common interfaces:** Implements `RestClient` and `WebSocketConnection` traits.
- **Idiomatic Rust:** Strong typing, enums for all fixed-value fields.

---

## 📦 Import Examples

### REST

```rust
use venues::deribit::public::rest::{
    RestClient,
    GetTimeRequest, GetTimeResponse,
    TestRequest, TestResponse, TestResult,
    GetStatusRequest, GetStatusResponse, GetStatusResult,
    GetComboIdsRequest, GetComboIdsResponse,
    GetCombosRequest, GetCombosResponse, ComboInfo, ComboLeg
};
```

### WebSocket

```rust
use venues::deribit::{
    DeribitWebSocketClient,
    HelloRequest,
    HelloResponse,
    JsonRpcRequest,
    RateLimiter,
    AccountTier
};
```

---

## 📝 Notes

- **No credentials required** for public endpoints.
- **Private endpoints** are not implemented in this venue module.
- All code passes clippy and follows project conventions.
