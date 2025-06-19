# WebSocket Implementation Recommendations

## Executive Summary

Based on analysis of the current repository structure, API documentation, and existing WebSocket patterns, this document provides comprehensive recommendations for implementing WebSocket functionality.

**Key Recommendation: Use separate public and private WebSocket clients** for better separation of concerns, security, and maintainability.

## Current State Analysis

### Existing Infrastructure

- ✅ Common WebSocket trait (`WebSocketConnection<T>`) in `/websockets/src/lib.rs`
- ✅ Public WebSocket client implemented in `venues/src/deribit/public/websocket/`
- ✅ REST clients follow public/private separation pattern
- ❌ No private WebSocket implementation exists yet

### API Characteristics

- **Public WebSocket**: No authentication required, methods start with `public/`
- **Private WebSocket**: Authentication required, methods start with `private/`
- **JSON-RPC 2.0**: Both use same message format over WebSocket
- **Rate Limiting**: Different limits for public vs private operations
- **Same URL**: Both public and private use same WebSocket endpoint but with different authentication

## Recommendation: Separate Public and Private WebSocket Clients

### Rationale

**✅ Advantages of Separate Clients:**

1. **Security Isolation**: Private authentication data isolated from public operations
2. **Clear Separation of Concerns**: Matches existing REST client pattern
3. **Simplified Authentication**: Private client always authenticated, public never is
4. **Independent Rate Limiting**: Each client can have appropriate rate limiter
5. **Easier Testing**: Can mock/test public and private functionality independently
6. **Lifecycle Management**: Can connect/disconnect independently based on needs
7. **Error Handling**: Different error recovery strategies for public vs private
8. **Code Maintainability**: Clearer code organization and responsibilities

**❌ Disadvantages:**

1. **Two Connections**: Slightly more connection overhead
2. **Code Duplication**: Some shared WebSocket infrastructure code

**Why This Beats Single Client:**

