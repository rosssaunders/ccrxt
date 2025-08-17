# Credentials Struct Migration Plan

## Overview
This document outlines the comprehensive plan to migrate all venues from using individual `api_key` and `api_secret` parameters to dedicated `Credentials` structs for better security and consistency.

## Current State

### ✅ Already Migrated
- **Bullish** (`venues/src/bullish/private/rest/credentials.rs`)
- **KuCoin Spot** (`venues/src/kucoin/spot/private/rest/credentials.rs`)

### ❌ Venues Requiring Migration
Total: 19 private REST clients across 12 venues

| Venue | Products | Paths |
|-------|----------|--------|
| Binance | 4 | `spot/`, `usdm/`, `coinm/`, `options/` |
| GateIO | 5 | `spot/`, `perpetual/`, `delivery/`, `options/`, `unified/` |
| BitMart | 2 | `spot/`, `contract/` |
| OKX | 1 | `private/rest/` |
| Deribit | 1 | `private/rest/` |
| Coinbase Exchange | 1 | `private/rest/` |
| Crypto.com | 1 | `private/rest/` |
| Bybit | 1 | `private/rest/` |
| BingX | 1 | `spot/private/rest/` |
| Bitget | 1 | `spot/private/rest/` |
| KuCoin Futures | 1 | `futures/private/rest/` |

## Migration Strategy

### Phase 1: Simple Venues (Single Private REST Client)
Start with venues that have only one private REST client to establish the pattern:

1. **OKX** (`okx/private/rest/`)
2. **Deribit** (`deribit/private/rest/`)
3. **Coinbase Exchange** (`coinbaseexchange/private/rest/`)
4. **Crypto.com** (`cryptocom/private/rest/`)
5. **Bybit** (`bybit/private/rest/`)

### Phase 2: Multi-Product Venues
Handle venues with multiple products that share credentials:

1. **Binance** (4 products)
   - Create shared `binance/shared/credentials.rs`
   - Update all 4 products to use shared credentials
   - Products: spot, usdm, coinm, options

2. **GateIO** (5 products)
   - Create shared `gateio/shared/credentials.rs`
   - Update all 5 products to use shared credentials
   - Products: spot, perpetual, delivery, options, unified

3. **BitMart** (2 products)
   - Evaluate if shared or separate credentials are needed
   - Products: spot, contract

### Phase 3: Remaining Venues
1. **BingX** (`bingx/spot/private/rest/`)
2. **Bitget** (`bitget/spot/private/rest/`)
3. **KuCoin Futures** (`kucoin/futures/private/rest/`)
   - Align with existing KuCoin Spot pattern

## Implementation Pattern

### Step 1: Create `credentials.rs`

```rust
//! [Venue Name] API credentials

use rest::secrets::SecretString;

/// Credentials for authenticating with [Venue] private REST API.
///
/// All fields are securely stored using SecretString.
#[derive(Debug, Clone)]
pub struct Credentials {
    /// API key (required)
    pub api_key: SecretString,

    /// API secret (required)
    pub api_secret: SecretString,

    // Add venue-specific fields as needed
    // Examples:
    // /// API passphrase (required for KuCoin)
    // pub api_passphrase: SecretString,
}
```

### Step 2: Add Serialization (Optional but Recommended)

For venues that need serialization, implement safe Serialize/Deserialize that masks secrets:

```rust
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::SerializeStruct;
use serde::de::{MapAccess, Visitor};

impl Serialize for Credentials {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("Credentials", 2)?;
        st.serialize_field("apiKey", &"***")?;
        st.serialize_field("apiSecret", &"***")?;
        st.end()
    }
}

impl<'de> Deserialize<'de> for Credentials {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Implementation similar to Bullish pattern
        // See venues/src/bullish/private/rest/credentials.rs for full example
    }
}
```

### Step 3: Update `client.rs`

