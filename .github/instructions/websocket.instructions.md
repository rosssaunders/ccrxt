---
applyTo: "venues/src/**/websocket/**"
---

# WebSocket Implementation Instructions

These instructions apply specifically to WebSocket implementation and extend the general WebSocket instructions.

## Architecture Pattern

- **MUST** use separate public and private WebSocket clients
- **MUST** follow public/private separation established by REST clients
- **MUST NOT** mix authentication concerns between public and private clients

## Architecture Pattern - Composition for Platform Abstraction

Venue WebSocket clients **MUST** use the composition pattern to achieve platform independence:

### The Composition Pattern

```rust
pub struct BinanceSpotWebSocketClient {
    // MUST use trait object for platform abstraction
    inner: Option<Box<dyn WebSocketConnection<BinanceMessage>>>,
    // Venue-specific fields
    subscriptions: HashSet<String>,
    request_id: Arc<AtomicU64>,
}
```

### Why Composition, Not Direct Implementation

1. **Platform Independence**: The `Box<dyn WebSocketConnection<T>>` allows the same venue code to work on:
   - Native Rust (using tokio-tungstenite)
   - WebAssembly (using web-sys WebSocket)
   - Future platforms without code changes

2. **Separation of Concerns**:
   - Generic WebSocket logic (connection, send, receive) in the trait implementation
   - Venue-specific logic (subscriptions, message formatting) in the venue client

3. **Zero-Cost Abstraction**: Platform selection happens at compile time through feature flags

### Implementation Pattern

```rust
impl BinanceSpotWebSocketClient {
    pub async fn connect(&mut self) -> Result<(), WebSocketError> {
        // Create platform-appropriate client via builder
        let client = WebSocketClientBuilder::new(&self.url)
            .build::<BinanceMessage>()  // Returns native OR wasm based on target
            .map_err(|e| WebSocketError::ConnectionFailed(e.to_string()))?;
        
        // Store as trait object
        self.inner = Some(Box::new(client));
        
        // Delegate to inner connection
        if let Some(inner) = &mut self.inner {
            inner.connect().await?;
        }
        
        Ok(())
    }
    
    // All WebSocket operations delegate to inner
    pub fn event_stream(&mut self) -> Pin<Box<dyn Stream<Item = WebSocketEvent<BinanceMessage>> + Send>> {
        if let Some(inner) = &mut self.inner {
            inner.event_stream()
        } else {
            Box::pin(futures::stream::empty())
        }
    }
}

## File Structure Requirements

### Client Structure

- `client.rs` - Contains WebSocket client struct and connection management only
- `client.rs` **MUST NOT** contain message construction, serialization, or endpoint-specific logic
- `client.rs` **MUST** only orchestrate and delegate to endpoint-specific files
- `client.rs` **MUST** use composition pattern with `inner: Option<Box<dyn WebSocketConnection<VenueMessage>>>`
- `client.rs` **MUST NOT** directly implement `WebSocketConnection` trait
- `client.rs` **MUST** delegate core WebSocket operations (connect, disconnect, send, event_stream) to inner connection

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

## Platform Independence Requirements

### Cross-Platform Compatibility

Venue WebSocket clients **MUST** work on both native and WASM targets without code changes:

```rust
// This SAME code must work on both platforms:
let mut client = BinanceSpotWebSocketClient::new();
client.connect().await?;
client.subscribe_trades("BTCUSDT").await?;
```

### Using WebSocketClientBuilder

```rust
impl BinanceSpotWebSocketClient {
    pub fn new() -> Self {
        Self::with_url("wss://stream.binance.com:9443/ws")
    }
    
    pub fn with_url(url: impl Into<String>) -> Self {
        Self {
            inner: None,  // Will be created on connect()
            url: url.into(),
            // ... venue-specific fields
        }
    }
    
    pub async fn connect(&mut self) -> Result<(), WebSocketError> {
        // Builder automatically returns correct implementation for platform
        let client = WebSocketClientBuilder::new(&self.url)
            .build::<BinanceMessage>()?;  // Native or WASM based on compile target
        
        self.inner = Some(Box::new(client));
        
        // Delegate to platform-specific implementation
        if let Some(inner) = &mut self.inner {
            inner.connect().await?;
        }
        
        Ok(())
    }
}
```

### Platform-Specific Code Isolation

- Platform-specific code **MUST** be isolated in the `websockets` crate
- Venue code **MUST NOT** contain `#[cfg(target_arch = "wasm32")]` attributes
- Venue code **MUST NOT** directly use platform-specific libraries (tokio-tungstenite, web-sys)

