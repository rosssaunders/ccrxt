use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};
use async_trait::async_trait;

/// Rate limiter for Binance Spot API
pub struct BinanceSpotRateLimiter {
    // Weight-based rate limits
    weight_limits: Mutex<HashMap<String, (u32, Instant)>>,
    // IP-based rate limits
    ip_limits: Mutex<HashMap<String, (u32, Instant)>>,
    // Track remaining requests
    remaining: AtomicU32,
}

impl BinanceSpotRateLimiter {
    pub fn new() -> Self {
        Self {
            weight_limits: Mutex::new(HashMap::new()),
            ip_limits: Mutex::new(HashMap::new()),
            remaining: AtomicU32::new(1200), // Default limit
        }
    }

    /// Check if a request can be made based on weight limits
    pub async fn check_weight_limit(&self, endpoint: &str, weight: u32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut limits = self.weight_limits.lock().await;
        let now = Instant::now();
        
        // Clean up old entries
        limits.retain(|_, (_, time)| now.duration_since(*time) < Duration::from_secs(60));
        
        // Check if we're over the limit
        if let Some((current_weight, _)) = limits.get(endpoint) {
            if *current_weight + weight > 1200 {
                return Err("Rate limit exceeded".into());
            }
        }
        
        // Update the weight
        limits.insert(endpoint.to_string(), (weight, now));
        self.remaining.store(1200 - weight, Ordering::Relaxed);
        Ok(())
    }

    /// Check if a request can be made based on IP limits
    pub async fn check_ip_limit(&self, ip: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut limits = self.ip_limits.lock().await;
        let now = Instant::now();
        
        // Clean up old entries
        limits.retain(|_, (_, time)| now.duration_since(*time) < Duration::from_secs(60));
        
        // Check if we're over the limit
        if let Some((count, _)) = limits.get(ip) {
            if *count >= 1200 {
                return Err("Rate limit exceeded".into());
            }
        }
        
        // Update the count
        limits.insert(ip.to_string(), (1, now));
        Ok(())
    }
}