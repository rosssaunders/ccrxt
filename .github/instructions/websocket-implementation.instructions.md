---
applyTo: "websockets/**,venues/src/**/websocket/**"
---

# WebSocket Implementation Specification

This specification defines how WebSocket clients must be implemented across all venues to ensure consistency, cross-platform compatibility (native Rust and WASM), and user control over connection management.

## Core Principles

1. **Transparency**: All connection events MUST be visible to library users
2. **User Control**: Users decide when/how to connect/reconnect - NO automatic reconnection
3. **Platform Parity**: Same API and behavior across native and WASM targets
4. **Pure Wrappers**: WebSocket implementations are pure wrappers - no fix-up logic
5. **Event-Driven**: All state changes communicated via events
6. **Composition Pattern**: Venue clients use composition with trait objects for platform abstraction

## Disconnection and Reconnection Handling

### How Users Get Notified

Users receive ALL connection state changes through the event stream:

```rust
WebSocketEvent::Disconnected { reason: DisconnectReason }
```

Where `DisconnectReason` provides detailed information:
- `UserInitiated` - User called disconnect()
- `RemoteClosed { code, reason }` - Server closed connection with WebSocket close code
- `NetworkError { details }` - Network failure (connection lost, timeout, etc.)
- `ProtocolError { details }` - WebSocket protocol violation
- `InvalidMessage { details }` - Deserialization or invalid message format

### No Automatic Reconnection

The library does NOT automatically reconnect. This is by design because:
1. **Visibility** - Users need to know when disconnections happen
2. **Control** - Different applications have different reconnection requirements
3. **Context** - Some disconnections shouldn't trigger reconnection (e.g., authentication failures, policy violations)
4. **State** - Users may need to re-subscribe to streams or re-authenticate after reconnection
5. **Backoff** - Reconnection strategy (exponential backoff, max attempts) is application-specific

### User Implementation Pattern

Users implement their own reconnection logic based on their requirements:

```rust
async fn run_with_reconnection() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BinanceSpotWebSocketClient::new();
    let mut reconnect_attempts = 0;
    const MAX_ATTEMPTS: u32 = 5;
    
    loop {
        // Connect
        match client.connect().await {
            Ok(_) => {
                println!("Connected");
                reconnect_attempts = 0; // Reset on successful connection
                
                // Subscribe to streams
                client.subscribe_trades("BTCUSDT").await?;
                
                // Process events
                let mut events = client.event_stream();
                while let Some(event) = events.next().await {
                    match event {
                        WebSocketEvent::Disconnected { reason } => {
                            println!("Disconnected: {:?}", reason);
                            
                            // Decide whether to reconnect
                            match reason {
                                DisconnectReason::UserInitiated => {
                                    return Ok(()); // Don't reconnect
                                }
                                DisconnectReason::RemoteClosed { code, .. } => {
                                    if code == 1008 { // Policy violation
                                        return Err("Policy violation".into());
                                    }
                                    break; // Reconnect
                                }
                                _ => break, // Reconnect for other reasons
                            }
                        }
                        WebSocketEvent::Message { message } => {
                            // Process message
                        }
                        WebSocketEvent::Error { error } => {
                            // Log error but continue (not necessarily fatal)
                            eprintln!("Error: {}", error);
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
        
        // Reconnection logic with exponential backoff
        reconnect_attempts += 1;
        if reconnect_attempts > MAX_ATTEMPTS {
            return Err("Max reconnection attempts exceeded".into());
        }
        
        let backoff = Duration::from_secs(2u64.pow(reconnect_attempts.min(6)));
        println!("Reconnecting in {:?}...", backoff);
        sleep(backoff).await;
        
        // Cleanup before reconnecting
        let _ = client.disconnect().await;
    }
}
```

## Architecture Overview

### Layer Structure

```
┌─────────────────────────────────────┐
│         User Application            │
├─────────────────────────────────────┤
│      Venue WebSocket Client         │  (e.g., BinanceWebSocketClient)
├─────────────────────────────────────┤
│    WebSocket Abstraction Layer      │  (websockets crate)
├─────────────────────────────────────┤
│     Platform Transport Layer        │  (Native or WASM)
└─────────────────────────────────────┘
```

## 1. Core WebSocket Traits (`websockets` crate)

### 1.1 WebSocketConnection Trait

