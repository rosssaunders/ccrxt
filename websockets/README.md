# WebSockets Crate

A platform-agnostic WebSocket client library providing a unified interface for both native Rust and WebAssembly (WASM) targets. This crate is the foundation for all venue WebSocket implementations in CCRXT.

## Overview

This crate provides:

- **Unified WebSocket Trait**: A common `WebSocketConnection` trait that all implementations must follow
- **Platform Abstraction**: Automatic selection of native (tokio-tungstenite) or WASM (web-sys) implementation
- **Composition Pattern**: Venues use trait objects (`Box<dyn WebSocketConnection<T>>`) for platform independence
- **Event-Driven Architecture**: All connection events and messages delivered through a unified event stream
- **No Automatic Reconnection**: Full user control over connection lifecycle and reconnection logic

## Architecture

### Composition Pattern

The key architectural decision is the use of **composition over inheritance**. Venue clients don't implement the WebSocket trait directly; instead, they compose it:

```rust
pub struct BinanceWebSocketClient {
    // Composition: trait object for platform abstraction
    inner: Option<Box<dyn WebSocketConnection<BinanceMessage>>>,
    // Venue-specific state
    subscriptions: HashSet<String>,
}
```

This pattern enables:

1. **Single Codebase**: Write venue code once, run on both native and WASM
2. **Platform Independence**: No `#[cfg]` attributes in venue code
3. **Zero-Cost Abstraction**: Platform selection at compile time
4. **Future Proof**: New platforms can be added without changing venue code

### Platform Selection

```
┌─────────────────────────────────────┐
│         User Application            │
├─────────────────────────────────────┤
│      Venue WebSocket Client         │  (e.g., BinanceWebSocketClient)
├─────────────────────────────────────┤
│        WebSocketBuilder             │  (Detects platform at compile time)
├─────────────────────────────────────┤
│     Platform Implementation         │
├────────────┬────────────────────────┤
│   Native   │        WASM            │
│  (tokio-   │     (web-sys)          │
│tungstenite)│                        │
└────────────┴────────────────────────┘
```

## Core Components

### 1. WebSocketConnection Trait

The main trait that all WebSocket implementations must provide:

```rust
#[async_trait]
pub trait WebSocketConnection<T: VenueMessage> {
    async fn connect(&mut self) -> Result<(), WebSocketError>;
    async fn disconnect(&mut self) -> Result<(), WebSocketError>;
    fn is_connected(&self) -> bool;
    fn connection_state(&self) -> ConnectionState;
    fn event_stream(&mut self) -> Pin<Box<dyn Stream<Item = WebSocketEvent<T>> + Send>>;
    async fn send(&mut self, message: T) -> Result<(), WebSocketError>;
}
```

### 2. Event System

All WebSocket events are delivered through a unified event stream:

```rust
pub enum WebSocketEvent<T: VenueMessage> {
    Connected,
    Disconnected { reason: DisconnectReason },
    Error { error: WebSocketError },
    Message { message: T },
    PingReceived { data: Vec<u8> },
    PongReceived { data: Vec<u8> },
}
```

### 3. Connection States

```rust
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
}
```

### 4. Disconnection Reasons

Detailed disconnection information for proper error handling:

```rust
pub enum DisconnectReason {
    UserInitiated,              // User called disconnect()
    RemoteClosed { code, reason }, // Server closed connection
    NetworkError { details },    // Network failure
    ProtocolError { details },   // Protocol violation
    InvalidMessage { details },  // Deserialization error
}
```

## Usage

### Basic Connection

```rust
use websockets::{WebSocketClientBuilder, WebSocketConnection};

// Create platform-appropriate client
let mut client = WebSocketClientBuilder::new("wss://example.com")
    .header("Authorization", "Bearer token")
    .build::<MyMessage>()?;

// Connect
client.connect().await?;

// Send a message
client.send(MyMessage::Subscribe { channel: "trades" }).await?;

// Process events
let mut events = client.event_stream();
while let Some(event) = events.next().await {
    match event {
        WebSocketEvent::Message { message } => {
            // Handle message
        }
        WebSocketEvent::Disconnected { reason } => {
            // Handle disconnection
            break;
        }
        _ => {}
    }
}
```

