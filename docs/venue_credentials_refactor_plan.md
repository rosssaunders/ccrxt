# Venue Credentials Refactor Plan

This living document tracks the migration to per-venue `...Credentials` structs and the associated client/test updates.

## Goals

- Each venue exposes a single credentials struct with the `Credentials` suffix (for UI discovery).
- Private RestClient constructors accept only the credentials struct (no raw key/secret args).
- Tests and READMEs updated accordingly.

## Global Tasks

- [x] Update credentials/venue instructions to mandate `...Credentials` structs and placement.
- [ ] Add a small helper to redact SecretString in Debug when needed.

## Venue-by-Venue Plan

### OKX

- Credentials: `OkxCredentials { api_key: SecretString, secret_key: SecretString, passphrase: SecretString }`
- Non-secret config: `OkxPrivateConfig { simulate: bool, broker_id: Option<String> }`
- Files:
  - Add `venues/src/okx/private/rest/credentials.rs` and re-export from `venues/src/okx/private/rest/mod.rs`.
  - Update `venues/src/okx/private/rest/client.rs` RestClient::new to take `OkxCredentials` and `OkxPrivateConfig`.
  - Replace internal uses of raw secrets with `creds` fields; use config for simulated-trading/broker headers.
- Tests:
  - Update unit tests constructing RestClient to pass `OkxCredentials`.
  - Add serde round-trip tests for `OkxCredentials` with optional fields.
- Docs:
  - Update `venues/src/okx/README.md` to document credentials.

### Binance USDM

- Credentials: `BinanceUsdmCredentials { api_key: SecretString, secret_key: SecretString }`
- Files:
  - Add `venues/src/binance/usdm/private/rest/credentials.rs`; re-export via `mod.rs`.
  - Update private RestClient::new signature to take `BinanceUsdmCredentials`.
- Tests:
  - Update unit/integration tests to construct credentials struct.
  - Add serde round-trip test for `BinanceUsdmCredentials`.
- Docs:
  - Update venue README with credentials struct.

### KuCoin (Spot/Futures)

- Credentials:
  - `KucoinSpotCredentials { api_key, secret_key, passphrase, key_version: Option<u8> }`
  - `KucoinFuturesCredentials { api_key, secret_key, passphrase, key_version: Option<u8> }`
- Similar file, test, and doc updates as above.

### Coinbase Exchange

- Credentials: `CoinbaseExchangeCredentials { api_key, secret_key, passphrase }`
- Update private RestClient and tests; doc credentials.

### Bybit

- Credentials: `BybitCredentials { api_key, secret_key }`

### Bitget

- Credentials: `BitgetCredentials { api_key, secret_key, passphrase }`

### Bitmart

- Credentials: `BitmartCredentials { api_key, secret_key, memo }`

### Gate.io

- Credentials: `GateioCredentials { api_key, secret_key, sub_account: Option<SecretString> }`

### Crypto.com

- Credentials: `CryptocomCredentials { api_key, secret_key, subaccount_id: Option<SecretString> }`

### Deribit

- Credentials: `DeribitCredentials { client_id: SecretString, client_secret: SecretString }`

### Bullish

- Credentials: `BullishCredentials { api_key: SecretString, secret_key: SecretString }`

### BingX

- Credentials: `BingxCredentials { api_key: SecretString, secret_key: SecretString }`

## Test Migration Checklist (per venue)

- Replace raw key/secret args with credentials struct in RestClient::new tests.
- Centralize env → credentials mapping in integration tests.
- Add `serde_json` round-trip for the credentials struct (no secrets printed).
- Ensure no tests require real secrets.

## Naming and Discovery

- Suffix must be exactly `Credentials`.
- Struct derives `Serialize`/`Deserialize` for UI generation; optional `Default` when applicable.

## Notes

- Keep base_url, reqwest::Client, and RateLimiter as separate parameters.
- Shared helpers for header/signature building should accept a reference to the credentials struct.
- recv_window (and similar request window/timeout/query tuning) is a per-call parameter, not part of credentials (e.g., Binance/Bybit REST methods accept it where applicable).
