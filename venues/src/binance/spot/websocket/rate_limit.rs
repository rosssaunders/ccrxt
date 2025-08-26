//! WebSocket rate limiting for Binance Spot
//!
//! Binance WebSocket limits:
//! - 5 messages per second per connection
//! - 1024 concurrent streams per connection
//! - 300 connections per 5 minutes per IP

use std::collections::{HashSet, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};

use thiserror::Error;
use tokio::sync::RwLock;

/// WebSocket rate limit errors
#[derive(Debug, Error)]
pub enum RateLimitError {
    #[error("Message rate exceeded: {limit} messages/sec")]
    MessageRateExceeded {
        limit: u32,

        retry_after: Duration,
    },

    #[error("Subscription limit exceeded: {current}/{limit} subscriptions")]
    SubscriptionLimitExceeded {
        current: u32,

        limit: u32,
    },

    #[error("Connection rate exceeded: {limit} per {window:?}")]
    ConnectionRateExceeded {
        limit: u32,

        window: Duration,

        retry_after: Duration,
    },
}

/// Configuration for WebSocket rate limits
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum messages per second
    pub max_messages_per_second: u32,

    /// Maximum concurrent subscriptions
    pub max_subscriptions: u32,

    /// Maximum connections per window
    pub max_connections_per_window: u32,

    /// Connection rate limit window
    pub connection_window: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_messages_per_second: 5,
            max_subscriptions: 1024,
            max_connections_per_window: 300,
            connection_window: Duration::from_secs(300), // 5 minutes
        }
    }
}

/// Message rate limiter - tracks messages sent per second
#[derive(Debug)]
struct MessageRateLimiter {
    /// Timestamps of recent messages
    timestamps: VecDeque<Instant>,

    /// Maximum messages per second
    max_per_second: u32,
}

impl MessageRateLimiter {
    fn new(max_per_second: u32) -> Self {
        Self {
            timestamps: VecDeque::new(),
            max_per_second,
        }
    }

    /// Check if a message can be sent and record it
    fn check_and_record(&mut self) -> Result<(), RateLimitError> {
        let now = Instant::now();
        let cutoff = now - Duration::from_secs(1);

        // Remove timestamps older than 1 second
        while let Some(&front) = self.timestamps.front() {
            if front < cutoff {
                self.timestamps.pop_front();
            } else {
                break;
            }
        }

        // Check if we're at the limit
        if self.timestamps.len() >= self.max_per_second as usize {
            // Calculate when we can retry
            let oldest = self.timestamps.front().unwrap();
            let retry_after = Duration::from_secs(1) - now.duration_since(*oldest);

            return Err(RateLimitError::MessageRateExceeded {
                limit: self.max_per_second,
                retry_after,
            });
        }

        // Record this message
        self.timestamps.push_back(now);
        Ok(())
    }

    /// Reset the rate limiter
    fn reset(&mut self) {
        self.timestamps.clear();
    }
}

/// Subscription limiter - tracks active subscriptions
#[derive(Debug)]
struct SubscriptionLimiter {
    /// Active subscription IDs
    active: HashSet<String>,

    /// Maximum subscriptions
    max_subscriptions: u32,
}

impl SubscriptionLimiter {
    fn new(max_subscriptions: u32) -> Self {
        Self {
            active: HashSet::new(),
            max_subscriptions,
        }
    }

    /// Check if a subscription can be added
    fn check(&self, stream_id: &str) -> Result<(), RateLimitError> {
        // Already subscribed is OK
        if self.active.contains(stream_id) {
            return Ok(());
        }

        // Check limit
        if self.active.len() >= self.max_subscriptions as usize {
            return Err(RateLimitError::SubscriptionLimitExceeded {
                current: self.active.len() as u32,
                limit: self.max_subscriptions,
            });
        }

        Ok(())
    }

    /// Add a subscription
    fn add(&mut self, stream_id: String) {
        self.active.insert(stream_id);
    }

    /// Remove a subscription
    fn remove(&mut self, stream_id: &str) -> bool {
        self.active.remove(stream_id)
    }

    /// Clear all subscriptions
    fn clear(&mut self) {
        self.active.clear();
    }

    /// Get current subscription count
    fn count(&self) -> u32 {
        self.active.len() as u32
    }
}