- Authentication complexity is eliminated (private client always auth'd)
- Rate limiting is simpler (separate rate limiters)
- Follows established repository patterns
- Better security posture
- Easier to understand and maintain

## Recommended File Structure

```
venues/src/deribit/
├── public/
│   ├── rest/              # Existing
│   └── websocket/         # Existing
│       ├── client.rs      # ✅ Public WebSocket client (existing)
│       ├── hello.rs       # ✅ public/hello (existing)
│       ├── subscribe.rs   # ✅ public/subscribe (existing)
│       ├── unsubscribe.rs # 🔄 public/unsubscribe (needs implementation)
│       ├── heartbeat.rs   # 🔄 public/set_heartbeat (needs implementation)
│       └── mod.rs         # ✅ Module exports (existing)
└── private/
    ├── rest/              # Existing
    └── websocket/         # 🆕 NEW - Private WebSocket client
        ├── client.rs      # 🆕 Private WebSocket client with authentication
        ├── auth.rs        # 🆕 private/auth - WebSocket authentication
        ├── buy.rs         # 🆕 private/buy - Place buy orders
        ├── sell.rs        # 🆕 private/sell - Place sell orders
        ├── cancel.rs      # 🆕 private/cancel - Cancel orders
        ├── cancel_all.rs  # 🆕 private/cancel_all - Cancel all orders
        ├── edit.rs        # 🆕 private/edit - Edit orders
        ├── get_positions.rs # 🆕 private/get_positions - Get positions
        ├── get_orders.rs  # 🆕 private/get_open_orders_by_currency - Get orders
        ├── subscribe.rs   # 🆕 private/subscribe - Subscribe to private channels
        └── mod.rs         # 🆕 Module exports
```

### File Structure Principles

Following the established WebSocket instructions:

1. **One file per message type**: Each WebSocket method gets its own file
2. **Complete message implementation**: Request/response structs + execution function in same file
3. **Client orchestration only**: `client.rs` only delegates, no message logic
4. **Mirrors REST structure**: Same modularity as REST endpoints

## Authentication Strategy

### Public WebSocket Client

- **No Authentication**: Connects without credentials
- **Methods**: `public/hello`, `public/subscribe`, `public/unsubscribe`, etc.
- **Rate Limiting**: Public endpoint limits

### Private WebSocket Client

- **Authentication Required**: Must authenticate before using private methods
- **Auth Flow**:
  1. Connect to WebSocket
  2. Send `private/auth` with API credentials
  3. Receive authentication confirmation
  4. Can then use private methods
- **Credentials**: Uses same API key/secret as REST client
- **Rate Limiting**: Private endpoint limits

### Authentication Implementation

```rust
// private/websocket/auth.rs
impl PrivateWebSocketClient {
    pub async fn authenticate(&mut self) -> Result<AuthResponse,WebSocketError> {
        // Create auth request with API credentials
        // Send via WebSocket
        // Wait for auth confirmation
    }
}
```

## Implementation Plan

### Phase 1: Core Private WebSocket Infrastructure

1. Create `venues/src/deribit/private/websocket/client.rs`
2. Implement authentication in `venues/src/deribit/private/websocket/auth.rs`
3. Add module structure and exports

### Phase 2: Essential Trading Operations

1. Implement `buy.rs` - Place buy orders
2. Implement `sell.rs` - Place sell orders
3. Implement `cancel.rs` - Cancel specific orders
4. Implement `cancel_all.rs` - Cancel all orders

### Phase 3: Position and Order Management

1. Implement `get_positions.rs` - Query positions
2. Implement `get_orders.rs` - Query open orders
3. Implement `edit.rs` - Edit existing orders

### Phase 4: Subscriptions and Streaming

1. Implement `subscribe.rs` - Subscribe to private channels
2. Add streaming data handling for trades, orders, positions

### Phase 5: Additional Public WebSocket Methods

1. Complete `unsubscribe.rs` in public WebSocket
2. Implement `heartbeat.rs` for connection management

## Rate Limiting Strategy

### Public Client

```rust
// Use existing public rate limiter
let rate_limiter = RateLimiter::new(account_tier);
let public_client = PublicWebSocketClient::new(None, rate_limiter);
```

### Private Client

```rust
// Use separate private rate limiter
let private_rate_limiter = RateLimiter::new(account_tier);
let private_client = PrivateWebSocketClient::new(
    api_key,
    api_secret,
    None, // URL
    private_rate_limiter
);
```

## Testing Strategy

### Unit Tests

- Test each WebSocket message type independently
- Mock WebSocket connections for testing
- Test authentication flow
- Test error handling

### Integration Tests

- Test real WebSocket connections (using test environment)
- Test authentication with real credentials
- Test message flow end-to-end

### Example Test Structure

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_auth_request_structure() { /* ... */ }

    #[test]
    fn test_buy_order_serialization() { /* ... */ }

    #[tokio::test]
    async fn test_authentication_flow() { /* ... */ }
}
```

## Error Handling

### Connection Errors

- Separate error handling for public vs private connections
- Automatic reconnection strategies
- Authentication re-establishment on reconnect

### Message Errors

- JSON-RPC error handling
- Rate limit error handling
- Invalid message format handling

## Security Considerations

### Credential Management

- Use same secure credential pattern as REST clients
- Never log sensitive authentication data
- Secure credential storage in memory

### Connection Security

- Use WSS (secure WebSocket) connections
- Validate server certificates
- Handle connection state securely

## Future Extensibility

### Message Types

- Easy to add new WebSocket methods by creating new files
- Follows established patterns
- Type-safe message handling

### Multiple Instruments

- Support for multiple currency pairs
- Bulk operations where supported by API
- Efficient subscription management

## Migration Path

### From Current State

1. No breaking changes to existing public WebSocket
2. Add private WebSocket as new functionality
3. Both clients can coexist and be used independently

### Usage Examples

```rust
// Public WebSocket usage (existing)
let mut public_client =WebSocketClient::new(None, rate_limiter);
public_client.connect().await?;
public_client.subscribe(channels).await?;

// Private WebSocket usage (new)
let mut private_client = PrivateWebSocketClient::new(api_key, api_secret, None, rate_limiter);
private_client.connect().await?;
private_client.authenticate().await?;
private_client.buy(order_params).await?;
```

## Conclusion

The recommended approach of **separate public and private WebSocket clients** provides:

- ✅ Clear separation of concerns
- ✅ Better security isolation
- ✅ Simplified authentication management
- ✅ Consistent with existing REST patterns
- ✅ Future extensibility
- ✅ Independent rate limiting

This approach balances simplicity, security, and maintainability while following established repository patterns and WebSocket best practices.
