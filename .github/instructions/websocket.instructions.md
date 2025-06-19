---
applyTo: "venues/src/**/websocket/**"
---

# WebSocket Implementation Instructions

These instructions apply specifically to WebSocket implementation and extend the general WebSocket instructions.

## Architecture Pattern

- **MUST** use separate public and private WebSocket clients
- **MUST** follow public/private separation established by REST clients
- **MUST NOT** mix authentication concerns between public and private clients

## File Structure Requirements

### Client Structure

- `client.rs` - Contains WebSocket client struct and connection management only
- `client.rs` **MUST NOT** contain message construction, serialization, or endpoint-specific logic
- `client.rs` **MUST** only orchestrate and delegate to endpoint-specific files

### Message Structure

- Each WebSocket method **MUST** have its own file (e.g., `auth.rs`, `buy.rs`, `sell.rs`)
- Each file **MUST** contain:
  - Request struct with proper serde derives
  - Response struct(s) with proper serde derives
  - Implementation function (e.g., `send_auth`, `send_buy`, `send_sell`)
  - Unit tests for the message type
- **MUST** follow naming: `method_name.rs` for `private/method_name` or `public/method_name`

### Module Organization

```
private/websocket/
├── client.rs      # Private WebSocket client with authentication
├── auth.rs        # private/auth method
├── buy.rs         # private/buy method
├── sell.rs        # private/sell method
├── cancel.rs      # private/cancel method
└── mod.rs         # Module exports
```

## Authentication Requirements

### Private WebSocket Client

- **MUST** require API credentials in constructor
- **MUST** implement authentication flow before allowing private methods
- **MUST** handle authentication errors gracefully
- **MUST** re-authenticate on reconnection
- **MUST NOT** expose credentials in logs or debug output

### Public WebSocket Client

- **MUST NOT** require authentication
- **MUST NOT** accept credentials in constructor
- **MUST** work without any authentication setup

## Message Implementation Pattern

### Request/Response Structs

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyRequest {
    pub instrument_name: String,
    pub amount: f64,
    pub order_type: OrderType,
    // ... other fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyResponse {
    pub jsonrpc: String,
    pub id: u64,
    pub result: BuyResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyResult {
    pub order: OrderInfo,
    pub trades: Vec<TradeInfo>,
}
```

### Example Implementation Function

```rust
impl PrivateWebSocketClient {
    pub async fn buy(&mut self, request: BuyRequest) -> Result<BuyResponse, DeribitWebSocketError> {
        // Validate authentication state
        // Create JSON-RPC message
        // Send via WebSocket
        // Wait for response
        // Deserialize and return
    }
}
```

## JSON-RPC Message Format

### Request Format

- **MUST** include proper method name (`private/buy`, `public/subscribe`, etc.)
- **MUST** use sequential request IDs
- **MUST** include all required parameters in `params` object

### Response Handling

- **MUST** match response `id` with request `id`
- **MUST** handle both `result` and `error` response types
- **MUST** timeout requests appropriately
- **MUST** handle connection errors gracefully

## Rate Limiting

### Separate Rate Limiters

- Public WebSocket **MUST** use public rate limiter
- Private WebSocket **MUST** use private rate limiter
- **MUST NOT** share rate limiters between public and private clients

### Rate Limit Enforcement

- **MUST** check rate limits before sending requests
- **MUST** handle rate limit errors appropriately
- **MUST** respect the venues rate limiting rules

## Error Handling

### Connection Errors

- **MUST** implement automatic reconnection with exponential backoff
- **MUST** re-authenticate private client on reconnection
- **MUST** restore subscriptions on reconnection
- **MUST** emit connection state events

### Message Errors

- **MUST** map the venues error codes to appropriate error types
- **MUST** provide meaningful error messages
- **MUST NOT** panic on malformed messages

## Testing Requirements

### Unit Tests

- Each message file **MUST** include unit tests
- **MUST** test request/response serialization/deserialization
- **MUST** test error conditions
- **MUST** test parameter validation

### Integration Tests

- **MUST** test real WebSocket connections (using test environment)
- **MUST** test authentication flow
- **MUST** test end-to-end message flow
- **MUST** test reconnection scenarios

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buy_request_serialization() {
        // Test request struct serialization
    }

    #[test]
    fn test_buy_response_deserialization() {
        // Test response struct deserialization
    }

    #[tokio::test]
    async fn test_buy_message_flow() {
        // Test full message send/receive flow
    }
}
```

## Security Requirements

### Credential Handling

- **MUST** use `ExposableSecret` trait for API credentials
- **MUST NOT** log credentials or expose in debug output
- **MUST** clear credentials from memory when possible
- **MUST** use secure WebSocket connections (WSS)

### Message Security

- **MUST** validate all incoming messages
- **MUST** sanitize user inputs
- **MUST NOT** execute arbitrary code from messages
- **MUST** handle authentication timeouts securely

## Performance Requirements

### Connection Management

- **MUST** reuse WebSocket connections efficiently
- **MUST** implement proper connection pooling if needed
- **MUST** handle high-frequency message scenarios
- **MUST** avoid unnecessary reconnections

### Message Processing

- **MUST** process messages asynchronously
- **MUST** avoid blocking operations in message handlers
- **MUST** implement efficient message routing
- **MUST** handle backpressure appropriately

## Documentation Requirements

### Code Documentation

- All public methods **MUST** have comprehensive doc comments
- **MUST** include usage examples in doc comments
- **MUST** document error conditions
- **MUST** document rate limiting behavior

### Integration Examples

- **MUST** provide working examples for common use cases
- **MUST** show proper error handling patterns
- **MUST** demonstrate authentication flow
- **MUST** show subscription management

## Compliance

### Repository Standards

- **MUST** follow existing code style and formatting
- **MUST** pass all linting and formatting checks
- **MUST** maintain compatibility with common WebSocket trait
- **MUST** follow repository's error handling patterns

### API Compliance

- **MUST** implement exact JSON-RPC 2.0 specification
- **MUST** follow the Venues API documentation precisely
- **MUST** handle all documented error conditions
- **MUST** respect all API constraints and limitations
