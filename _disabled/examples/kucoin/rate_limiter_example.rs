//! KuCoin Rate Limiter Example
//!
//! This example demonstrates how to use the KuCoin rate limiter to manage API request quotas
//! according to VIP level limits and different resource pools.

use venues::kucoin::{RateLimiter, ResourcePool, VipLevel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 KuCoin Rate Limiter Example\n");

    // Create a rate limiter for VIP 5 level
    let mut rate_limiter = RateLimiter::new_with_vip(VipLevel::Vip5);
    println!("📊 Created rate limiter for VIP 5 level");

    // Show initial limits for all resource pools
    println!("\n📋 Initial rate limits:");
    let statuses = rate_limiter.get_all_statuses().await;
    for (pool, status) in &statuses {
        println!(
            "  {:?}: {}/{} requests ({}ms until reset)",
            pool, status.used, status.limit, status.reset_time_ms
        );
    }

    // Example 1: Check limits for different endpoints
    println!("\n🔍 Testing endpoint categorization:");
    let endpoints = vec![
        "/api/v1/orders",                // Spot trading
        "/api/v1/contracts/XBTUSDM",     // Futures
        "/api/v1/accounts",              // Management
        "/api/v1/earn/products",         // Earn
        "/api/v1/copytrading/positions", // Copy trading
        "/api/v1/symbols",               // Public
    ];

    for endpoint in endpoints {
        let pool = ResourcePool::from_endpoint_path(endpoint);
        println!("  {} -> {:?}", endpoint, pool);
    }

    // Example 2: Make some requests with different weights
    println!("\n⚡ Simulating API requests:");

    // Spot order (weight 2)
    match rate_limiter.check_limits(ResourcePool::Spot, 2).await {
        Ok(()) => println!("  ✅ Spot order request (weight 2) - approved"),
        Err(e) => println!("  ❌ Spot order request failed: {}", e),
    }

    // Futures position query (weight 1)
    match rate_limiter.check_limits(ResourcePool::Futures, 1).await {
        Ok(()) => println!("  ✅ Futures position query (weight 1) - approved"),
        Err(e) => println!("  ❌ Futures position query failed: {}", e),
    }

    // Account balance check (weight 5)
    match rate_limiter.check_limits(ResourcePool::Management, 5).await {
        Ok(()) => println!("  ✅ Account balance check (weight 5) - approved"),
        Err(e) => println!("  ❌ Account balance check failed: {}", e),
    }

    // Show updated limits
    println!("\n📊 Updated rate limits after requests:");
    let statuses = rate_limiter.get_all_statuses().await;
    for (pool, status) in &statuses {
        if status.used > 0 {
            println!(
                "  {:?}: {}/{} requests ({} remaining)",
                pool, status.used, status.limit, status.remaining
            );
        }
    }

    // Example 3: Test rate limit exceeded
    println!("\n🚫 Testing rate limit exceeded:");
    match rate_limiter.check_limits(ResourcePool::Spot, 20000).await {
        Ok(()) => println!("  ✅ Large request approved"),
        Err(e) => println!("  ❌ Large request failed as expected: {}", e),
    }

    // Example 4: Upgrade VIP level
    println!("\n⬆️ Upgrading to VIP 12:");
    rate_limiter.update_vip_level(VipLevel::Vip12).await;

    let spot_status = rate_limiter.get_status(ResourcePool::Spot).await.unwrap();
    println!("  New spot limit: {} requests per 30s", spot_status.limit);

    // Example 5: Check if we can proceed without consuming quota
    println!("\n🔍 Testing quota availability:");
    let can_proceed = rate_limiter
        .check_can_proceed(ResourcePool::Spot, 100)
        .await;
    println!("  Can make spot request with weight 100: {}", can_proceed);

    // Example 6: Parse rate limit headers from response
    println!("\n📨 Parsing rate limit headers:");
    use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("gw-ratelimit-limit"),
        HeaderValue::from_static("40000"),
    );
    headers.insert(
        HeaderName::from_static("gw-ratelimit-remaining"),
        HeaderValue::from_static("39900"),
    );
    headers.insert(
        HeaderName::from_static("gw-ratelimit-reset"),
        HeaderValue::from_static("25000"),
    );

    let rate_limit_header = venues::kucoin::RateLimitHeader::from_headers(&headers);
    println!(
        "  Parsed headers: limit={:?}, remaining={:?}, reset={:?}ms",
        rate_limit_header.limit, rate_limit_header.remaining, rate_limit_header.reset
    );

    println!("\n✨ Example completed successfully!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_example_flow() {
        let mut rate_limiter = RateLimiter::new_with_vip(VipLevel::Vip1);

        // Test basic functionality
        assert!(
            rate_limiter
                .check_limits(ResourcePool::Spot, 1)
                .await
                .is_ok()
        );
        assert!(
            rate_limiter
                .check_limits(ResourcePool::Futures, 1)
                .await
                .is_ok()
        );

        // Test VIP upgrade
        rate_limiter.update_vip_level(VipLevel::Vip5).await;
        let status = rate_limiter.get_status(ResourcePool::Spot).await.unwrap();
        assert_eq!(status.limit, 16000); // VIP 5 spot limit

        // Test endpoint categorization
        assert_eq!(
            ResourcePool::from_endpoint_path("/api/v1/orders"),
            ResourcePool::Spot
        );
        assert_eq!(
            ResourcePool::from_endpoint_path("/api/v1/symbols"),
            ResourcePool::Public
        );
    }
}
