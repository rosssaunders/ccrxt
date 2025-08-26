---
applyTo: "venues/**"
---

# Venue Structure and Organization

**REFERENCE IMPLEMENTATION**: `venues/src/gateio/` is the reference implementation that all venues MUST follow.

## File Structure and Naming

### Root Venue Directory Structure

Each venue MUST follow this exact directory structure:

```
venues/src/<venue>/
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

### Core Module Requirements

- **`credentials.rs`** - MUST exist at venue root level defining venue's `Credentials` struct
- **`enums.rs`** - MUST contain all venue-specific enums (OrderType, OrderSide, etc.)
- **`errors.rs`** - MUST define venue-specific error types and `RestResult` type alias
- **`rate_limit.rs`** - MUST contain venue rate limiting implementation
- **`rate_limiter_trait.rs`** - MUST define venue rate limiter trait
- **`private_client.rs`** - MUST contain `PrivateRestClient` implementation
- **`public_client.rs`** - MUST contain `PublicRestClient` implementation

### Endpoint Organization

- **ALL endpoint files MUST be under `public/rest/` or `private/rest/` subdirectories**
- **Endpoints MUST be organized by product/category** (e.g., `spot/`, `delivery/`, `options/`, `unified/`, `wallet/`)
- **Each endpoint MUST be in its own file** - never combine multiple endpoints in one file
- **File naming**: Use descriptive names matching the API endpoint (e.g., `spot_accounts.rs`, `create_order.rs`)
- **WebSocket endpoints**: Use `ws_<endpoint>.rs` naming convention

### Directory Structure Rules

- **Public endpoints**: `public/rest/<product>/<endpoint>.rs`
- **Private endpoints**: `private/rest/<product>/<endpoint>.rs`  
- **Each product directory MUST have its own `mod.rs`** with proper exports
- **No subdirectories for individual endpoints** - all endpoint files directly under product directory
- **Common logic**: Place shared code at venue root level, not in subdirectories

### Import Path Requirements

- **For imports of modules in the same crate**: Use `crate::…` absolute paths instead of `super::…` or relative paths
- **Do NOT apply this rule to external crates** - continue using their crate name (e.g., `rest::…`)
- **Shared logic MUST be factored into reusable modules** - no duplication across public/private modules

# Client Constructor Requirements

- Private REST `RestClient::new` MUST accept a single credentials struct (the venue-specific `...Credentials`) rather than separate API key/secret/passphrase parameters.
- Base URL, HTTP client, and rate limiter remain separate constructor parameters.

# Venue README Requirements

Each venue MUST include a README file that provides:

1. Links to the source documentation
2. The authentication type (e.g., API Key + Secret)
3. A list of endpoints implemented
4. The credentials struct fields, their meaning, and recommended secure provisioning (env vars; never hard-code secrets).