### Venue Implementation Pattern

```rust
pub struct MyVenueWebSocketClient {
    // Use composition, not inheritance
    inner: Option<Box<dyn WebSocketConnection<MyMessage>>>,
    url: String,
}

impl MyVenueWebSocketClient {
    pub async fn connect(&mut self) -> Result<(), WebSocketError> {
        // Create platform-appropriate client
        let client = WebSocketClientBuilder::new(&self.url)
            .build::<MyMessage>()?;

        // Store as trait object
        self.inner = Some(Box::new(client));

        // Delegate to inner
        if let Some(inner) = &mut self.inner {
            inner.connect().await?;
        }

        Ok(())
    }
}
```

## Connection Management

### No Automatic Reconnection

This library does NOT automatically reconnect. This is by design:

1. **Visibility**: Users need to know when disconnections happen
2. **Control**: Different applications have different reconnection requirements
3. **Context**: Some disconnections shouldn't trigger reconnection
4. **State**: Users may need to re-subscribe or re-authenticate after reconnection

### User-Controlled Reconnection Example

```rust
async fn maintain_connection(client: &mut MyWebSocketClient) {
    let mut backoff = Duration::from_secs(1);

    loop {
        match client.connect().await {
            Ok(_) => {
                backoff = Duration::from_secs(1); // Reset backoff

                // Process events until disconnection
                while let Some(event) = client.event_stream().next().await {
                    match event {
                        WebSocketEvent::Disconnected { reason } => {
                            // Decide whether to reconnect
                            if should_reconnect(&reason) {
                                break; // Will reconnect
                            } else {
                                return; // Exit
                            }
                        }
                        WebSocketEvent::Message { message } => {
                            handle_message(message);
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }

        // Wait before reconnecting
        tokio::time::sleep(backoff).await;
        backoff = (backoff * 2).min(Duration::from_secs(60));
    }
}
```

## Rate Limiting

While the base WebSocket crate doesn't enforce rate limiting, venue implementations should implement appropriate rate limiting for:

- **Message Rate**: Limit messages per second (e.g., 5 msg/sec for Binance)
- **Subscription Limits**: Maximum concurrent subscriptions (e.g., 1024 for Binance)
- **Connection Rate**: Limit connection attempts per time window

See venue-specific implementations for examples.

## Platform-Specific Features

### Native (tokio-tungstenite)

- Full async/await support with tokio runtime
- TLS support through native-tls or rustls
- Custom headers in WebSocket handshake
- Efficient message handling with zero-copy where possible

### WASM (web-sys)

- Browser WebSocket API integration
- Automatic handling of browser connection limits
- Event-driven callbacks through JavaScript interop
- Compatible with wasm-bindgen and wasm-pack

## Features

```toml
[features]
default = ["native"]
native = ["tokio-tungstenite", "tokio"]
wasm = ["web-sys", "wasm-bindgen", "wasm-bindgen-futures"]
```

## Examples

See the `examples/` directory for:

- `connection_management.rs` - Demonstrates proper connection lifecycle management

## Design Principles

1. **Transparency**: All connection events visible to users
2. **User Control**: No automatic behaviors; users control everything
3. **Platform Parity**: Same API across native and WASM
4. **Pure Wrappers**: No fix-up logic or hidden behaviors
5. **Event-Driven**: All state changes communicated via events
6. **Composition**: Use trait objects for platform abstraction

## Testing

```bash
# Run tests
cargo test

# Run native example
cargo run --example connection_management

# Build for WASM
cargo build --target wasm32-unknown-unknown --features wasm --no-default-features
```

## Dependencies

- `async-trait` - Async trait support
- `futures` - Stream and async utilities
- `serde` - Serialization/deserialization
- `thiserror` - Error handling
- `tokio-tungstenite` (native) - Native WebSocket implementation
- `web-sys` (wasm) - Browser WebSocket API bindings

## License

See repository LICENSE file.