```rust
#[cfg(not(target_arch = "wasm32"))]
#[async_trait]
pub trait WebSocketConnection<T>: Send + Sync
where
    T: VenueMessage,
{
    /// Connect to the WebSocket endpoint
    /// Returns error if connection fails
    /// Does NOT automatically retry
    async fn connect(&mut self) -> BoxResult<()>;
    
    /// Disconnect from the WebSocket endpoint
    /// Gracefully closes the connection
    async fn disconnect(&mut self) -> BoxResult<()>;
    
    /// Check if currently connected
    fn is_connected(&self) -> bool;
    
    /// Get detailed connection state
    fn connection_state(&self) -> ConnectionState;
    
    /// Get a stream of all WebSocket events
    /// Includes connection events and messages
    fn event_stream(&mut self) -> Pin<Box<dyn Stream<Item = WebSocketEvent<T>> + Send>>;
    
    /// Send a message over the WebSocket
    async fn send(&mut self, message: T) -> BoxResult<()>;
}

// WASM version without Send + Sync bounds
#[cfg(target_arch = "wasm32")]
#[async_trait(?Send)]
pub trait WebSocketConnection<T>
where
    T: VenueMessage,
{
    // Same methods as above but without Send + Sync requirements
}
```

### 1.2 Event System

```rust
/// All possible WebSocket events
#[derive(Debug, Clone)]
pub enum WebSocketEvent<T: VenueMessage> {
    /// Connection established successfully
    Connected,
    
    /// Connection closed (with reason)
    Disconnected { reason: DisconnectReason },
    
    /// Error occurred (connection may still be active)
    Error { error: WebSocketError },
    
    /// Message received from server
    Message { message: T },
    
    /// Ping received (for venues that expose ping/pong)
    PingReceived { data: Vec<u8> },
    
    /// Pong received
    PongReceived { data: Vec<u8> },
}

#[derive(Debug, Clone)]
pub enum DisconnectReason {
    /// User called disconnect()
    UserInitiated,
    
    /// Server closed connection
    RemoteClosed { code: u16, reason: String },
    
    /// Network error
    NetworkError { details: String },
    
    /// Protocol error
    ProtocolError { details: String },
    
    /// Invalid message received
    InvalidMessage { details: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    /// Not connected
    Disconnected,
    
    /// Connection in progress
    Connecting,
    
    /// Connected and ready
    Connected,
    
    /// Disconnection in progress
    Disconnecting,
}
```

### 1.3 WebSocket Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum WebSocketError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Already connected")]
    AlreadyConnected,
    
    #[error("Not connected")]
    NotConnected,
    
    #[error("Send failed: {0}")]
    SendFailed(String),
    
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Platform error: {0}")]
    PlatformError(String),
}
```

## 2. Platform Implementations

### 2.1 Native Implementation (`websockets/src/native.rs`)

**MUST** use `tokio-tungstenite` for WebSocket connections.

```rust
#[cfg(not(target_arch = "wasm32"))]
pub struct NativeWebSocketClient<T: VenueMessage> {
    url: String,
    ws_stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    state: ConnectionState,
    event_tx: mpsc::UnboundedSender<WebSocketEvent<T>>,
    event_rx: Option<mpsc::UnboundedReceiver<WebSocketEvent<T>>>,
}
```

Key requirements:
- **MUST NOT** implement automatic reconnection
- **MUST** emit `Disconnected` event when connection drops
- **MUST** handle backpressure appropriately
- **MUST** support TLS connections

### 2.2 WASM Implementation (`websockets/src/wasm.rs`)

**MUST** use `web-sys` WebSocket API.

```rust
#[cfg(target_arch = "wasm32")]
pub struct WasmWebSocketClient<T: VenueMessage> {
    url: String,
    ws: Option<web_sys::WebSocket>,
    state: ConnectionState,
    event_tx: mpsc::UnboundedSender<WebSocketEvent<T>>,
    event_rx: Option<mpsc::UnboundedReceiver<WebSocketEvent<T>>>,
}
```

Key requirements:
- **MUST** handle browser-specific WebSocket constraints
- **MUST** use JavaScript event callbacks
- **MUST** handle browser connection limits
- **MUST** emit same events as native implementation

### 2.3 Platform Selection

```rust
/// Platform-agnostic WebSocket client builder
pub struct WebSocketClientBuilder {
    url: String,
    headers: HashMap<String, String>,
}

impl WebSocketClientBuilder {
    pub fn new(url: impl Into<String>) -> Self { ... }
    
    pub fn header(mut self, key: String, value: String) -> Self { ... }
    