## Rate Limiting

### WebSocket-Specific Rate Limits

WebSocket rate limiting differs significantly from REST API rate limiting. Each venue has specific limits for:

1. **Message Rate Limits**: Maximum messages per second (e.g., Binance: 5 messages/second)
2. **Connection Limits**: Maximum connections per IP (e.g., Binance: 300 per 5 minutes)
3. **Subscription Limits**: Maximum concurrent subscriptions (e.g., Binance: 1024 streams)
4. **Request Weight**: Some venues apply weight-based limits to WebSocket requests

### Implementation Requirements

#### Rate Limiter Structure

```rust
pub struct WebSocketRateLimiter {
    // Message rate tracking
    message_timestamps: VecDeque<Instant>,
    max_messages_per_second: u32,
    
    // Subscription tracking
    active_subscriptions: HashSet<String>,
    max_subscriptions: u32,
    
    // Connection tracking
    connection_attempts: VecDeque<Instant>,
    max_connections_per_window: u32,
    connection_window: Duration,
    
    // Request weight tracking (if applicable)
    weight_usage: HashMap<Duration, u32>,
    weight_limits: HashMap<Duration, u32>,
}
```

#### Message Rate Limiting

```rust
impl WebSocketRateLimiter {
    pub async fn check_message_rate(&mut self) -> Result<(), RateLimitError> {
        let now = Instant::now();
        
        // Remove timestamps older than 1 second
        while let Some(&front) = self.message_timestamps.front() {
            if now.duration_since(front) > Duration::from_secs(1) {
                self.message_timestamps.pop_front();
            } else {
                break;
            }
        }
        
        // Check if we're at the limit
        if self.message_timestamps.len() >= self.max_messages_per_second as usize {
            return Err(RateLimitError::MessageRateExceeded {
                limit: self.max_messages_per_second,
                window: Duration::from_secs(1),
            });
        }
        
        // Record this message
        self.message_timestamps.push_back(now);
        Ok(())
    }
}
```

#### Subscription Limiting

```rust
impl WebSocketRateLimiter {
    pub fn check_subscription_limit(&self, stream_id: &str) -> Result<(), RateLimitError> {
        if self.active_subscriptions.contains(stream_id) {
            return Ok(()); // Already subscribed
        }
        
        if self.active_subscriptions.len() >= self.max_subscriptions as usize {
            return Err(RateLimitError::SubscriptionLimitExceeded {
                limit: self.max_subscriptions,
                current: self.active_subscriptions.len() as u32,
            });
        }
        
        Ok(())
    }
    
    pub fn add_subscription(&mut self, stream_id: String) {
        self.active_subscriptions.insert(stream_id);
    }
    
    pub fn remove_subscription(&mut self, stream_id: &str) {
        self.active_subscriptions.remove(stream_id);
    }
}
```

#### Connection Rate Limiting

```rust
impl WebSocketRateLimiter {
    pub async fn check_connection_rate(&mut self) -> Result<(), RateLimitError> {
        let now = Instant::now();
        
        // Remove attempts outside the window
        while let Some(&front) = self.connection_attempts.front() {
            if now.duration_since(front) > self.connection_window {
                self.connection_attempts.pop_front();
            } else {
                break;
            }
        }
        
        // Check if we're at the limit
        if self.connection_attempts.len() >= self.max_connections_per_window as usize {
            return Err(RateLimitError::ConnectionRateExceeded {
                limit: self.max_connections_per_window,
                window: self.connection_window,
            });
        }
        
        // Record this connection attempt
        self.connection_attempts.push_back(now);
        Ok(())
    }
}
```

### Integration with WebSocket Client

