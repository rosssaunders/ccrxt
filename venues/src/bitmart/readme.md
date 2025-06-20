# BitMart Venue API

This venue implements **BitMart's public and private API** (REST and WebSocket) for use in the `venues` crate, following project conventions for structure, error handling, and rate limiting.

---

## 📚 Documentation

- [BitMart API Reference](https://developer-pro.bitmart.com/en/)
- [BitMart WebSocket API](https://developer-pro.bitmart.com/en/#websocket-api)
- [BitMart REST API](https://developer-pro.bitmart.com/en/#rest-api)

---

## 🔐 Authentication

- **Public endpoints:** No authentication required.
- **Private endpoints:** API Key + Secret + Memo (see [BitMart authentication docs](https://developer-pro.bitmart.com/en/#authentication)).

---

## 🗂️ Implemented Endpoints

### REST (public/rest/)

- `/public/currency_list` – List all supported currencies
- `/public/trading_pairs_list` – List all trading pairs
- `/public/trading_pair_details` – Get details for a trading pair
- `/public/ticker_all_pairs` – Get ticker for all pairs
- `/public/ticker` – Get ticker for a specific pair
- `/public/latest_kline` – Get latest kline (candlestick) data
- `/public/history_kline` – Get historical kline (candlestick) data
- `/public/depth` – Get order book depth
- `/public/recent_trades` – Get recent trades

### WebSocket (public/websocket/)

- `/public/ticker` – Real-time ticker updates
- `/public/depth` – Real-time order book updates
- `/public/trades` – Real-time trade updates

---

## 🚫 Private Endpoints

**Private endpoints are implemented in this venue.**

You can find private REST endpoints under `venues/src/bitmart/private/rest/` and private WebSocket endpoints under `venues/src/bitmart/private/websocket/`.

### Implemented Private REST Endpoints (`private/rest/`)

- `/private/account_balance` – Get account balance
- `/private/currencies` – Get supported currencies (private)
- `/private/spot_wallet_balance` – Get spot wallet balance
- `/private/deposit_address` – Get deposit address
- `/private/withdraw_quota` – Get withdraw quota
- `/private/withdraw` – Withdraw funds
- `/private/withdraw_address_list` – Get withdraw address list
- `/private/deposit_withdraw_history` – Get deposit/withdraw history
- `/private/deposit_withdraw_detail` – Get deposit/withdraw detail
- `/private/margin_isolated_account` – Get isolated margin account info
- `/private/margin_asset_transfer` – Margin asset transfer
- `/private/basic_fee_rate` – Get basic fee rate
- `/private/actual_trade_fee_rate` – Get actual trade fee rate
- `/private/submit_order` – Place a new order
- `/private/cancel_order` – Cancel an order
- `/private/query_order` – Query order details
- `/private/query_orders` – Query order list
- `/private/query_trades` – Query account trades
- `/private/query_order_trades` – Query trades for a specific order

### Implemented Private WebSocket Endpoints (`private/websocket/`)

- `/private/orders` – Real-time order updates
- `/private/trades` – Real-time trade updates
- `/private/balance` – Real-time balance updates

See the [BitMart API documentation](https://developer-pro.bitmart.com/en/#private-account) for the full list of private endpoints.

**Authentication:**  
Private endpoints require API Key + Secret + Memo. See the authentication section above for details.

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
use venues::bitmart::{
    public::rest::{RestClient, SystemTimeRequest},
    RateLimiter
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let rate_limiter = RateLimiter::default();
    let rest_client = RestClient::new("https://api.bitmart.com", client, rate_limiter);

    // Example: Get server time
    let response = rest_client.get_system_time(SystemTimeRequest {}).await?;
    println!("{:?}", response);

    Ok(())
}
```

---

## 🧪 Testing

- Unit tests are included in each endpoint file.
- Integration tests are located in the `tests/` directory at the repo root.

---

## 📝 Notes

- All credentials must be provided securely (see project credential handling instructions).
- All enums and error types follow project conventions.
- Rate limiting is implemented per venue requirements.
