# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

CCRXT is a Rust library providing low-latency wrappers around cryptocurrency exchange APIs. The project prioritizes performance for high-frequency trading applications while maintaining strict adherence to exchange-specific rate limits and API behaviors.

## Build and Run Commands

- **Build the entire project**: `cargo build`
- **Run tests**: `cargo test`
- **Build with release optimizations**: `cargo build --release`
- **Run a specific example**:
  - Coinbase market data: `cargo run --example coinbase_market_data -- --products "BTC-USD,ETH-USD" --orderbook --ticker`
  - Authenticated WebSocket: `cargo run --example authenticated_websocket`
- **Format code**: `cargo fmt`
- **Run linter**: `cargo clippy`
- **Generate documentation**: `cargo doc --no-deps --open`

## Authentication Setup

For examples requiring API authentication:

1. Create a `.env` file in the venues directory
2. Add the required tokens/keys (e.g., `COINBASE_JWT_TOKEN=your_jwt_token_here`)
3. Use the `dotenv` crate to load environment variables in your code

## Code Architecture

### Workspace Structure

The project is organized into three main crates:
- `venues`: Implementation of specific exchange APIs
- `websockets`: Common WebSocket traits and utilities
- `rest`: Common REST client traits and utilities

### Architecture & Coding Standards

Please refer to the detailed instruction files in `.github/instructions/`:
- `general-coding.instructions.md` - General coding standards and practices
- `venue.instructions.md` - Venue implementation guidelines
- `websocket.instructions.md` - WebSocket implementation patterns
- `rest.instructions.md` - REST API implementation guidelines
- `error-handling.instructions.md` - Error handling patterns
- `rate-limiting.instructions.md` - Rate limiting implementation
- `enums.instructions.md` - Enum usage and patterns
- `examples.instructions.md` - Example code guidelines
- `testing.instructions.md` - Testing practices
- `integration-tests.instructions.md` - Integration test standards
- `credentials.instructions.md` - Credential management guidelines
