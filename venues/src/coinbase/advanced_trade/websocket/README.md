# Coinbase Advanced Trade WebSocket

This module provides a high-performance WebSocket client for the Coinbase Advanced Trade API. It implements all public market data streams with a focus on low latency and reliability.

## Features

- Pure Rust implementation for maximum performance
- Support for all public market data channels
- Automatic reconnection handling
- Rate limit compliance
- Type-safe message handling

## Available Channels

- Level 2 Order Book (`level2`)
- Ticker (`ticker`)
- Ticker Batch (`ticker_batch`)
- Market Trades (`market_trades`)
- Candles (`candles`)
- Status (`status`)
- Heartbeats (`heartbeats`)

## Usage Examples

### Basic Connection and Order Book Subscription

```rust
use venues::coinbase::advanced_trade::websocket::public::CoinbaseAdvancedTradeWebSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new websocket instance
    let mut ws = CoinbaseAdvancedTradeWebSocket::new();
    
    // Connect to the websocket
    ws.connect().await?;
    
    // Subscribe to order book for BTC-USD
    ws.subscribe_orderbook(vec!["BTC-USD".to_string()]).await?;
    
    // Get the message stream
    let mut stream = ws.message_stream();
    
    // Process incoming messages
    while let Some(message) = stream.next().await {
        match message {
            Ok(msg) => println!("Received: {:?}", msg),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
    
    Ok(())
}
```

### Multiple Channel Subscription

```rust
use venues::coinbase::advanced_trade::websocket::public::CoinbaseAdvancedTradeWebSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ws = CoinbaseAdvancedTradeWebSocket::new();
    ws.connect().await?;
    
    let product_ids = vec!["BTC-USD".to_string(), "ETH-USD".to_string()];
    
    // Subscribe to multiple channels
    ws.subscribe_orderbook(product_ids.clone()).await?;
    ws.subscribe_ticker(product_ids.clone()).await?;
    ws.subscribe_market_trades(product_ids.clone()).await?;
    
    // Subscribe to heartbeats (no product IDs needed)
    ws.subscribe_heartbeats().await?;
    
    let mut stream = ws.message_stream();
    
    while let Some(message) = stream.next().await {
        match message {
            Ok(msg) => println!("Received: {:?}", msg),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
    
    Ok(())
}
```

### Unsubscribing and Cleanup

```rust
use venues::coinbase::advanced_trade::websocket::public::CoinbaseAdvancedTradeWebSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ws = CoinbaseAdvancedTradeWebSocket::new();
    ws.connect().await?;
    
    let product_ids = vec!["BTC-USD".to_string()];
    
    // Subscribe to order book
    ws.subscribe_orderbook(product_ids.clone()).await?;
    
    // Process messages for a while...
    
    // Unsubscribe from order book
    ws.unsubscribe_orderbook(product_ids.clone()).await?;
    
    // Clean up the connection
    ws.disconnect().await?;
    
    Ok(())
}
```

## Performance Considerations

1. The implementation is designed for high-frequency trading scenarios
2. All message handling is done asynchronously
3. The websocket connection is maintained with automatic reconnection
4. Rate limits are strictly enforced to prevent disconnections

## Error Handling

The websocket client converts all Coinbase API errors into native Rust errors. Common error types include:

- Connection errors
- Authentication errors
- Rate limit errors
- Message parsing errors

## Rate Limits

The websocket client automatically handles rate limits according to Coinbase's specifications:

- Maximum of 5 subscriptions per second
- Maximum of 50 subscriptions per connection
- Heartbeat messages every 30 seconds

## Best Practices

1. Always implement proper error handling
2. Use heartbeats to monitor connection health
3. Implement reconnection logic for production use
4. Monitor rate limits to prevent disconnections
5. Clean up subscriptions when done
6. Use appropriate buffer sizes for your use case 