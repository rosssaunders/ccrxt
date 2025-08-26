use std::{
    collections::HashSet,
    pin::Pin,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};

use futures::Stream;
use tokio::{sync::RwLock, time::interval};
use websockets::{
    builder::WebSocketClientBuilder, ConnectionState, WebSocketConnection, WebSocketError,
    WebSocketEvent,
};

use super::{
    enums::StreamType,
    message::{BinanceMessage, BinanceRequest},
    rate_limit::{RateLimitError, RateLimitStats, WebSocketRateLimiter},
};

/// Binance Spot public WebSocket client
/// 
/// Provides access to Binance's public market data streams.
/// 
/// # Connection Management
/// 
/// This client does NOT automatically reconnect. Users must handle
/// reconnection based on their requirements. Binance WebSocket connections
/// are valid for 24 hours and require periodic pings.
/// 
/// # Example
/// 
/// ```ignore
/// let mut client = BinanceSpotWebSocketClient::new();
/// client.connect().await?;
/// 
/// // Subscribe to BTC/USDT trades
/// client.subscribe_trades("BTCUSDT").await?;
/// 
/// // Process events
/// while let Some(event) = client.event_stream().next().await {
///     match event {
///         WebSocketEvent::Message { message } => {
///             // Handle Binance message
///         }
///         WebSocketEvent::Disconnected { reason } => {
///             // Handle disconnection
///             break;
///         }
///         _ => {}
///     }
/// }
/// ```
pub struct BinanceSpotWebSocketClient {
    /// Inner WebSocket connection
    inner: Option<Box<dyn WebSocketConnection<BinanceMessage>>>,
    /// WebSocket endpoint URL
    url: String,
    /// Request ID counter
    request_id: Arc<AtomicU64>,
    /// Active subscriptions
    subscriptions: HashSet<String>,
    /// Ping interval handle
    ping_task: Option<tokio::task::JoinHandle<()>>,
    /// Rate limiter for WebSocket operations
    rate_limiter: Arc<RwLock<WebSocketRateLimiter>>,
}

impl BinanceSpotWebSocketClient {
    /// Create a new Binance Spot WebSocket client
    pub fn new() -> Self {
        Self::with_url("wss://stream.binance.com:9443/ws")
    }
    
    /// Create a new client with a custom URL (e.g., for testnet)
    pub fn with_url(url: impl Into<String>) -> Self {
        Self {
            inner: None,
            url: url.into(),
            request_id: Arc::new(AtomicU64::new(1)),
            subscriptions: HashSet::new(),
            ping_task: None,
            rate_limiter: Arc::new(RwLock::new(WebSocketRateLimiter::new())),
        }
    }
    
    /// Connect to Binance WebSocket
    pub async fn connect(&mut self) -> Result<(), WebSocketError> {
        // Check connection rate limit
        {
            let limiter = self.rate_limiter.write().await;
            limiter.check_connection().await
                .map_err(|e| match e {
                    RateLimitError::ConnectionRateExceeded { retry_after, .. } => {
                        WebSocketError::RateLimitExceeded(format!(
                            "Connection rate limit exceeded. Retry after {:?}",
                            retry_after
                        ))
                    }
                    _ => WebSocketError::RateLimitExceeded(e.to_string()),
                })?;
        }
        
        // Create WebSocket client
        let client = WebSocketClientBuilder::new(&self.url)
            .build::<BinanceMessage>()
            .map_err(|e| WebSocketError::ConnectionFailed(e.to_string()))?;
        
        self.inner = Some(Box::new(client));
        
        // Connect
        if let Some(inner) = &mut self.inner {
            inner.connect().await?;
            
            // Start ping task to keep connection alive (Binance requires ping every 20 seconds)
            self.start_ping_task();
        }
        
        Ok(())
    }
    
