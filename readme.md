# CCRXT

[![CI](https://github.com/rosssaunders/ccrxt/actions/workflows/ci.yml/badge.svg)](https://github.com/rosssaunders/ccrxt/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/ccrxt.svg)](https://crates.io/crates/ccrxt)
[![Documentation](https://docs.rs/ccrxt/badge.svg)](https://docs.rs/ccrxt)

Rust wrappers around cryptocurrency exchange APIs with a focus on low-latency, high-frequency trading applications.

> **Note:** This library is currently in development. The Crates.io and Documentation badges will become functional when the library is published.

## Supported Exchanges

| Exchange | Type | Status | Features |
|----------|------|--------|----------|
| **Binance** | COIN-M Futures | ✅ Complete | Public & Private REST, Rate Limiting, WebSocket Support |
| **Binance** | USD-M Futures (USDT-M) | ✅ Complete | Public & Private REST, Rate Limiting, WebSocket Support |
| **Binance** | Portfolio Margin | ✅ Complete | Public & Private REST, Rate Limiting |
| **Binance** | Options (EAPI) | 🚧 In Progress | Rate Limiting Framework |
| **Crypto.com** | Spot Trading | ✅ Complete | Public & Private REST, Rate Limiting, Advanced Orders |
| **OKX** | Spot & Derivatives | ✅ Complete | Public & Private REST, Rate Limiting |

### Exchange Features

- **✅ Complete**: Full implementation with comprehensive API coverage
- **🚧 In Progress**: Basic infrastructure in place, endpoints being added
- **Public REST**: Market data, instrument info, order books
- **Private REST**: Account management, trading, order management
- **Rate Limiting**: Exchange-specific rate limit enforcement
- **WebSocket Support**: Real-time data feeds (where available)

## Design Principles

1. All venues to implement the low latency APIs. If Websocket available, use that over REST calls.
2. All venue rate limiting to be implemented exactly.
3. All wrappers around the endpoints should be pure. Not fixes and helper functions.
4. All websockets to implement the common websocket trait.
