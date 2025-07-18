# Binance Spot Trading Venue

This directory implements the Binance Spot Trading API for the CCRXT project.

---

## 📚 Source Documentation

- [Binance Spot API Docs](https://binance-docs.github.io/apidocs/spot/en/)
- [Binance Spot WebSocket Docs](https://binance-docs.github.io/apidocs/spot/en/#websocket-market-streams)

---

## 🔐 Authentication Type

- **API Key + Secret**  
  All private endpoints require both an API Key and Secret, which must be provided as `SecretString` types and securely stored.

---

## 🚀 Implemented Endpoints

### Public REST Endpoints

- `ping` — [public/rest/ping.rs](public/rest/ping.rs)
- `server_time` — [public/rest/server_time.rs](public/rest/server_time.rs)
- `exchange_info` — [public/rest/exchange_info.rs](public/rest/exchange_info.rs)
- `depth` — [public/rest/depth.rs](public/rest/depth.rs)
- `trades` — [public/rest/trades.rs](public/rest/trades.rs)
- `historical_trades` — [public/rest/historical_trades.rs](public/rest/historical_trades.rs)
- `agg_trades` — [public/rest/agg_trades.rs](public/rest/agg_trades.rs)
- `klines` — [public/rest/klines.rs](public/rest/klines.rs)
- `ui_klines` — [public/rest/ui_klines.rs](public/rest/ui_klines.rs)
- `avg_price` — [public/rest/avg_price.rs](public/rest/avg_price.rs)
- `ticker` — [public/rest/ticker.rs](public/rest/ticker.rs)
- `ticker_24hr` — [public/rest/ticker_24hr.rs](public/rest/ticker_24hr.rs)
- `ticker_price` — [public/rest/ticker_price.rs](public/rest/ticker_price.rs)
- `ticker_book` — [public/rest/ticker_book.rs](public/rest/ticker_book.rs)
- `ticker_trading_day` — [public/rest/ticker_trading_day.rs](public/rest/ticker_trading_day.rs)

### Private REST Endpoints

- `account` — [private/rest/account.rs](private/rest/account.rs)
- `account_commission` — [private/rest/account_commission.rs](private/rest/account_commission.rs)
- `all_orders` — [private/rest/all_orders.rs](private/rest/all_orders.rs)
- `all_orderlist` — [private/rest/all_orderlist.rs](private/rest/all_orderlist.rs)
- `amend_order` — [private/rest/amend_order.rs](private/rest/amend_order.rs)
- `cancel_all_orders` — [private/rest/cancel_all_orders.rs](private/rest/cancel_all_orders.rs)
- `cancel_order` — [private/rest/cancel_order.rs](private/rest/cancel_order.rs)
- `cancel_orderlist` — [private/rest/cancel_orderlist.rs](private/rest/cancel_orderlist.rs)
- `cancel_replace` — [private/rest/cancel_replace.rs](private/rest/cancel_replace.rs)
- `my_allocations` — [private/rest/my_allocations.rs](private/rest/my_allocations.rs)
- `my_prevented_matches` — [private/rest/my_prevented_matches.rs](private/rest/my_prevented_matches.rs)
- `my_trades` — [private/rest/my_trades.rs](private/rest/my_trades.rs)
- `oco_order` — [private/rest/oco_order.rs](private/rest/oco_order.rs)
- `oco_orderlist` — [private/rest/oco_orderlist.rs](private/rest/oco_orderlist.rs)
- `open_orders` — [private/rest/open_orders.rs](private/rest/open_orders.rs)
- `open_orderlist` — [private/rest/open_orderlist.rs](private/rest/open_orderlist.rs)
- `order` — [private/rest/order.rs](private/rest/order.rs)
- `order_amendments` — [private/rest/order_amendments.rs](private/rest/order_amendments.rs)
- `oto_order` — [private/rest/oto_order.rs](private/rest/oto_order.rs)
- `otoco_order` — [private/rest/otoco_order.rs](private/rest/otoco_order.rs)
- `query_order` — [private/rest/query_order.rs](private/rest/query_order.rs)
- `query_orderlist` — [private/rest/query_orderlist.rs](private/rest/query_orderlist.rs)
- `rate_limit_order` — [private/rest/rate_limit_order.rs](private/rest/rate_limit_order.rs)
- `sor_order` — [private/rest/sor_order.rs](private/rest/sor_order.rs)
- `test_order` — [private/rest/test_order.rs](private/rest/test_order.rs)

### REST Clients

- Public REST client: [public/rest/client.rs](public/rest/client.rs)
- Private REST client: [private/rest/client.rs](private/rest/client.rs)

### Rate Limiting

- Venue-specific rate limiting logic: [rate_limit.rs](rate_limit.rs)

---

## 🗂️ File Structure

- All endpoint files are organized under `public/` or `private/` subdirectories.
- REST and WebSocket clients are in their respective subdirectories.
- Each endpoint is implemented in its own file.
- Common logic (e.g., rate limiting) is factored into shared modules.

---

## 🛠️ Coding & Contribution Notes

- All enums for response fields are defined in [enums.rs](enums.rs) and implement all required traits.
- Error handling is centralized in [errors.rs](errors.rs) and follows project conventions.
- All code passes project clippy rules and uses structured logging.
- Credentials are always handled as `SecretString` and never as plain strings.

---

## 📄 See Also

- [General Coding Standards](../../../.github/instructions/general-coding.instructions.md)
- [Error Handling](../../../.github/instructions/error-handling.instructions.md)
- [Enum Usage](../../../.github/instructions/enums.instructions.md)
- [Rate Limiting](../../../.github/instructions/rate-limiting.instructions.md)