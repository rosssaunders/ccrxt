# CCRXT

[![CI](https://github.com/rosssaunders/ccrxt/actions/workflows/ci.yml/badge.svg)](https://github.com/rosssaunders/ccrxt/actions/workflows/ci.yml)
[![Source Coverage](https://img.shields.io/endpoint?url=https://rosssaunders.github.io/ccrxt/badges/source-coverage.json)](https://github.com/rosssaunders/ccrxt/actions/workflows/coverage.yml)
[![Integration Coverage](https://img.shields.io/endpoint?url=https://rosssaunders.github.io/ccrxt/badges/integration-coverage.json)](https://github.com/rosssaunders/ccrxt/actions/workflows/coverage.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/ccrxt.svg)](https://crates.io/crates/ccrxt)
[![Documentation](https://docs.rs/ccrxt/badge.svg)](https://docs.rs/ccrxt)

Rust wrappers around cryptocurrency exchange APIs with a focus on low-latency, high-frequency trading applications.

> **Note:** This library is currently in development. The Crates.io and Documentation badges will become functional when the library is published.

## Supported Exchanges

| Exchange       | Type                   | Status         | Public Tests | Private Tests | REST | WebSocket |
| -------------- | ---------------------- | -------------- | ------------ | ------------- | ---- | --------- |
| **Binance**    | COIN-M Futures         | ✅ Complete    | ✅           | ❌            | ✅   | ❌        |
| **Binance**    | USD-M Futures (USDT-M) | ✅ Complete    | ✅           | ❌            | ✅   | ❌        |
| **Binance**    | Portfolio Margin       | ✅ Complete    | ❌           | ❌            | ✅   | ❌        |
| **Binance**    | Options (EAPI)         | 🚧 In Progress | ❌           | ❌            | 🚧   | ❌        |
| **Binance**    | Spot                   | ✅ Complete    | ✅           | ❌            | ✅   | ❌        |
| **BingX**      | Spot                   | 🚧 In Progress | ✅           | ❌            | 🚧   | ❌        |
| **Bitget**     | Spot                   | 🚧 In Progress | ✅           | ❌            | 🚧   | ❌        |
| **Bitmart**    | Contract               | 🚧 In Progress | ✅           | ❌            | 🚧   | ❌        |
| **Bitmart**    | Spot                   | 🚧 In Progress | ✅           | ❌            | 🚧   | ❌        |
| **Bullish**    | Spot                   | 🚧 In Progress | ✅           | ❌            | 🚧   | ❌        |
| **Bybit**      | Spot & Derivatives     | 🚧 In Progress | ✅           | ❌            | 🚧   | ❌        |
| **Coinbase**   | Exchange               | ✅ Complete    | ✅           | ❌            | ✅   | ❌        |
| **Crypto.com** | Spot Trading           | ✅ Complete    | ✅           | ❌            | ✅   | ❌        |
| **Deribit**    | Public API             | ✅ Complete    | ✅           | ❌            | ✅   | ❌        |
| **Gate.io**    | Delivery               | 🚧 In Progress | ✅           | ❌            | 🚧   | ❌        |
| **Gate.io**    | Options                | 🚧 In Progress | ✅           | ❌            | 🚧   | ❌        |
| **Gate.io**    | Perpetual              | 🚧 In Progress | ✅           | ❌            | 🚧   | ❌        |
| **Gate.io**    | Spot                   | 🚧 In Progress | ✅           | ❌            | 🚧   | ❌        |
| **Gate.io**    | Unified                | 🚧 In Progress | ✅           | ❌            | 🚧   | ❌        |
| **KuCoin**     | Futures                | 🚧 In Progress | ✅           | ❌            | 🚧   | ❌        |
| **KuCoin**     | Spot                   | 🚧 In Progress | ✅           | ❌            | 🚧   | ❌        |
| **OKX**        | Spot & Derivatives     | ✅ Complete    | ✅           | ❌            | ✅   | ❌        |

### Exchange Features

- **✅ Complete**: Full implementation with comprehensive API coverage
- **🚧 In Progress**: Basic infrastructure in place, endpoints being added

### Implementation Status

- **REST**: Public and private REST API endpoints with rate limiting and error handling
- **WebSocket**: Real-time data feeds and streaming APIs
- **✅**: Fully implemented and tested
- **🚧**: In development
- **❌**: Not yet implemented

### Testing Status

- **Public Tests**: Integration tests for public API endpoints (market data, instrument info, etc.)
- **Private Tests**: Integration tests for private API endpoints (account data, trading operations)
- **✅**: Tests implemented and passing
- **❌**: Tests not yet implemented

## Venue Documentation

Each venue includes a README with:

- Links to the official API documentation
- Authentication requirements
- List of implemented endpoints

See the [venues](venues/) directory for details.

## Design Principles

1. All venues implement low latency APIs. If WebSocket is available, it is preferred over REST.
2. Venue rate limiting is implemented exactly as specified by the exchange.
3. Wrappers around endpoints are pure; no helper or fix-up logic is included.
4. All WebSocket clients implement the common WebSocket trait.
5. File and module structure follows strict conventions for maintainability and clarity.

## Testing

- Unit tests are colocated with the code they test and do not require credentials or network access.
- Integration tests are in the [tests/](tests/) directory and cover real API interactions (where supported).

## Examples

- Example code for each venue is in [venues/examples/](venues/examples/).
- Examples are self-contained and demonstrate usage of public APIs, with clear instructions for credentials if
