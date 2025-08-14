/// Rate limiting compatibility module for WASM and native platforms
/// 
/// This module provides a unified interface for rate limiting that works
/// both in native environments (using governor) and WASM contexts.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::time_compat::{Duration, Instant};

/// A quota specification for rate limiting
#[derive(Debug, Clone, Copy)]
pub struct Quota {
    /// Maximum number of requests allowed
    pub max_burst: u32,
    /// Time period for the quota
    pub period: Duration,
}

impl Quota {
    /// Create a quota with a specific period
    pub fn with_period(period: Duration) -> Option<Self> {
        Some(Self {
            max_burst: 1,
            period,
        })
    }

    /// Set the burst size (max requests allowed)
    pub fn allow_burst(mut self, burst: std::num::NonZeroU32) -> Self {
        self.max_burst = burst.get();
        self
    }

    /// Create a per-second quota
    pub fn per_second(burst: std::num::NonZeroU32) -> Self {
        Self {
            max_burst: burst.get(),
            period: Duration::from_secs(1),
        }
    }
}

/// Token bucket for rate limiting
#[derive(Debug, Clone)]
struct TokenBucket {
    /// Maximum tokens (burst size)
    max_tokens: u32,
    /// Current tokens available
    tokens: f64,
    /// Last refill time
    last_refill: Instant,
    /// Refill rate (tokens per second)
    refill_rate: f64,
}

impl TokenBucket {
    fn new(quota: Quota) -> Self {
        let refill_rate = quota.max_burst as f64 / quota.period.as_secs_f64();
        Self {
            max_tokens: quota.max_burst,
            tokens: quota.max_burst as f64,
            last_refill: Instant::now(),
            refill_rate,
        }
    }

    fn try_consume(&mut self, tokens: u32) -> bool {
        self.refill();
        
        if self.tokens >= tokens as f64 {
            self.tokens -= tokens as f64;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.max_tokens as f64);
        self.last_refill = now;
    }
}

/// Rate limiter implementation that works in both native and WASM
#[derive(Debug)]
pub struct RateLimiter<K> {
    buckets: Arc<RwLock<HashMap<K, TokenBucket>>>,
    quota: Quota,
}

impl<K: Eq + std::hash::Hash + Clone> RateLimiter<K> {
    /// Create a new keyed rate limiter
    pub fn keyed(quota: Quota) -> Self {
        Self {
            buckets: Arc::new(RwLock::new(HashMap::new())),
            quota,
        }
    }

    /// Check if a key can proceed (consumes one token)
    pub async fn check_key(&self, key: &K) -> Result<(), ()> {
        self.check_key_n(key, 1).await
    }

    /// Check if a key can proceed (consumes n tokens)
    pub async fn check_key_n(&self, key: &K, tokens: u32) -> Result<(), ()> {
        let mut buckets = self.buckets.write().await;
        
        let bucket = buckets.entry(key.clone())
            .or_insert_with(|| TokenBucket::new(self.quota));
        
        if bucket.try_consume(tokens) {
            Ok(())
        } else {
            Err(())
        }
    }

    /// Clean up old entries (optional, for memory management)
    pub async fn cleanup_stale_entries(&self, older_than: Duration) {
        let mut buckets = self.buckets.write().await;
        let now = Instant::now();
        
        buckets.retain(|_, bucket| {
            now.duration_since(bucket.last_refill) < older_than
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroU32;

    #[tokio::test]
    async fn test_rate_limiter_basic() {
        let quota = Quota::with_period(Duration::from_secs(1))
            .unwrap()
            .allow_burst(NonZeroU32::new(2).unwrap());
        
        let limiter = RateLimiter::<String>::keyed(quota);
        let key = "test_key".to_string();
        
        // First two requests should succeed
        assert!(limiter.check_key(&key).await.is_ok());
        assert!(limiter.check_key(&key).await.is_ok());
        
        // Third request should fail (exceeded burst)
        assert!(limiter.check_key(&key).await.is_err());
    }

    #[tokio::test]
    async fn test_rate_limiter_multiple_keys() {
        let quota = Quota::with_period(Duration::from_secs(1))
            .unwrap()
            .allow_burst(NonZeroU32::new(1).unwrap());
        
        let limiter = RateLimiter::<String>::keyed(quota);
        let key1 = "key1".to_string();
        let key2 = "key2".to_string();
        
        // Each key should have its own quota
        assert!(limiter.check_key(&key1).await.is_ok());
        assert!(limiter.check_key(&key2).await.is_ok());
        
        // Both keys should be rate limited after one use
        assert!(limiter.check_key(&key1).await.is_err());
        assert!(limiter.check_key(&key2).await.is_err());
    }

    #[tokio::test]
    async fn test_rate_limiter_refill() {
        let quota = Quota::with_period(Duration::from_millis(100))
            .unwrap()
            .allow_burst(NonZeroU32::new(1).unwrap());
        
        let limiter = RateLimiter::<String>::keyed(quota);
        let key = "test_key".to_string();
        
        // First request should succeed
        assert!(limiter.check_key(&key).await.is_ok());
        
        // Second request should fail immediately
        assert!(limiter.check_key(&key).await.is_err());
        
        // Wait for refill
        tokio::time::sleep(Duration::from_millis(150)).await;
        
        // Should succeed after refill
        assert!(limiter.check_key(&key).await.is_ok());
    }
}