    #[cfg(not(target_arch = "wasm32"))]
    pub fn build<T: VenueMessage>() -> Result<NativeWebSocketClient<T>, WebSocketError> { ... }
    
    #[cfg(target_arch = "wasm32")]
    pub fn build<T: VenueMessage>() -> Result<WasmWebSocketClient<T>, WebSocketError> { ... }
}
```

## 2.4 Composition Pattern for Platform Abstraction

### The Pattern

The composition pattern is **REQUIRED** for all venue WebSocket implementations. This pattern enables:

1. **Single Codebase**: Write venue code once, run everywhere (native & WASM)
2. **Platform Independence**: No `#[cfg]` attributes in venue code
3. **Type Safety**: Strong typing with venue-specific message types
4. **Testability**: Easy to inject mock implementations
5. **Future Proof**: New platforms can be added without changing venue code

### How It Works

```
┌─────────────────────────────────────────────────────────┐
│                  User Application Code                   │
│                 let client = BinanceClient::new()        │
└─────────────────────┬───────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────┐
│              Venue WebSocket Client                      │
│         struct BinanceClient {                           │
│           inner: Option<Box<dyn WebSocketConnection>>    │
│         }                                                │
└─────────────────────┬───────────────────────────────────┘
                      │ connect()
┌─────────────────────▼───────────────────────────────────┐
│            WebSocketClientBuilder                        │
│         Detects platform at compile time                 │
└────────┬────────────────────────────┬───────────────────┘
         │                            │
    [Native Build]              [WASM Build]
         │                            │
┌────────▼──────────┐      ┌─────────▼──────────┐
│NativeWebSocketClient│      │WasmWebSocketClient│
│ tokio-tungstenite  │      │    web-sys        │
└───────────────────┘      └────────────────────┘
         │                            │
         └────────────┬───────────────┘
                      │
            Both implement WebSocketConnection<T>
                      │
         Stored as Box<dyn WebSocketConnection<T>>
```

### Implementation Example

```rust
// This is how Binance ACTUALLY implements it:
pub struct BinanceSpotWebSocketClient {
    /// Inner WebSocket connection - trait object for platform abstraction
    inner: Option<Box<dyn WebSocketConnection<BinanceMessage>>>,
    /// WebSocket endpoint URL
    url: String,
    /// Request ID counter
    request_id: Arc<AtomicU64>,
    /// Active subscriptions
    subscriptions: HashSet<String>,
}

impl BinanceSpotWebSocketClient {
    pub async fn connect(&mut self) -> Result<(), WebSocketError> {
        // Builder returns platform-specific type at compile time
        let client = WebSocketClientBuilder::new(&self.url)
            .build::<BinanceMessage>()?;  // NativeWebSocketClient OR WasmWebSocketClient
        
        // Box into trait object - this is the key abstraction
        self.inner = Some(Box::new(client));
        
        // All operations delegate to the trait object
        if let Some(inner) = &mut self.inner {
            inner.connect().await?;
        }
        
        Ok(())
    }
}
```

### Why Not Direct Implementation?

Venues **MUST NOT** implement `WebSocketConnection` directly because:

1. **Platform Lock-in**: Direct implementation would require platform-specific code
2. **Code Duplication**: Would need separate implementations for native and WASM
3. **Maintenance Burden**: Changes to trait would require updating all venues twice
4. **Testing Complexity**: Harder to mock and test

### Benefits of Composition

1. **Zero-Cost Abstraction**: Platform selection happens at compile time
2. **Clean Separation**: Platform code in `websockets` crate, venue logic in venue crate
3. **Flexibility**: Venues can add custom methods while delegating core functionality
4. **Consistency**: All venues follow the same pattern

## 3. Venue Implementation Requirements

### 3.1 File Structure

Each venue **MUST** follow this structure:

```
venues/src/<exchange>/websocket/
├── mod.rs              # Module exports
├── message.rs          # Venue-specific message types
├── error.rs            # Venue-specific errors
├── public/
│   ├── mod.rs
│   ├── client.rs       # Public WebSocket client
│   ├── subscribe.rs    # Subscription methods
│   └── unsubscribe.rs  # Unsubscription methods
└── private/
    ├── mod.rs
    ├── client.rs       # Private WebSocket client
    ├── auth.rs         # Authentication flow
    └── orders.rs       # Order management
```

### 3.2 Venue Message Types

Each venue **MUST** define its message types:

