# OKX Venue

## 📚 Source Documentation

- [OKX API Documentation](https://www.okx.com/docs-v5/en/)

---

## 🔐 Authentication

- **Type:** API Key + Secret + Passphrase
- All credentials are securely handled using [`SecretString`](https://docs.rs/secrecy/latest/secrecy/struct.SecretString.html).
- Credentials must be passed as `impl Into<SecretString>` to all constructors and methods that require them.

---

## 📦 Directory Structure

- `public/` — Public endpoints (no authentication required)
  - `rest/` — REST API endpoints and client (`RestClient`)
  - `websocket/` — WebSocket endpoints and client (`WsClient`)
- `private/` — Private endpoints (authentication required)
  - `rest/` — REST API endpoints and client (`RestClient`)
  - `websocket/` — WebSocket endpoints and client (`WsClient`)
- `rate_limit.rs` — Venue-specific rate limiting configuration
- `enums.rs` — All enums for request/response types
- `errors.rs` — Error types and error response mapping

---

## ✅ Implemented Endpoints

### Private REST Endpoints

- `activate_option`
- `adjust_position_margin_balance`
- `amend_order`
- `bills_archive`
- `bills_details`
- `bills_details_archive`
- `bills_history`
- `bills_history_archive`
- `cancel_all_after`
- `cancel_batch_orders`
- `cancel_order`
- `close_position`
- `get_account_balance`
- `get_account_config`
- `get_account_instruments`
- `get_account_position_risk`
- `get_account_positions`
- `get_account_positions_history`
- `get_account_positions_risk`
- `get_account_switch_precheck`
- `get_asset_valuation`
- `get_fee_rates`
- `get_max_avail_size`
- `get_max_buy_sell_amount`
- `get_max_withdrawal`
- `get_order_details`
- `get_order_history`
- `get_order_history_archive`
- `get_order_list`
- `get_pending_orders`
- `get_position_risk`
- `get_positions`
- `get_positions_history`
- `get_positions_risk`
- `get_quick_margin_borrow_repay_history`
- `get_trade_fee`
- `get_transaction_details`
- `get_transaction_history`
- `place_batch_orders`
- `place_order`
- `quick_margin_borrow_repay`
- `set_account_leverage`
- `set_account_mode`
- `set_auto_deposit_margin`
- `set_position_mode`
- `set_risk_limit`
- `transfer_funds`
- `withdraw_funds`

### Public REST Endpoints

- `get_currencies`
- `get_exchange_rate`
- `get_funding_rate`
- `get_funding_rate_history`
- `get_index_components`
- `get_index_tickers`
- `get_instruments`
- `get_mark_price`
- `get_order_book`
- `get_system_time`
- `get_ticker`
- `get_tickers`
- `get_trades`
- `get_underlying`
- `get_vip_interest_rate_loan_quota`

---

## 🛡️ Error Handling

- All errors are defined in [`errors.rs`](src/okx/errors.rs) and follow project conventions.
- Each error includes code, message, and relevant fields.
- Error enums and response structs are mapped according to the OKX API.
- Implements `From<ErrorResponse>` for the error enum, mapping all known codes.
- Each HTTP status code maps to a specific error code.
- All functions return the venue's `Result` type and use the `?` operator.

---

## 🚦 Rate Limiting

- Venue-specific rate limiting is configured in [`rate_limit.rs`](src/okx/rate_limit.rs).
- Uses an existing rate limiter crate, separated from endpoint logic.

---

## 🧪 Testing

- **Unit tests** are included in each endpoint file, as per project policy.
- **Integration tests** are located in the repo root `tests/` directory.

---

## 📝 Examples

- Example code is provided in [`venues/examples/okx/`](../../examples/okx/).
- Each example file is self-contained and runnable, with clear instructions if credentials or setup are required.

---

## 🛠️ Coding Standards

- All code is idiomatic, high-performance Rust and passes project clippy rules.
- Logging uses the `log` or `tracing` facade.
- No sensitive information is ever logged or printed.
- All enums are defined in `enums.rs` and used in response/request structs.
- Each endpoint is implemented in its own file under the appropriate `public/rest/` or `private/rest/` directory.
- The REST client struct is always named `RestClient`.

---

## 📣 Contributing

Please follow the [project coding standards](../../.github/instructions/) and review the [venue integration instructions](../../.github/instructions/venue.instructions.md) before