    /// Disconnect from Binance WebSocket
    pub async fn disconnect(&mut self) -> Result<(), WebSocketError> {
        // Stop ping task
        if let Some(task) = self.ping_task.take() {
            task.abort();
        }
        
        // Clear subscriptions
        self.subscriptions.clear();
        
        // Reset rate limiter connection state
        {
            let limiter = self.rate_limiter.write().await;
            limiter.reset_connection_state().await;
        }
        
        // Disconnect
        if let Some(inner) = &mut self.inner {
            inner.disconnect().await?;
        }
        
        Ok(())
    }
    
    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.inner
            .as_ref()
            .map(|i| i.is_connected())
            .unwrap_or(false)
    }
    
    /// Get connection state
    pub fn connection_state(&self) -> ConnectionState {
        self.inner
            .as_ref()
            .map(|i| i.connection_state())
            .unwrap_or(ConnectionState::Disconnected)
    }
    
    /// Get event stream
    pub fn event_stream(&mut self) -> Pin<Box<dyn Stream<Item = WebSocketEvent<BinanceMessage>> + Send>> {
        if let Some(inner) = &mut self.inner {
            inner.event_stream()
        } else {
            Box::pin(futures::stream::empty())
        }
    }
    
    /// Get the next request ID
    fn next_request_id(&self) -> u64 {
        self.request_id.fetch_add(1, Ordering::SeqCst)
    }
    
    /// Start ping task to keep connection alive
    fn start_ping_task(&mut self) {
        if let Some(inner) = &self.inner {
            if inner.is_connected() {
                let ping_task = tokio::spawn(async move {
                    let mut ticker = interval(Duration::from_secs(20));
                    loop {
                        ticker.tick().await;
                        // In a real implementation, we'd send a ping frame here
                        // For now, this is a placeholder
                    }
                });
                self.ping_task = Some(ping_task);
            }
        }
    }
    
    /// Build a stream name (utility method for subscription modules)
    pub(super) fn build_stream_name(symbol: &str, stream_type: StreamType, params: Option<&str>) -> String {
        let symbol_lower = symbol.to_lowercase();
        match params {
            Some(p) => format!("{}@{}_{}", symbol_lower, stream_type.to_stream_name(), p),
            None => format!("{}@{}", symbol_lower, stream_type.to_stream_name()),
        }
    }
    
    /// Subscribe to multiple streams (internal use only)
    pub(super) async fn subscribe(&mut self, streams: &[String]) -> Result<(), WebSocketError> {
        if !self.is_connected() {
            return Err(WebSocketError::NotConnected);
        }
        
        // Check subscription limits for each stream
        {
            let limiter = self.rate_limiter.read().await;
            for stream in streams {
                limiter.check_subscription(stream).await
                    .map_err(|e| match e {
                        RateLimitError::SubscriptionLimitExceeded { current, limit } => {
                            WebSocketError::RateLimitExceeded(format!(
                                "Subscription limit exceeded: {}/{}",
                                current, limit
                            ))
                        }
                        _ => WebSocketError::RateLimitExceeded(e.to_string()),
                    })?;
            }
        }
        
        // Check message rate limit
        {
            let limiter = self.rate_limiter.write().await;
            limiter.check_message().await
                .map_err(|e| match e {
                    RateLimitError::MessageRateExceeded { retry_after, .. } => {
                        WebSocketError::RateLimitExceeded(format!(
                            "Message rate limit exceeded. Retry after {:?}",
                            retry_after
                        ))
                    }
                    _ => WebSocketError::RateLimitExceeded(e.to_string()),
                })?;
        }
        
        let request = BinanceRequest {
            method: "SUBSCRIBE".to_string(),
            params: streams.to_vec(),
            id: self.next_request_id(),
        };
        
        // Send subscription request
        if let Some(inner) = &mut self.inner {
            inner.send(BinanceMessage::Request(request)).await?;
            
            // Track subscriptions in both client and rate limiter
            let limiter = self.rate_limiter.write().await;
            for stream in streams {
                self.subscriptions.insert(stream.clone());
                limiter.add_subscription(stream.clone()).await;
            }
        }
        
        Ok(())
    }
    
    /// Unsubscribe from multiple streams (internal use only)
    pub(super) async fn unsubscribe(&mut self, streams: &[String]) -> Result<(), WebSocketError> {
        if !self.is_connected() {
            return Err(WebSocketError::NotConnected);
        }
        
        // Check message rate limit
        {
            let limiter = self.rate_limiter.write().await;
            limiter.check_message().await
                .map_err(|e| match e {
                    RateLimitError::MessageRateExceeded { retry_after, .. } => {
                        WebSocketError::RateLimitExceeded(format!(
                            "Message rate limit exceeded. Retry after {:?}",
                            retry_after
                        ))
                    }
                    _ => WebSocketError::RateLimitExceeded(e.to_string()),
                })?;
        }
        
        let request = BinanceRequest {
            method: "UNSUBSCRIBE".to_string(),
            params: streams.to_vec(),
            id: self.next_request_id(),
        };
        
        // Send unsubscription request
        if let Some(inner) = &mut self.inner {
            inner.send(BinanceMessage::Request(request)).await?;
            
            // Remove from tracked subscriptions in both client and rate limiter
            let limiter = self.rate_limiter.write().await;
            for stream in streams {
                self.subscriptions.remove(stream);
                limiter.remove_subscription(stream).await;
            }
        }
        
        Ok(())
    }
    
    /// Get current subscriptions
    pub fn subscriptions(&self) -> &HashSet<String> {
        &self.subscriptions
    }
    
    /// Get rate limit statistics
    pub async fn get_rate_limit_stats(&self) -> RateLimitStats {
        let limiter = self.rate_limiter.read().await;
        limiter.get_stats().await
    }
}

impl Default for BinanceSpotWebSocketClient {
    fn default() -> Self {
        Self::new()
    }
}