/// Connection rate limiter - tracks connection attempts
#[derive(Debug)]
struct ConnectionLimiter {
    /// Connection attempt timestamps
    attempts: VecDeque<Instant>,

    /// Maximum connections per window
    max_per_window: u32,

    /// Time window for connection limiting
    window: Duration,
}

impl ConnectionLimiter {
    fn new(max_per_window: u32, window: Duration) -> Self {
        Self {
            attempts: VecDeque::new(),
            max_per_window,
            window,
        }
    }

    /// Check if a connection can be made and record it
    fn check_and_record(&mut self) -> Result<(), RateLimitError> {
        let now = Instant::now();
        let cutoff = now - self.window;

        // Remove attempts outside the window
        while let Some(&front) = self.attempts.front() {
            if front < cutoff {
                self.attempts.pop_front();
            } else {
                break;
            }
        }

        // Check if we're at the limit
        if self.attempts.len() >= self.max_per_window as usize {
            // Calculate when we can retry
            let oldest = self.attempts.front().unwrap();
            let retry_after = self.window - now.duration_since(*oldest);

            return Err(RateLimitError::ConnectionRateExceeded {
                limit: self.max_per_window,
                window: self.window,
                retry_after,
            });
        }

        // Record this attempt
        self.attempts.push_back(now);
        Ok(())
    }

    /// Get current connection count in window
    fn count(&self) -> u32 {
        self.attempts.len() as u32
    }
}

/// Rate limit usage statistics
#[derive(Debug, Clone)]
pub struct RateLimitStats {
    /// Messages sent in the last second
    pub messages_in_last_second: u32,

    /// Maximum messages per second
    pub max_messages_per_second: u32,

    /// Active subscriptions
    pub active_subscriptions: u32,

    /// Maximum subscriptions
    pub max_subscriptions: u32,

    /// Connection attempts in current window
    pub connections_in_window: u32,

    /// Maximum connections per window
    pub max_connections_per_window: u32,

    /// Connection window duration
    pub connection_window: Duration,
}

/// WebSocket rate limiter for Binance
pub struct WebSocketRateLimiter {
    /// Message rate limiter
    message_limiter: Arc<RwLock<MessageRateLimiter>>,

    /// Subscription limiter
    subscription_limiter: Arc<RwLock<SubscriptionLimiter>>,

    /// Connection limiter
    connection_limiter: Arc<RwLock<ConnectionLimiter>>,

    /// Configuration
    config: RateLimitConfig,
}

impl WebSocketRateLimiter {
    /// Create a new rate limiter with default Binance limits
    pub fn new() -> Self {
        Self::with_config(RateLimitConfig::default())
    }

    /// Create a new rate limiter with custom configuration
    pub fn with_config(config: RateLimitConfig) -> Self {
        Self {
            message_limiter: Arc::new(RwLock::new(MessageRateLimiter::new(
                config.max_messages_per_second,
            ))),
            subscription_limiter: Arc::new(RwLock::new(SubscriptionLimiter::new(
                config.max_subscriptions,
            ))),
            connection_limiter: Arc::new(RwLock::new(ConnectionLimiter::new(
                config.max_connections_per_window,
                config.connection_window,
            ))),
            config,
        }
    }

    /// Check and record a message send
    pub async fn check_message(&self) -> Result<(), RateLimitError> {
        let mut limiter = self.message_limiter.write().await;
        limiter.check_and_record()
    }

    /// Check if a subscription can be added
    pub async fn check_subscription(&self, stream_id: &str) -> Result<(), RateLimitError> {
        let limiter = self.subscription_limiter.read().await;
        limiter.check(stream_id)
    }

    /// Add a subscription
    pub async fn add_subscription(&self, stream_id: String) {
        let mut limiter = self.subscription_limiter.write().await;
        limiter.add(stream_id);
    }

    /// Remove a subscription
    pub async fn remove_subscription(&self, stream_id: &str) -> bool {
        let mut limiter = self.subscription_limiter.write().await;
        limiter.remove(stream_id)
    }

    /// Clear all subscriptions (on disconnect)
    pub async fn clear_subscriptions(&self) {
        let mut limiter = self.subscription_limiter.write().await;
        limiter.clear();
    }