```rust
/// Venue-specific message wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BinanceMessage {
    /// Subscription confirmation
    SubscribeResponse(SubscribeResponse),
    
    /// Market data update
    MarketData(MarketDataUpdate),
    
    /// Order update
    OrderUpdate(OrderUpdate),
    
    /// Error message
    Error(ErrorMessage),
}

impl VenueMessage for BinanceMessage {}
```

### 3.3 Venue Client Implementation - Composition Pattern

**IMPORTANT**: Venue clients **MUST NOT** directly implement `WebSocketConnection`. They **MUST** use composition.

```rust
pub struct BinanceWebSocketClient {
    // MUST use Option<Box<dyn WebSocketConnection<T>>> for lazy initialization
    inner: Option<Box<dyn WebSocketConnection<BinanceMessage>>>,
    // Store URL for connection
    url: String,
    // Venue-specific state
    subscriptions: HashSet<String>,
    request_id: Arc<AtomicU64>,
}

impl BinanceWebSocketClient {
    /// Create new client - does NOT connect immediately
    pub fn new() -> Self {
        Self::with_url("wss://stream.binance.com:9443/ws")
    }
    
    pub fn with_url(url: impl Into<String>) -> Self {
        Self {
            inner: None,  // Connection created on connect()
            url: url.into(),
            subscriptions: HashSet::new(),
            request_id: Arc::new(AtomicU64::new(1)),
        }
    }
    
    /// Connect to WebSocket endpoint
    /// Creates platform-appropriate implementation via builder
    pub async fn connect(&mut self) -> Result<(), WebSocketError> {
        // Use builder to get platform-specific implementation
        let client = WebSocketClientBuilder::new(&self.url)
            .build::<BinanceMessage>()  // Returns NativeWebSocketClient or WasmWebSocketClient
            .map_err(|e| WebSocketError::ConnectionFailed(e.to_string()))?;
        
        // Box it as trait object for platform abstraction
        self.inner = Some(Box::new(client));
        
        // Delegate actual connection to inner implementation
        if let Some(inner) = &mut self.inner {
            inner.connect().await?;
        }
        
        Ok(())
    }
    
    /// Disconnect from WebSocket
    pub async fn disconnect(&mut self) -> Result<(), WebSocketError> {
        if let Some(inner) = &mut self.inner {
            inner.disconnect().await?;
        }
        self.subscriptions.clear();
        Ok(())
    }
    
    /// Check connection status - delegates to inner
    pub fn is_connected(&self) -> bool {
        self.inner
            .as_ref()
            .map(|i| i.is_connected())
            .unwrap_or(false)
    }
    
    /// Get event stream - delegates to inner
    pub fn event_stream(&mut self) -> Pin<Box<dyn Stream<Item = WebSocketEvent<BinanceMessage>> + Send>> {
        if let Some(inner) = &mut self.inner {
            inner.event_stream()
        } else {
            Box::pin(futures::stream::empty())
        }
    }
    
    /// Subscribe to market data - venue-specific logic
    pub async fn subscribe(&mut self, symbol: &str) -> Result<(), WebSocketError> {
        if !self.is_connected() {
            return Err(WebSocketError::NotConnected);
        }
        
        // Venue-specific message construction
        let msg = BinanceMessage::Subscribe { 
            symbol: symbol.to_string(),
            id: self.request_id.fetch_add(1, Ordering::SeqCst),
        };
        
        // Delegate sending to inner
        if let Some(inner) = &mut self.inner {
            inner.send(msg).await?;
            self.subscriptions.insert(symbol.to_string());
        }
        
        Ok(())
    }
}
```

## 4. Connection Management Patterns

### 4.1 User Reconnection Example

Users **MUST** implement their own reconnection logic:

```rust
/// Example user implementation of reconnection with exponential backoff
async fn maintain_connection(client: &mut BinanceWebSocketClient) {
    let mut backoff = Duration::from_secs(1);
    let max_backoff = Duration::from_secs(60);
    
    loop {
        match client.event_stream().next().await {
            Some(WebSocketEvent::Disconnected { reason }) => {
                log::warn!("WebSocket disconnected: {:?}", reason);
                
                // User decides whether to reconnect
                if should_reconnect(&reason) {
                    log::info!("Attempting reconnection in {:?}", backoff);
                    tokio::time::sleep(backoff).await;
                    
                    match client.connect().await {
                        Ok(_) => {
                            log::info!("Reconnected successfully");
                            backoff = Duration::from_secs(1);
                            // Re-subscribe to channels
                            resubscribe(client).await;
                        }
                        Err(e) => {
                            log::error!("Reconnection failed: {}", e);
                            backoff = (backoff * 2).min(max_backoff);
                        }
                    }
                }
            }
            Some(WebSocketEvent::Message { message }) => {
                // Process message
                handle_message(message);
            }
            Some(WebSocketEvent::Error { error }) => {
                log::error!("WebSocket error: {}", error);
                // User decides how to handle error
            }
            _ => {}
        }
    }
}
```

