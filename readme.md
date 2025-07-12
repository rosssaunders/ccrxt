# CCRXT

[![CI](https://github.com/rosssaunders/ccrxt/actions/workflows/ci.yml/badge.svg)](https://github.com/rosssaunders/ccrxt/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://rosssaunders.github.io/ccrxt/badges/coverage.json)](https://github.com/rosssaunders/ccrxt/actions/workflows/coverage.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/ccrxt.svg)](https://crates.io/crates/ccrxt)
[![Documentation](https://docs.rs/ccrxt/badge.svg)](https://docs.rs/ccrxt)

Rust wrappers around cryptocurrency exchange APIs with a focus on low-latency, high-frequency trading applications.

> **Note:** This library is currently in development. The Crates.io and Documentation badges will become functional when the library is published.

## Supported Exchanges

| Exchange       | Type                   | Status         | Features                                                                                    |
| -------------- | ---------------------- | -------------- | ------------------------------------------------------------------------------------------- |
| **Binance**    | COIN-M Futures         | ✅ Complete    | Public & Private REST, Rate Limiting, WebSocket Support                                     |
| **Binance**    | USD-M Futures (USDT-M) | ✅ Complete    | Public & Private REST, Rate Limiting, WebSocket Support                                     |
| **Binance**    | Portfolio Margin       | ✅ Complete    | Public & Private REST, Rate Limiting, Error Handling, Request Signing                       |
| **Binance**    | Options (EAPI)         | 🚧 In Progress | Rate Limiting Framework                                                                     |
| **Crypto.com** | Spot Trading           | ✅ Complete    | Public & Private REST, Rate Limiting, Advanced Orders                                       |
| **OKX**        | Spot & Derivatives     | ✅ Complete    | Public & Private REST, Rate Limiting, Integration Tests                                     |
| **Deribit**    | Public API             | ✅ Complete    | Public REST & WebSocket, Rate Limiting, JSON-RPC 2.0, Full Test Coverage                    |
| **Bitmart**    | Spot & Derivatives     | 🚧 In Progress | Error Handling, REST Integration                                                            |
| **Coinbase**   | Exchange               | ✅ Complete    | Private REST, Rate Limiting, Cursor Pagination, Secure Auth, Error Mapping, Sandbox Support |
| **BingX**      | Spot & Derivatives     | 🚧 In Progress | REST Integration                                                                            |
| **Bitget**     | Spot & Derivatives     | 🚧 In Progress | REST Integration                                                                            |
| **Bullish**    | Spot                   | 🚧 In Progress | REST Integration                                                                            |
| **Bybit**      | Spot & Derivatives     | 🚧 In Progress | REST Integration                                                                            |

### Exchange Features

- **✅ Complete**: Full implementation with comprehensive API coverage
- **🚧 In Progress**: Basic infrastructure in place, endpoints being added
- **Public REST**: Market data, instrument info, order books
- **Private REST**: Account management, trading, order management
- **Rate Limiting**: Exchange-specific rate limit enforcement
- **WebSocket Support**: Real-time data feeds (where available)
- **Error Handling**: Consistent and robust error mapping per venue
- **Test Coverage**: Unit and integration tests for all stable venues

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
