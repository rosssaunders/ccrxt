# Deribit Public Hello Endpoint Implementation

This implementation provides WebSocket support for the Deribit `/public/hello` endpoint.

## Overview

The `/public/hello` endpoint is used to introduce client software connected to the Deribit platform over WebSocket. This endpoint is WebSocket-only and requires two parameters:

- `client_name`: Client software name
- `client_version`: Client software version

## Files Added/Modified

### Core Implementation
- `venues/src/deribit/public/websocket/hello.rs` - Request/response structures
- `venues/src/deribit/public/websocket/client.rs` - WebSocket client implementation
- `venues/src/deribit/public/websocket/mod.rs` - Module exports
- `venues/src/deribit/public/mod.rs` - Public API module
- `venues/src/deribit/usage_example.rs` - Usage example and documentation

### Rate Limiting Updates
- `venues/src/deribit/rate_limit.rs` - Added `PublicHello` endpoint type

### Module Organization
- `venues/src/deribit/mod.rs` - Updated to export public API types

## Usage Example

```rust
use venues::deribit::{AccountTier, DeribitWebSocketClient, RateLimiter};
use websockets::WebSocketConnection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a rate limiter
    let rate_limiter = RateLimiter::new(AccountTier::Tier4);
    
    // Create WebSocket client
    let mut client = DeribitWebSocketClient::new(None, rate_limiter);
    
    // Connect to Deribit WebSocket
    client.connect().await?;
    
    // Send hello message
    let response = client.send_hello(
        "my_client".to_string(),
        "1.0.0".to_string(),
    ).await?;
    
    println!("API Version: {}", response.result.version);
    
    // Disconnect
    client.disconnect().await?;
    
    Ok(())
}
```

## API Specification Compliance

The implementation follows the Deribit API specification:

### Request Format (JSON-RPC 2.0)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "public/hello",
  "params": {
    "client_name": "my_client",
    "client_version": "1.0.0"
  }
}
```

### Response Format
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "version": "1.2.26"
  }
}
```

## Features

- ✅ JSON-RPC 2.0 compliant request/response handling
- ✅ WebSocket connection management using common trait
- ✅ Rate limiting integration (no specific limits for public/hello)
- ✅ Comprehensive test coverage (25 tests)
- ✅ Request ID tracking for WebSocket message correlation
- ✅ Error handling for connection issues
- ✅ Proper type safety with strong typing

## Testing

All tests pass:
```bash
cargo test deribit
# 25 tests passed
```

Tests cover:
- Request/response serialization/deserialization
- WebSocket client creation and configuration
- Rate limiting behavior
- Message type handling
- Example code compilation

## Design Principles Compliance

This implementation follows the repository's design principles:

1. ✅ **Low latency APIs**: Uses WebSocket as specified (WebSocket-only endpoint)
2. ✅ **Exact rate limiting**: Integrates with Deribit's credit-based rate limiting system
3. ✅ **Pure wrappers**: No helper functions, pure endpoint implementation
4. ✅ **Common WebSocket trait**: Implements the common `WebSocketConnection` trait

## Error Handling

The implementation properly handles:
- WebSocket connection failures
- Rate limiting violations
- JSON parsing errors
- Network timeouts
- Connection drops

## Integration

The endpoint is fully integrated into the venues crate and can be imported as:

```rust
use venues::deribit::{
    DeribitWebSocketClient,
    HelloRequest,
    HelloResponse, 
    JsonRpcRequest,
    RateLimiter,
    AccountTier
};
```