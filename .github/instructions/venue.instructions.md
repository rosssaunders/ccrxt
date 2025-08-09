---
applyTo: "venues/**"
---

# File Structure and Naming

- All endpoint files MUST be under either a `public/` or `private/` subdirectory within the venue directory (e.g., venues/src/binance/coinm/public/ or venues/src/binance/coinm/private/).
- Subdirectories for endpoints are REQUIRED: use `public/` for public endpoints and `private/` for private endpoints.
- REST clients and WebSocket clients MUST be placed in their own subdirectories within `public/` and `private/` (e.g., `public/rest/`, `public/websocket/`, `private/rest/`, `private/websocket/`).
- The REST client struct MUST be named `RestClient` (not `PrivateRest` or any other name) and the WebSocket client struct MUST be named `WsClient`.
- If the venue has private endpoints, a `credentials.rs` file MUST exist under `private/rest/` defining the venue's `...Credentials` struct. It MUST be re-exported from `private/rest/mod.rs`.
- File naming convention within each subdirectory:
  - REST endpoints: <endpoint>.rs (e.g., order.rs, account.rs)
  - WebSocket endpoints: ws\_<endpoint>.rs (e.g., ws_trades.rs, ws_depth.rs)
- Each endpoint in its own file.
- Common code (websockets, rate limiting) can be in subdirectories at the venue level.
- For imports of modules in the same crate, use `crate::…` absolute paths instead of `super::…` or relative paths. Do **not** apply this rule to external crates—continue using their crate name (e.g., `rest::…`).
- Shared logic (e.g., client request helpers, rate limiter helpers) MUST be factored into private modules or helpers and reused, not duplicated across public/private or similar modules.

# Client Constructor Requirements

- Private REST `RestClient::new` MUST accept a single credentials struct (the venue-specific `...Credentials`) rather than separate API key/secret/passphrase parameters.
- Base URL, HTTP client, and rate limiter remain separate constructor parameters.

# Venue README Requirements

Each venue MUST include a README file that provides:

1. Links to the source documentation
2. The authentication type (e.g., API Key + Secret)
3. A list of endpoints implemented
4. The credentials struct fields, their meaning, and recommended secure provisioning (env vars; never hard-code secrets).
