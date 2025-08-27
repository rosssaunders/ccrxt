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

### Venue Structure (Reference: Gate.io)

**`venues/src/gateio/` is the reference implementation that all venues MUST follow.**

Each venue follows this exact structure:
```
venues/src/<exchange>/
├── mod.rs                          # Main module exports and re-exports
├── credentials.rs                  # Venue-specific credentials struct
├── enums.rs                        # All venue enums (OrderType, etc.)
├── errors.rs                       # Error types and handling
├── rate_limit.rs                   # Rate limiting implementation
├── rate_limiter_trait.rs           # Rate limiter trait definition
├── private_client.rs               # Private API client
├── public_client.rs                # Public API client
├── public/                         # Public endpoints
│   ├── mod.rs
│   └── rest/
│       ├── mod.rs
│       └── <product>/              # Product-specific endpoints
│           ├── mod.rs
│           ├── endpoint1.rs
│           └── endpoint2.rs
└── private/                        # Private endpoints
    ├── mod.rs
    └── rest/
        ├── mod.rs
        └── <product>/              # Product-specific endpoints
            ├── mod.rs
            ├── endpoint1.rs
            └── endpoint2.rs
```

### Critical Implementation Patterns

1. **Endpoint Organization**: Each endpoint MUST be in its own file under `public/rest/<product>/` or `private/rest/<product>/`
2. **Product Categorization**: Group endpoints by product/service (spot, delivery, options, unified, wallet, etc.)
3. **Documentation Format**: All endpoint functions MUST use `[docs](URL)` format with rate limits
4. **Pure Wrappers**: Endpoint implementations are pure - no fix-up logic or helpers
5. **Parameter Structs**: All endpoint functions MUST take a single struct parameter (except URL path params)
6. **Rate Limiting**: Implement venue-specific rate limiting at root level (`rate_limit.rs`, `rate_limiter_trait.rs`)
7. **SecretString for Credentials**: All API keys/secrets use `SecretString` type, never plain `String`
8. **Enums for Fixed Values**: All fields with fixed sets of values must use enums, not strings
9. **HTTP Verb Functions**: Use verb-specific functions (`send_get_request`, `send_post_request`) not generic methods

### Architecture & Coding Standards

Detailed instruction files in `.github/instructions/`:
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
- `http-performance.instructions.md` - HTTP performance optimization

## Important Implementation Rules

### Documentation and Formatting (from .github/instructions/):
- **Documentation links**: Use inline format `[docs](URL)` in function doc comments, never reference-style
- **Endpoint functions MUST include**:
  - Title matching exchange docs exactly
  - Link to official API documentation: `[docs](URL)`
  - Rate limit information
  - Arguments with descriptions
  - Return value description
- **All struct fields MUST have doc comments** with purpose, valid values, constraints
- **Blank line required between each struct field**
- **Field names in serde attributes MUST exactly match API documentation**

### Performance Requirements (CRITICAL):
- **HTTP verbs MUST NOT be passed as parameters** - use verb-specific functions
- **MUST use**: `send_get_request()`, `send_post_request()`, `send_put_request()`, etc.
- **MUST NOT use**: generic `send_request()` with method parameter
- **All endpoint functions MUST take single struct for parameters** (except URL path params)

### File Structure Rules:
- **Each endpoint MUST be in its own file** - no combining endpoints
- **Endpoint URL paths MUST be defined as constants** (SCREAMING_SNAKE_CASE)
- **Each mod import MUST be on its own line** to avoid merge conflicts
- **DO NOT implement helper methods** on request/response structs (e.g., `new()`, `with_*()`, constructors, utility methods). Users should construct these structs directly. Only implement the essential RestClient API methods.

### From .cursorrules:
- Code prioritizes performance over cleanliness (while maintaining correctness)
- URL encoding must use `serde_urlencoded`, never manual concatenation
- Error handling must preserve original error codes and messages
- Never use regex for error parsing - use direct string matching

### Linting Rules (from Cargo.toml):
- `unsafe_code = "forbid"` - No unsafe code allowed
- `unwrap_used = "deny"` - No `.unwrap()` - could panic in production
- `panic = "deny"` - No `panic!()` - crashes are unacceptable
- `indexing_slicing = "deny"` - No direct indexing - could panic
- `todo = "deny"` - No TODO in production code