### 4.2 NO Automatic Reconnection

Implementations **MUST NOT**:
- Automatically reconnect on disconnection
- Hide connection failures from users
- Retry operations without user knowledge
- Implement internal reconnection loops

## 5. Authentication

### 5.1 Private WebSocket Clients

Private clients **MUST**:
- Require credentials in constructor
- Expose authentication state
- Emit authentication errors as events
- NOT automatically re-authenticate

```rust
pub struct PrivateWebSocketClient {
    inner: Box<dyn WebSocketConnection<VenueMessage>>,
    api_key: SecretString,
    api_secret: SecretString,
    authenticated: bool,
}

impl PrivateWebSocketClient {
    pub async fn authenticate(&mut self) -> Result<(), WebSocketError> {
        // Send authentication message
        // Wait for authentication response
        // Set authenticated flag
        // Return error if authentication fails
    }
}
```

## 6. Rate Limiting

### 6.1 WebSocket Rate Limiting Overview

WebSocket rate limiting is fundamentally different from REST API rate limiting:

| Aspect | REST API | WebSocket |
|--------|----------|----------|
| Scope | Request-based | Connection & message-based |
| Limits | Request weight, order count | Messages/sec, subscriptions, connections |
| Tracking | Per endpoint weight | Per message, per stream |
| Reset | Time windows (1min, 5min) | Rolling windows, connection lifetime |

### 6.2 Types of WebSocket Rate Limits

#### 6.2.1 Message Rate Limits

**Purpose**: Prevent message flooding
**Typical Limits**: 5-100 messages per second
**Enforcement**: Per connection

```rust
/// Track outbound message rate
pub struct MessageRateLimiter {
    timestamps: VecDeque<Instant>,
    max_per_second: u32,
}

impl MessageRateLimiter {
    pub fn check_and_record(&mut self) -> Result<(), RateLimitError> {
        let now = Instant::now();
        let cutoff = now - Duration::from_secs(1);
        
        // Remove old timestamps
        while self.timestamps.front().map_or(false, |&t| t < cutoff) {
            self.timestamps.pop_front();
        }
        
        // Check limit
        if self.timestamps.len() >= self.max_per_second as usize {
            let wait_until = self.timestamps.front().unwrap() + Duration::from_secs(1);
            return Err(RateLimitError::MessageRate { 
                wait_until,
                limit: self.max_per_second,
            });
        }
        
        self.timestamps.push_back(now);
        Ok(())
    }
}
```

#### 6.2.2 Subscription Limits

**Purpose**: Limit resource usage per connection
**Typical Limits**: 100-1024 concurrent subscriptions
**Enforcement**: Per connection

```rust
/// Track active subscriptions
pub struct SubscriptionLimiter {
    active: HashSet<String>,
    max_subscriptions: u32,
}

impl SubscriptionLimiter {
    pub fn check(&self, stream_id: &str) -> Result<(), RateLimitError> {
        if self.active.contains(stream_id) {
            return Ok(()); // Already subscribed
        }
        
        if self.active.len() >= self.max_subscriptions as usize {
            return Err(RateLimitError::SubscriptionLimit {
                current: self.active.len() as u32,
                limit: self.max_subscriptions,
            });
        }
        
        Ok(())
    }
    
    pub fn add(&mut self, stream_id: String) {
        self.active.insert(stream_id);
    }
    
    pub fn remove(&mut self, stream_id: &str) -> bool {
        self.active.remove(stream_id)
    }
    
    pub fn clear(&mut self) {
        self.active.clear();
    }
}
```

#### 6.2.3 Connection Limits

**Purpose**: Prevent connection flooding
**Typical Limits**: 20-300 connections per time window
**Enforcement**: Per IP address

