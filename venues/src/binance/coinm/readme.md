# Binance COIN-M Futures Venue

This directory implements the Binance COIN-Margined (COIN-M) Futures API for the CCRXT project.

---

## 📚 Source Documentation

- [Binance COIN-M Futures API Docs](https://binance-docs.github.io/apidocs/delivery/en/)
- [Binance COIN-M Futures WebSocket Docs](https://binance-docs.github.io/apidocs/delivery/en/#websocket-market-streams)

---

## 🔐 Authentication Type

- **API Key + Secret**  
  All private endpoints require both an API Key and Secret, which must be provided as `SecretString` types and securely stored.

---

## 🚀 Implemented Endpoints

### Public REST Endpoints

- `exchange_info` — [public/rest/exchange_info.rs](src/binance/coinm/public/rest/exchange_info.rs)

### Private REST Endpoints

- `account` — [private/rest/account.rs](src/binance/coinm/private/rest/account.rs)
- `account_trades` — [private/rest/account_trades.rs](src/binance/coinm/private/rest/account_trades.rs)
- `all_orders` — [private/rest/all_orders.rs](src/binance/coinm/private/rest/all_orders.rs)
- `batch_order` — [private/rest/batch_order.rs](src/binance/coinm/private/rest/batch_order.rs)
- `cancel_order` — [private/rest/cancel_order.rs](src/binance/coinm/private/rest/cancel_order.rs)
- `open_orders` — [private/rest/open_orders.rs](src/binance/coinm/private/rest/open_orders.rs)
- `order` — [private/rest/order.rs](src/binance/coinm/private/rest/order.rs)
- `position_risk` — [private/rest/position_risk.rs](src/binance/coinm/private/rest/position_risk.rs)
- `query_order` — [private/rest/query_order.rs](src/binance/coinm/private/rest/query_order.rs)

### REST Clients

- Public REST client: [public/rest/client.rs](src/binance/coinm/public/rest/client.rs)
- Private REST client: [private/rest/client.rs](src/binance/coinm/private/rest/client.rs)

### Rate Limiting

- Venue-specific rate limiting logic: [rate_limit.rs](src/binance/coinm/rate_limit.rs)
- Rate limiting specification: [specs/rate_limiting.md](src/binance/coinm/specs/rate_limiting.md)

---

## 🗂️ File Structure

- All endpoint files are organized under `public/` or `private/` subdirectories.
- REST and WebSocket clients are in their respective subdirectories.
- Each endpoint is implemented in its own file.
- Common logic (e.g., rate limiting) is factored into shared modules.

---

## 🛠️ Coding & Contribution Notes

- All enums for response fields are defined in [enums.rs](src/binance/coinm/enums.rs) and implement all required traits.
- Error handling is centralized in [errors.rs](src/binance/coinm/errors.rs) and follows project conventions.
- All code passes project clippy rules and uses structured logging.
- Credentials are always handled as `SecretString` and never as plain strings.

---

## 📄 See Also

- [General Coding Standards](../../.github/instructions/general-coding.instructions.md)
- [Error Handling](../../.github/instructions/error-handling.instructions.md)
- [Enum Usage](../../.github/instructions/enums.instructions.md)
- [Rate Limiting](../../.github/instructions/rate-limiting.instructions.md)
