# WebSocket Abstraction Layer

## Purpose

Enhance the websockets crate to provide a platform-agnostic WebSocket client abstraction. Native implementation is available now; WASM can be added later if needed.

## Current State

- `WebSocketClient` trait implemented
- Native implementation using `tokio-tungstenite`
- Connection and message correlation utilities available

## Required Enhancements

### 1. Platform Abstraction

- `WebSocketClient` trait similar to `HttpClient` (done)
- Native implementation using `tokio-tungstenite` (done)
- WASM implementation (deferred; optional future work)

### 2. Connection Management

- Connection state tracking (Connected, Disconnected, Connecting, Error)
- Automatic reconnection with exponential backoff
- Connection events for host code to handle data gaps
- Graceful shutdown and cleanup

### 3. Rate Limiting Framework

- **Rate Limiter Trait**: Generic `WebSocketRateLimiter` trait for all venues
- **Venue-Specific Implementations**: Each venue implements custom rate limiting
- **Rate Limiting Types**:
  - Connection limits (per IP, per UID, per product)
  - Request rate limits (per second, per minute, per hour)  
  - Subscription limits (per connection)
- **Algorithms**: Token bucket, sliding window, leaky bucket support
- **Features**:
  - Burst capacity handling
  - Request queuing when at capacity
  - Exponential backoff on violations
  - Rate limit event emission for monitoring
  - Connection cleanup and tracking

### 3. Message Handling

- Request/response correlation with unique IDs
- Message routing and dispatch
- Timeout handling for requests
- Error propagation and handling

### 4. Subscription Management

- Track active subscriptions for reconnection
- Subscription state persistence
- Batch subscribe/unsubscribe operations

## Files to Create/Modify

- `src/lib.rs` - Export new traits and types
- `src/client.rs` - Main WebSocket client trait
- `src/native.rs` - Native implementation with tokio-tungstenite
- `src/connection.rs` - Connection management utilities
- `src/message.rs` - Message correlation and routing

## Design Constraints

- Native-first; WASM support may be added later
- Host code controls all connection lifecycle decisions
- No automatic reconnection without explicit host approval
- Thread-safe for native
- Minimal dependencies to avoid conflicts with venues
