use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::sync::Mutex;
use crate::rate_limit::RateLimiter;

pub struct BinanceCoinMRateLimiter {
    // Weight-based rate limits
    weight_limits: Mutex<HashMap<String, (u32, Instant)>>,
    // IP-based rate limits
    ip_limits: Mutex<HashMap<String, (u32, Instant)>>,
}

impl BinanceCoinMRateLimiter {
    pub fn new() -> Self {
        Self {
            weight_limits: Mutex::new(HashMap::new()),
            ip_limits: Mutex::new(HashMap::new()),
        }
    }

    pub async fn check_weight_limit(&self, endpoint: &str, weight: u32) -> Result<(), crate::rate_limit::RateLimitError> {
        let mut limits = self.weight_limits.lock().await;
        let now = Instant::now();
        
        // Clean up old entries
        limits.retain(|_, (_, time)| now.duration_since(*time) < Duration::from_secs(60));
        
        // Check if we're over the limit
        if let Some((current_weight, _)) = limits.get(endpoint) {
            if *current_weight + weight > 1200 {
                return Err(crate::rate_limit::RateLimitError::Exceeded);
            }
        }
        
        // Update the weight
        limits.insert(endpoint.to_string(), (weight, now));
        Ok(())
    }

    pub async fn check_ip_limit(&self, ip: &str) -> Result<(), crate::rate_limit::RateLimitError> {
        let mut limits = self.ip_limits.lock().await;
        let now = Instant::now();
        
        // Clean up old entries
        limits.retain(|_, (_, time)| now.duration_since(*time) < Duration::from_secs(60));
        
        // Check if we're over the limit
        if let Some((count, _)) = limits.get(ip) {
            if *count >= 1200 {
                return Err(crate::rate_limit::RateLimitError::Exceeded);
            }
        }
        
        // Update the count
        limits.insert(ip.to_string(), (1, now));
        Ok(())
    }
}

impl RateLimiter for BinanceCoinMRateLimiter {
    async fn check_rate_limit(&self, endpoint: &str) -> Result<(), crate::rate_limit::RateLimitError> {
        self.check_weight_limit(endpoint, 1).await
    }
} 