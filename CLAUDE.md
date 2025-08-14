# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

CCRXT is a Rust library providing low-latency wrappers around cryptocurrency exchange APIs. The project prioritizes performance for high-frequency trading applications while maintaining strict adherence to exchange-specific rate limits and API behaviors.

Key principles:
- All venues implement low-latency APIs (WebSocket preferred over REST when available)
- Rate limiting is implemented exactly as specified by each exchange
- Wrappers are pure - no helper or fix-up logic
- All WebSocket clients implement the common WebSocket trait
- File and module structure follows strict conventions

## Build and Run Commands

### Core Build Commands
- **Build the entire project**: `cargo build`
- **Build with release optimizations**: `cargo build --release` 
- **Format code**: `cargo fmt`
- **Run linter**: `cargo clippy`
- **Check without building**: `cargo check`
- **Generate documentation**: `cargo doc --no-deps --open`

### Testing Commands
- **Run all tests**: `cargo test`
- **Run a specific test**: `cargo test test_name`
- **Run tests for a specific venue**: `cargo test binance::`
- **Run integration tests only**: `cargo test --test '*'`
- **Run unit tests only**: `cargo test --lib`
- **Run tests with output**: `cargo test -- --nocapture`
- **Run private integration tests**: `RUN_PRIVATE_TESTS=true cargo test`

### Example Commands
- **List all examples**: `cargo run --example` (will show available examples)
- **Run a specific example**:
  - Coinbase market data: `cargo run --example coinbase_market_data -- --products "BTC-USD,ETH-USD" --orderbook --ticker`
  - Authenticated WebSocket: `cargo run --example authenticated_websocket`

## Authentication Setup

For examples and tests requiring API authentication:

1. Create a `.env` file in the venues directory
2. Add the required tokens/keys for each exchange:
   ```
   COINBASE_JWT_TOKEN=your_jwt_token_here
   BINANCE_API_KEY=your_api_key
   BINANCE_API_SECRET=your_api_secret
   # Add other exchange credentials as needed
   ```
3. Use the `dotenv` crate to load environment variables in your code
4. For integration tests requiring authentication, set `RUN_PRIVATE_TESTS=true`

## Code Architecture

### Workspace Structure

The project is a Rust workspace with three main crates:
- `venues/`: Implementation of specific exchange APIs
  - Each exchange has its own module (e.g., `binance/`, `coinbase/`)
  - Submodules for different products (e.g., `spot/`, `coinm/`, `usdm/`)
- `websockets/`: Common WebSocket traits and utilities
- `rest/`: Common REST client traits and utilities

### File Organization

Each venue follows a strict file structure:
```
venues/src/<exchange>/<product>/
├── mod.rs                    # Module exports and re-exports
├── enums.rs                  # All enums for the venue
├── errors.rs                 # Error types and handling
├── client.rs                 # Main client struct(s)
├── public_*.rs              # Public REST endpoints
├── private_*.rs             # Private REST endpoints  
├── ws_public_*.rs           # Public WebSocket endpoints
├── ws_private_*.rs          # Private WebSocket endpoints
└── common/                  # Common code (rate limiting, auth, etc.)
```

### Critical Patterns

1. **No Subdirectories for Endpoints**: All endpoint files must be directly under the venue directory
2. **Pure Wrappers**: Endpoint implementations are pure - no fix-up logic or helpers
3. **Top-Level Exports**: Integration tests must use only top-level exports from venue modules
4. **SecretString for Credentials**: All API keys/secrets use `SecretString` type, never plain `String`
5. **Enums for Fixed Values**: All fields with fixed sets of values must use enums, not strings

### Architecture & Coding Standards

Detailed instruction files in `.github/instructions/`:
- `general-coding.instructions.md` - General coding standards and practices
- `venue.instructions.md` - Venue implementation guidelines
- `websocket.instructions.md` - WebSocket implementation patterns
- `websocket-implementation.instructions.md` - WebSocket implementation specification for native/WASM
- `rest.instructions.md` - REST API implementation guidelines
- `error-handling.instructions.md` - Error handling patterns
- `rate-limiting.instructions.md` - Rate limiting implementation
- `enums.instructions.md` - Enum usage and patterns
- `examples.instructions.md` - Example code guidelines
- `testing.instructions.md` - Testing practices
- `integration-tests.instructions.md` - Integration test standards
- `credentials.instructions.md` - Credential management guidelines
- `http-performance.instructions.md` - HTTP performance optimization

## Important Implementation Rules

### From .cursorrules:
- Code prioritizes performance over cleanliness (while maintaining correctness)
- URL encoding must use `serde_urlencoded`, never manual concatenation
- All struct fields must have doc comments with clear descriptions
- Field names in serde attributes must exactly match API documentation
- All endpoint functions must include a link to official API documentation
- Error handling must preserve original error codes and messages
- Never use regex for error parsing - use direct string matching

### Linting Rules (from Cargo.toml):
- `unsafe_code = "forbid"` - No unsafe code allowed
- `unwrap_used = "deny"` - No `.unwrap()` - could panic in production
- `panic = "deny"` - No `panic!()` - crashes are unacceptable
- `indexing_slicing = "deny"` - No direct indexing - could panic
- `todo = "deny"` - No TODO in production code