```rust
/// Track connection attempts
pub struct ConnectionLimiter {
    attempts: VecDeque<Instant>,
    max_per_window: u32,
    window: Duration,
}

impl ConnectionLimiter {
    pub fn check_and_record(&mut self) -> Result<(), RateLimitError> {
        let now = Instant::now();
        let cutoff = now - self.window;
        
        // Remove old attempts
        while self.attempts.front().map_or(false, |&t| t < cutoff) {
            self.attempts.pop_front();
        }
        
        // Check limit
        if self.attempts.len() >= self.max_per_window as usize {
            let wait_until = self.attempts.front().unwrap() + self.window;
            return Err(RateLimitError::ConnectionRate {
                wait_until,
                limit: self.max_per_window,
                window: self.window,
            });
        }
        
        self.attempts.push_back(now);
        Ok(())
    }
}
```

#### 6.2.4 Request Weight Limits (Venue-Specific)

Some venues apply weight-based limits to WebSocket operations:

```rust
/// Track weighted operations
pub struct WeightLimiter {
    usage: HashMap<Duration, VecDeque<(Instant, u32)>>,
    limits: HashMap<Duration, u32>,
}

impl WeightLimiter {
    pub fn check_and_record(&mut self, weight: u32, window: Duration) -> Result<(), RateLimitError> {
        let now = Instant::now();
        let cutoff = now - window;
        
        let entries = self.usage.entry(window).or_default();
        
        // Remove old entries
        while entries.front().map_or(false, |(t, _)| *t < cutoff) {
            entries.pop_front();
        }
        
        // Calculate current usage
        let current: u32 = entries.iter().map(|(_, w)| w).sum();
        let limit = self.limits.get(&window).copied().unwrap_or(u32::MAX);
        
        if current + weight > limit {
            return Err(RateLimitError::WeightLimit {
                used: current,
                requested: weight,
                limit,
                window,
            });
        }
        
        entries.push_back((now, weight));
        Ok(())
    }
}
```

### 6.3 Integrated Rate Limiter

```rust
/// Comprehensive WebSocket rate limiter
pub struct WebSocketRateLimiter {
    message_limiter: MessageRateLimiter,
    subscription_limiter: SubscriptionLimiter,
    connection_limiter: ConnectionLimiter,
    weight_limiter: Option<WeightLimiter>, // Not all venues use this
}

impl WebSocketRateLimiter {
    /// Create with venue-specific limits
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            message_limiter: MessageRateLimiter {
                timestamps: VecDeque::new(),
                max_per_second: config.max_messages_per_second,
            },
            subscription_limiter: SubscriptionLimiter {
                active: HashSet::new(),
                max_subscriptions: config.max_subscriptions,
            },
            connection_limiter: ConnectionLimiter {
                attempts: VecDeque::new(),
                max_per_window: config.max_connections_per_window,
                window: config.connection_window,
            },
            weight_limiter: config.weight_limits.map(|limits| WeightLimiter {
                usage: HashMap::new(),
                limits,
            }),
        }
    }
    
    /// Check before sending any message
    pub async fn check_message(&mut self) -> Result<(), RateLimitError> {
        self.message_limiter.check_and_record()
    }
    
    /// Check before subscribing
    pub fn check_subscription(&self, stream_id: &str) -> Result<(), RateLimitError> {
        self.subscription_limiter.check(stream_id)
    }
    
    /// Check before connecting
    pub async fn check_connection(&mut self) -> Result<(), RateLimitError> {
        self.connection_limiter.check_and_record()
    }
    
    /// Check weighted operation
    pub async fn check_weight(&mut self, weight: u32, window: Duration) -> Result<(), RateLimitError> {
        if let Some(limiter) = &mut self.weight_limiter {
            limiter.check_and_record(weight, window)
        } else {
            Ok(())
        }
    }
    
    /// Record successful subscription
    pub fn add_subscription(&mut self, stream_id: String) {
        self.subscription_limiter.add(stream_id);
    }
    
    /// Remove subscription
    pub fn remove_subscription(&mut self, stream_id: &str) {
        self.subscription_limiter.remove(stream_id);
    }
    
    /// Reset on disconnect
    pub fn reset_connection_state(&mut self) {
        self.subscription_limiter.clear();
        self.message_limiter.timestamps.clear();
        // Note: connection_limiter persists across reconnections
    }
    
    /// Get current usage statistics
    pub fn get_stats(&self) -> RateLimitStats {
        RateLimitStats {
            messages_in_last_second: self.message_limiter.timestamps.len() as u32,
            active_subscriptions: self.subscription_limiter.active.len() as u32,
            recent_connections: self.connection_limiter.attempts.len() as u32,
        }
    }
}
```

### 6.4 Venue-Specific Configurations