#### Before:
```rust
pub struct RestClient {
    api_key: Box<dyn ExposableSecret>,
    api_secret: Box<dyn ExposableSecret>,
    // ... other fields
}

impl RestClient {
    pub fn new(
        api_key: Box<dyn ExposableSecret>,
        api_secret: Box<dyn ExposableSecret>,
        // ... other params
    ) -> Self {
        // ...
    }
}
```

#### After:
```rust
use super::credentials::Credentials;

pub struct RestClient {
    credentials: Credentials,
    // ... other fields
}

impl RestClient {
    pub fn new(
        credentials: Credentials,
        // ... other params
    ) -> Self {
        // ...
    }
}
```

### Step 4: Update Signing Methods

Replace direct access to `api_key` and `api_secret`:

#### Before:
```rust
let api_secret = self.api_secret.expose_secret();
```

#### After:
```rust
let api_secret = self.credentials.api_secret.expose_secret();
```

### Step 5: Update Module Exports

In `mod.rs`, add:
```rust
pub use credentials::Credentials;
```

### Step 6: Update All Usage Points

1. **Examples**: Update any examples that instantiate the client
2. **Tests**: Update integration tests
3. **Documentation**: Update README if applicable

## Special Considerations

### Multi-Product Venues (Binance, GateIO)

For venues with multiple products sharing the same credentials:

1. Create credentials in shared module:
   - `binance/shared/credentials.rs`
   - `gateio/shared/credentials.rs`

2. Re-export from each product's private REST module:
   ```rust
   // In binance/spot/private/rest/mod.rs
   pub use crate::binance::shared::credentials::Credentials;
   ```

### Venue-Specific Fields

Some venues require additional fields:

| Venue | Additional Fields |
|-------|------------------|
| KuCoin | `api_passphrase: SecretString` |
| Coinbase | Might need JWT token field |
| OKX | Might need passphrase |

### Security Requirements

1. **Never use plain `String`** for credentials - always use `SecretString`
2. **Implement safe serialization** that masks secrets (return `"***"`)
3. **Use `expose_secret()`** only when absolutely necessary (signing)
4. **Never log credentials** even in debug mode

## Testing Strategy

### For Each Migration:

1. **Compile Check**:
   ```bash
   cargo check --package venues
   ```

2. **Run Private Integration Tests**:
   ```bash
   RUN_PRIVATE_TESTS=true cargo test [venue_name]::
   ```

3. **Verify Examples** (if applicable):
   ```bash
   cargo run --example [venue_example]
   ```

4. **Security Audit**:
   - Ensure no secrets in logs
   - Verify serialization masks secrets
   - Check that `SecretString` is used throughout

## Migration Checklist

For each venue migration:

- [ ] Create `credentials.rs` file
- [ ] Implement `Credentials` struct with `SecretString` fields
- [ ] Add safe Serialize/Deserialize if needed
- [ ] Update `client.rs` to use `Credentials`
- [ ] Update all signing/auth methods
- [ ] Export `Credentials` from module
- [ ] Update examples (if any)
- [ ] Update integration tests
- [ ] Run tests with `RUN_PRIVATE_TESTS=true`
- [ ] Verify no secrets are exposed in logs/serialization

## Priority Order

### High Priority (Simple, High-Impact)
1. OKX
2. Deribit
3. Coinbase Exchange

### Medium Priority (Multi-Product, Complex)
4. Binance (4 products)
5. GateIO (5 products)

### Low Priority (Single Product, Less Used)
6. Crypto.com
7. Bybit
8. BitMart
9. BingX
10. Bitget
11. KuCoin Futures

## Success Criteria

- All 19 private REST clients use dedicated `Credentials` structs
- No plain `String` types for secrets anywhere
- Consistent pattern across all venues
- All tests pass with proper authentication
- No secrets exposed in logs or serialization

## References

- **Bullish Implementation**: `venues/src/bullish/private/rest/credentials.rs`
- **KuCoin Implementation**: `venues/src/kucoin/spot/private/rest/credentials.rs`
- **SecretString Documentation**: `rest::secrets::SecretString`