```rust
pub struct BinanceSpotWebSocketClient {
    inner: Option<Box<dyn WebSocketConnection<BinanceMessage>>>,
    rate_limiter: Arc<RwLock<WebSocketRateLimiter>>,
    subscriptions: HashSet<String>,
    // ... other fields
}

impl BinanceSpotWebSocketClient {
    pub async fn subscribe_trades(&mut self, symbol: &str) -> Result<(), WebSocketError> {
        let stream_id = format!("{}@trade", symbol.to_lowercase());
        
        // Check subscription limit
        {
            let limiter = self.rate_limiter.read().await;
            limiter.check_subscription_limit(&stream_id)
                .map_err(|e| WebSocketError::RateLimitExceeded)?;
        }
        
        // Check message rate
        {
            let mut limiter = self.rate_limiter.write().await;
            limiter.check_message_rate().await
                .map_err(|e| WebSocketError::RateLimitExceeded)?;
        }
        
        // Send subscription message
        let msg = BinanceMessage::Subscribe {
            method: "SUBSCRIBE".to_string(),
            params: vec![stream_id.clone()],
            id: self.next_id(),
        };
        
        if let Some(inner) = &mut self.inner {
            inner.send(msg).await?;
            
            // Update rate limiter
            let mut limiter = self.rate_limiter.write().await;
            limiter.add_subscription(stream_id);
        }
        
        Ok(())
    }
    
    pub async fn connect(&mut self) -> Result<(), WebSocketError> {
        // Check connection rate limit
        {
            let mut limiter = self.rate_limiter.write().await;
            limiter.check_connection_rate().await
                .map_err(|e| WebSocketError::RateLimitExceeded)?;
        }
        
        // Proceed with connection
        // ...
    }
}
```

### Venue-Specific Examples

#### Binance Rate Limits

```rust
impl Default for BinanceWebSocketRateLimiter {
    fn default() -> Self {
        Self {
            max_messages_per_second: 5,
            max_subscriptions: 1024,
            max_connections_per_window: 300,
            connection_window: Duration::from_secs(300), // 5 minutes
            // ...
        }
    }
}
```

#### Coinbase Rate Limits

```rust
impl Default for CoinbaseWebSocketRateLimiter {
    fn default() -> Self {
        Self {
            max_messages_per_second: 100,
            max_subscriptions: 100,
            max_connections_per_window: 20,
            connection_window: Duration::from_secs(60),
            // ...
        }
    }
}
```

### Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum RateLimitError {
    #[error("Message rate exceeded: {limit} messages per second")]
    MessageRateExceeded {
        limit: u32,
        window: Duration,
    },
    
    #[error("Subscription limit exceeded: {limit} (current: {current})")]
    SubscriptionLimitExceeded {
        limit: u32,
        current: u32,
    },
    
    #[error("Connection rate exceeded: {limit} per {window:?}")]
    ConnectionRateExceeded {
        limit: u32,
        window: Duration,
    },
    
    #[error("Request weight exceeded: {used}/{limit} in {window:?}")]
    WeightLimitExceeded {
        used: u32,
        limit: u32,
        window: Duration,
    },
}
```

### User-Facing Rate Limit Information

```rust
impl WebSocketRateLimiter {
    /// Get current rate limit usage statistics
    pub fn get_usage_stats(&self) -> RateLimitStats {
        RateLimitStats {
            messages_in_last_second: self.message_timestamps.len() as u32,
            max_messages_per_second: self.max_messages_per_second,
            active_subscriptions: self.active_subscriptions.len() as u32,
            max_subscriptions: self.max_subscriptions,
            connections_in_window: self.connection_attempts.len() as u32,
            max_connections_per_window: self.max_connections_per_window,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RateLimitStats {
    pub messages_in_last_second: u32,
    pub max_messages_per_second: u32,
    pub active_subscriptions: u32,
    pub max_subscriptions: u32,
    pub connections_in_window: u32,
    pub max_connections_per_window: u32,
}
```

### Separate Rate Limiters

- Public WebSocket **MUST** use public rate limiter
- Private WebSocket **MUST** use private rate limiter
- **MUST NOT** share rate limiters between public and private clients
- Each client instance **MUST** maintain its own rate limit state

### Rate Limit Enforcement

- **MUST** check rate limits before sending any message
- **MUST** check subscription limits before adding new subscriptions
- **MUST** check connection limits before establishing connections
- **MUST** handle rate limit errors appropriately (emit events, return errors)
- **MUST** respect the venue's specific rate limiting rules
- **MUST** expose rate limit usage to users for monitoring
- **MUST NOT** automatically retry rate-limited operations

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