```rust
/// Binance WebSocket rate limits
pub fn binance_rate_limits() -> RateLimitConfig {
    RateLimitConfig {
        max_messages_per_second: 5,
        max_subscriptions: 1024,
        max_connections_per_window: 300,
        connection_window: Duration::from_secs(300), // 5 minutes
        weight_limits: None, // Binance doesn't use weight for WS
    }
}

/// Coinbase WebSocket rate limits
pub fn coinbase_rate_limits() -> RateLimitConfig {
    RateLimitConfig {
        max_messages_per_second: 100,
        max_subscriptions: 100,
        max_connections_per_window: 20,
        connection_window: Duration::from_secs(60),
        weight_limits: None,
    }
}

/// OKX WebSocket rate limits
pub fn okx_rate_limits() -> RateLimitConfig {
    RateLimitConfig {
        max_messages_per_second: 30,
        max_subscriptions: 240,
        max_connections_per_window: 100,
        connection_window: Duration::from_secs(60),
        weight_limits: Some(HashMap::from([
            (Duration::from_secs(1), 120), // 120 weight per second
        ])),
    }
}
```

### 6.5 Integration with WebSocket Client

```rust
impl BinanceSpotWebSocketClient {
    pub fn new() -> Self {
        Self {
            inner: None,
            rate_limiter: Arc::new(RwLock::new(
                WebSocketRateLimiter::new(binance_rate_limits())
            )),
            // ...
        }
    }
    
    pub async fn connect(&mut self) -> Result<(), WebSocketError> {
        // Check connection rate limit
        self.rate_limiter.write().await
            .check_connection().await
            .map_err(|e| WebSocketError::RateLimitExceeded)?;
        
        // Proceed with connection
        let client = WebSocketClientBuilder::new(&self.url)
            .build::<BinanceMessage>()?;
        
        self.inner = Some(Box::new(client));
        
        if let Some(inner) = &mut self.inner {
            inner.connect().await?;
        }
        
        Ok(())
    }
    
    pub async fn subscribe_trades(&mut self, symbol: &str) -> Result<(), WebSocketError> {
        let stream_id = format!("{}@trade", symbol.to_lowercase());
        
        // Check subscription limit
        self.rate_limiter.read().await
            .check_subscription(&stream_id)
            .map_err(|e| WebSocketError::RateLimitExceeded)?;
        
        // Check message rate
        self.rate_limiter.write().await
            .check_message().await
            .map_err(|e| WebSocketError::RateLimitExceeded)?;
        
        // Send subscription
        let msg = self.create_subscribe_message(&stream_id);
        
        if let Some(inner) = &mut self.inner {
            inner.send(msg).await?;
            
            // Record subscription
            self.rate_limiter.write().await
                .add_subscription(stream_id);
        }
        
        Ok(())
    }
    
    pub async fn disconnect(&mut self) -> Result<(), WebSocketError> {
        if let Some(inner) = &mut self.inner {
            inner.disconnect().await?;
        }
        
        // Reset connection-specific state
        self.rate_limiter.write().await
            .reset_connection_state();
        
        Ok(())
    }
    
    /// Expose rate limit statistics to users
    pub async fn get_rate_limit_stats(&self) -> RateLimitStats {
        self.rate_limiter.read().await.get_stats()
    }
}
```