    /// Check and record a connection attempt
    pub async fn check_connection(&self) -> Result<(), RateLimitError> {
        let mut limiter = self.connection_limiter.write().await;
        limiter.check_and_record()
    }

    /// Reset connection-specific state (on disconnect)
    pub async fn reset_connection_state(&self) {
        // Clear messages and subscriptions, but not connection attempts
        let mut message_limiter = self.message_limiter.write().await;
        message_limiter.reset();

        let mut subscription_limiter = self.subscription_limiter.write().await;
        subscription_limiter.clear();
    }

    /// Get current rate limit statistics
    pub async fn get_stats(&self) -> RateLimitStats {
        let message_limiter = self.message_limiter.read().await;
        let subscription_limiter = self.subscription_limiter.read().await;
        let connection_limiter = self.connection_limiter.read().await;

        RateLimitStats {
            messages_in_last_second: message_limiter.timestamps.len() as u32,
            max_messages_per_second: self.config.max_messages_per_second,
            active_subscriptions: subscription_limiter.count(),
            max_subscriptions: self.config.max_subscriptions,
            connections_in_window: connection_limiter.count(),
            max_connections_per_window: self.config.max_connections_per_window,
            connection_window: self.config.connection_window,
        }
    }
}

impl Default for WebSocketRateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_message_rate_limiting() {
        let limiter = WebSocketRateLimiter::new();

        // Should allow 5 messages
        for _ in 0..5 {
            assert!(limiter.check_message().await.is_ok());
        }

        // 6th message should fail
        assert!(matches!(
            limiter.check_message().await,
            Err(RateLimitError::MessageRateExceeded { .. })
        ));
    }

    #[tokio::test]
    async fn test_subscription_limiting() {
        let config = RateLimitConfig {
            max_subscriptions: 3,
            ..Default::default()
        };
        let limiter = WebSocketRateLimiter::with_config(config);

        // Add 3 subscriptions
        for i in 0..3 {
            let stream_id = format!("stream_{}", i);
            assert!(limiter.check_subscription(&stream_id).await.is_ok());
            limiter.add_subscription(stream_id).await;
        }

        // 4th subscription should fail
        assert!(matches!(
            limiter.check_subscription("stream_4").await,
            Err(RateLimitError::SubscriptionLimitExceeded { .. })
        ));

        // Removing one should allow adding another
        assert!(limiter.remove_subscription("stream_0").await);
        assert!(limiter.check_subscription("stream_4").await.is_ok());
    }

    #[tokio::test]
    async fn test_connection_limiting() {
        let config = RateLimitConfig {
            max_connections_per_window: 2,
            connection_window: Duration::from_secs(1),
            ..Default::default()
        };
        let limiter = WebSocketRateLimiter::with_config(config);

        // Should allow 2 connections
        assert!(limiter.check_connection().await.is_ok());
        assert!(limiter.check_connection().await.is_ok());

        // 3rd connection should fail
        assert!(matches!(
            limiter.check_connection().await,
            Err(RateLimitError::ConnectionRateExceeded { .. })
        ));
    }

    #[tokio::test]
    async fn test_reset_connection_state() {
        let limiter = WebSocketRateLimiter::new();

        // Add some state
        limiter.check_message().await.unwrap();
        limiter.add_subscription("test_stream".to_string()).await;

        // Get initial stats
        let stats = limiter.get_stats().await;
        assert_eq!(stats.messages_in_last_second, 1);
        assert_eq!(stats.active_subscriptions, 1);

        // Reset connection state
        limiter.reset_connection_state().await;

        // Verify state was cleared
        let stats = limiter.get_stats().await;
        assert_eq!(stats.messages_in_last_second, 0);
        assert_eq!(stats.active_subscriptions, 0);
    }

    #[tokio::test]
    async fn test_duplicate_subscription_allowed() {
        let limiter = WebSocketRateLimiter::new();

        // Add a subscription
        let stream_id = "btcusdt@trade";
        assert!(limiter.check_subscription(stream_id).await.is_ok());
        limiter.add_subscription(stream_id.to_string()).await;

        // Checking the same subscription again should be OK
        assert!(limiter.check_subscription(stream_id).await.is_ok());
    }
}