---
applyTo: "venues/src/**"
---

# File Structure and Naming

- All endpoint files MUST be under either a `public/` or `private/` subdirectory within the venue directory (e.g., venues/src/binance/coinm/public/ or venues/src/binance/coinm/private/).
- Subdirectories for endpoints are REQUIRED: use `public/` for public endpoints and `private/` for private endpoints.
- REST clients and WebSocket clients MUST be placed in their own subdirectories within `public/` and `private/` (e.g., `public/rest/`, `public/websocket/`, `private/rest/`, `private/websocket/`).
- The REST client struct MUST be named `RestClient` (not `PrivateRest` or any other name) and the WebSocket client struct MUST be named `WsClient`.
- File naming convention within each subdirectory:
  - REST endpoints: <endpoint>.rs (e.g., order.rs, account.rs)
  - WebSocket endpoints: ws_<endpoint>.rs (e.g., ws_trades.rs, ws_depth.rs)
- Each endpoint in its own file.
- Common code (websockets, rate limiting) can be in subdirectories at the venue level.
- For imports of modules in the same crate, use `crate::…` absolute paths instead of `super::…` or relative paths. Do **not** apply this rule to external crates—continue using their crate name (e.g., `rest::…`).
- Shared logic (e.g., client request helpers, rate limiter helpers) MUST be factored into private modules or helpers and reused, not duplicated across public/private or similar modules.
