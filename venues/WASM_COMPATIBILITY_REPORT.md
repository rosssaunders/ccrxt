# WASM Compatibility Report for Venues Crate

## Executive Summary
The venues crate currently has several blockers preventing WASM compilation. These issues stem primarily from native-only dependencies, time handling, cryptography libraries, and async runtime requirements.

## Major Blockers

### 1. Tokio Runtime with "full" Features
**Location:** `venues/Cargo.toml:13`
```toml
tokio = { version = "1.0", features = ["full"] }
```
**Issue:** The "full" feature includes non-WASM compatible components:
- File system operations
- Process management
- Signal handling
- Native networking

**Solution:** Use conditional compilation with reduced feature set for WASM or replace with wasm-bindgen-futures

### 2. Native TLS in tokio-tungstenite
**Location:** `venues/Cargo.toml:14`
```toml
tokio-tungstenite = { version = "0.27.0", features = ["native-tls"] }
```
**Issue:** native-tls uses system SSL libraries that don't exist in WASM environment

**Solution:** 
- For WASM: Use `rustls-tls-webpki-roots` feature or browser's WebSocket API
- Implement conditional features in Cargo.toml

### 3. Ring Cryptography Library
**Location:** Used in Gate.io venue implementations
- `venues/src/gateio/delivery/private/rest/client.rs:7`
- `venues/src/gateio/spot/private/rest/client.rs:7`
- `venues/src/gateio/options/private/rest/client.rs:7`
- `venues/src/gateio/unified/private/rest/client.rs:7`
- `venues/src/gateio/perpetual/private/rest/client.rs:7`

**Issue:** Ring contains assembly code and C dependencies that don't compile to WASM

**Solution:** Replace with pure Rust alternatives:
- Use `hmac` and `sha2` crates directly (both support WASM)
- These are already dependencies, just need to migrate the code

### 4. std::time::Instant Usage
**Locations:** Rate limiting implementations
- `venues/src/bybit/rate_limit.rs` - Multiple uses of `Instant::now()`
- `venues/src/deribit/rate_limit.rs` - Token bucket implementation
- `venues/src/bitmart/contract/rate_limit.rs` - Duration handling

**Issue:** `std::time::Instant` is not available in WASM (no monotonic clock)

**Solution:** 
- Use `web-time` crate which provides WASM-compatible `Instant`
- Add conditional compilation: `#[cfg(target_arch = "wasm32")]`

### 5. Governor Rate Limiting Crate
**Location:** `venues/src/bitmart/contract/rate_limit.rs`
```rust
use governor::{Quota, RateLimiter, clock::DefaultClock, state::keyed::DefaultKeyedStateStore};
```
**Issue:** Governor internally uses `std::time::Instant`

**Solution:** 
- Create abstraction layer for rate limiting
- Implement WASM-compatible rate limiter using browser's performance.now()
- Use feature flags to switch implementations

### 6. Terminal UI Dependencies
**Location:** `venues/Cargo.toml:30-31`
```toml
ratatui = { workspace = true }
crossterm = { workspace = true }
```
**Issue:** Terminal UI libraries require OS-level terminal access

**Solution:** Make these optional dependencies with a feature flag

### 7. Dotenv Dependency
**Location:** `venues/Cargo.toml:34`
```toml
dotenv = "0.15"
```
**Issue:** File system access for .env files doesn't work in WASM

**Solution:** 
- Currently unused, can be removed
- If needed, use browser/node environment variable APIs

### 8. tokio::time::sleep Usage
**Location:** `venues/src/deribit/rate_limit.rs:774`
```rust
tokio::time::sleep(Duration::from_millis(50)).await;
```
**Issue:** Tokio timers don't work in WASM

**Solution:** Use wasm-bindgen futures and browser setTimeout

## Recommended Implementation Strategy

### Phase 1: Create Feature Flags
```toml
[features]
default = ["native"]
native = ["tokio/full", "tokio-tungstenite/native-tls", "terminal-ui"]
wasm = ["tokio/sync", "tokio-tungstenite/rustls-tls-webpki-roots", "web-time", "getrandom/js"]
terminal-ui = ["ratatui", "crossterm"]
```

### Phase 2: Abstract Time Handling
Create a `time` module with conditional compilation:
```rust
#[cfg(not(target_arch = "wasm32"))]
pub use std::time::{Duration, Instant};

#[cfg(target_arch = "wasm32")]
pub use web_time::{Duration, Instant};
```

### Phase 3: Replace Ring with Pure Rust
Migrate Gate.io HMAC implementations from Ring to hmac/sha2 crates.

### Phase 4: Abstract Rate Limiting
Create trait-based rate limiting with separate implementations for native and WASM.

### Phase 5: Conditional Async Runtime
Use cfg attributes to switch between tokio (native) and wasm-bindgen-futures (WASM).

## Testing Strategy
1. Set up WASM build target: `rustup target add wasm32-unknown-unknown`
2. Create test build script with `--target wasm32-unknown-unknown`
3. Use wasm-pack for browser testing
4. Set up CI to verify both native and WASM builds

## Dependencies to Add for WASM Support
```toml
[dependencies]
web-time = { version = "1.0", optional = true }
wasm-bindgen = { version = "0.2", optional = true }
wasm-bindgen-futures = { version = "0.4", optional = true }
getrandom = { version = "0.2", features = ["js"], optional = true }
```

## Timeline Estimate
- Phase 1-2: 1 day (feature flags and time abstraction)
- Phase 3: 1 day (Ring replacement)
- Phase 4: 2 days (rate limiting abstraction)
- Phase 5: 2-3 days (async runtime abstraction)
- Testing: 2 days

**Total: ~1.5 weeks for full WASM compatibility**