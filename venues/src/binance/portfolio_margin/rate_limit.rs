// Portfolio Margin Rate Limiting
//
// Portfolio Margin has identical rate limits to COIN-M Futures:
// - IP Limit: 6000/min 
// - Order Limits: 1200/min
//
// This module reuses the COIN-M rate limiting implementation since the limits are identical.

pub use crate::binance::coinm::{RateLimiter, RateLimitHeader};

/// Portfolio Margin Rate Limiter
/// 
/// Uses identical limits to COIN-M Futures:
/// - Raw requests: 61,000 per 5 min
/// - Request weight: 6,000 per 1 min  
/// - Orders: 100 per 10s, 1,200 per 1m
pub type PortfolioMarginRateLimiter = crate::binance::coinm::RateLimiter;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binance::coinm::{Errors, ApiError};
    
    #[tokio::test]
    async fn test_portfolio_margin_rate_limiter_creation() {
        let limiter = PortfolioMarginRateLimiter::new();
        
        // Test that we can check limits with the same parameters as COIN-M
        let result = limiter.check_limits(1, false).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_portfolio_margin_rate_limits() {
        let limiter = PortfolioMarginRateLimiter::new();
        
        // Test weight limit (6000/min identical to COIN-M)
        let result = limiter.check_limits(6001, false).await;
        assert!(result.is_err());
        
        if let Err(Errors::ApiError(ApiError::TooManyRequests { msg })) = result {
            assert!(msg.contains("6,000"));
        } else {
            panic!("Expected TooManyRequests error");
        }
    }
    
    #[tokio::test]
    async fn test_portfolio_margin_order_limits() {
        let limiter = PortfolioMarginRateLimiter::new();
        
        // Simulate 100 orders in 10s window
        for _ in 0..100 {
            limiter.increment_order().await;
        }
        
        // 101st order should fail (100/10s limit)
        let result = limiter.check_limits(1, true).await;
        assert!(result.is_err());
        
        if let Err(Errors::ApiError(ApiError::TooManyOrders { msg })) = result {
            assert!(msg.contains("100/10s"));
        } else {
            panic!("Expected TooManyOrders error");
        }
    }
}