### 6.6 Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum RateLimitError {
    #[error("Message rate exceeded: {limit}/sec, retry after {wait_until:?}")]
    MessageRate {
        wait_until: Instant,
        limit: u32,
    },
    
    #[error("Subscription limit exceeded: {current}/{limit}")]
    SubscriptionLimit {
        current: u32,
        limit: u32,
    },
    
    #[error("Connection rate exceeded: {limit} per {window:?}, retry after {wait_until:?}")]
    ConnectionRate {
        wait_until: Instant,
        limit: u32,
        window: Duration,
    },
    
    #[error("Weight limit exceeded: {used}+{requested} > {limit} in {window:?}")]
    WeightLimit {
        used: u32,
        requested: u32,
        limit: u32,
        window: Duration,
    },
}
```

### 6.7 User Guidance

#### Handling Rate Limit Errors

```rust
// User code example
async fn subscribe_with_retry(client: &mut BinanceSpotWebSocketClient, symbol: &str) {
    loop {
        match client.subscribe_trades(symbol).await {
            Ok(_) => break,
            Err(WebSocketError::RateLimitExceeded) => {
                // Get stats to understand the situation
                let stats = client.get_rate_limit_stats().await;
                println!("Rate limited. Stats: {:?}", stats);
                
                // Wait and retry
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            Err(e) => {
                eprintln!("Subscribe failed: {}", e);
                break;
            }
        }
    }
}
```

### 6.8 Rate Limit Enforcement Rules

- **MUST** track message rates per venue requirements
- **MUST** emit `RateLimitExceeded` error when limit reached
- **MUST NOT** automatically queue or retry rate-limited requests
- **MUST** expose rate limit status to users
- **MUST** check limits BEFORE operations, not after
- **MUST** maintain separate limiters for public and private clients
- **MUST** reset connection-specific state on disconnect
- **MUST** persist connection rate limits across reconnections
- **MUST** provide actionable error messages with retry guidance

## 7. Testing Requirements

### 7.1 Unit Tests

Each WebSocket implementation **MUST** have tests for:
- Connection establishment
- Graceful disconnection
- Event emission for all event types
- Message serialization/deserialization
- Error handling
- State transitions

### 7.2 Integration Tests

- Test against real WebSocket endpoints (test environment)
- Verify cross-platform compatibility
- Test disconnection scenarios
- Verify event ordering

### 7.3 Mock WebSocket for Testing

Provide mock implementations for testing:

```rust
#[cfg(test)]
pub struct MockWebSocketClient<T: VenueMessage> {
    // Mock implementation for testing
}
```

## 8. Performance Considerations

### 8.1 Message Processing

- **MUST** process messages asynchronously
- **MUST** avoid blocking operations in handlers
- **SHOULD** use zero-copy where possible
- **MUST** handle backpressure appropriately

### 8.2 Memory Management

- **MUST** bound internal buffers
- **MUST** emit events when buffers approach limits
- **MUST NOT** accumulate unbounded message history

## 9. Documentation Requirements

### 9.1 Required Documentation

Each WebSocket implementation **MUST** document:
- Connection lifecycle
- Event types and when they're emitted
- Authentication flow (if applicable)
- Rate limits
- Example usage with reconnection
- Platform-specific considerations

### 9.2 Doc Comment Example

```rust
/// Binance WebSocket client for real-time market data and trading
/// 
/// # Connection Management
/// 
/// This client does NOT automatically reconnect. Users must handle
/// reconnection based on their requirements.
/// 
/// # Example
/// 
/// ```rust
/// let mut client = BinanceWebSocketClient::new("wss://stream.binance.com:9443/ws");
/// client.connect().await?;
/// 
/// // Process events
/// while let Some(event) = client.event_stream().next().await {
///     match event {
///         WebSocketEvent::Disconnected { reason } => {
///             // Handle disconnection - decide whether to reconnect
///         }
///         WebSocketEvent::Message { message } => {
///             // Process message
///         }
///         _ => {}
///     }
/// }
/// ```
pub struct BinanceWebSocketClient { ... }
```

## 10. Common Anti-Patterns to Avoid

**DO NOT**:
- Implement automatic reconnection
- Hide connection state from users
- Silently drop or reorder messages
- Cache messages during disconnection
- Make assumptions about user reconnection preferences
- Implement connection pooling without user control
- Add helper methods that obscure connection state

## 11. Feature Flags

### 11.1 Cargo.toml Configuration

```toml
[features]
default = ["native"]
native = ["tokio-tungstenite", "tokio"]
wasm = ["web-sys", "wasm-bindgen", "wasm-bindgen-futures", "js-sys"]

[dependencies]
# Common
async-trait = "0.1"
futures = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "2.0"

# Native
tokio-tungstenite = { version = "0.26", optional = true, features = ["native-tls"] }
tokio = { version = "1.47", optional = true, features = ["full"] }

# WASM
web-sys = { version = "0.3", optional = true, features = [
    "WebSocket",
    "MessageEvent", 
    "CloseEvent",
    "ErrorEvent",
    "BinaryType"
]}
wasm-bindgen = { version = "0.2", optional = true }
wasm-bindgen-futures = { version = "0.4", optional = true }
js-sys = { version = "0.3", optional = true }
```

## 12. Compliance Checklist

Before implementing a WebSocket client, verify:

- [ ] No automatic reconnection logic
- [ ] All events exposed to users
- [ ] Connection state queryable
- [ ] Platform-specific code properly gated with `cfg`
- [ ] Same API surface for native and WASM
- [ ] Proper error propagation
- [ ] Rate limiting exposed to users
- [ ] Authentication state exposed (for private clients)
- [ ] Documentation includes reconnection examples
- [ ] Tests cover all event types
- [ ] No internal message buffering during disconnection
- [ ] User has full control over connection lifecycle