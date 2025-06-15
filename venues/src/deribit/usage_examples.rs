//! Usage examples for Deribit public API endpoints
//!
//! This module provides practical examples of how to use the Deribit JSON-RPC client
//! to interact with public endpoints.

#[cfg(test)]
mod examples {
    use crate::deribit::{
        public::JsonRpcClient,
        AccountTier, RateLimiter,
    };

    /// Example: Create a Deribit client and check platform status
    ///
    /// This example shows how to:
    /// 1. Set up a Deribit JSON-RPC client with rate limiting
    /// 2. Call the public/status endpoint
    /// 3. Handle the response
    ///
    /// Note: This is a documentation example that doesn't make real HTTP requests.
    #[allow(dead_code)]
    async fn example_check_platform_status() -> Result<(), Box<dyn std::error::Error>> {
        // Create an HTTP client
        let http_client = reqwest::Client::new();
        
        // Create a rate limiter for your account tier
        // Choose the appropriate tier based on your 7-day trading volume:
        // - Tier4: Up to $1M (5 req/sec, 20 burst)
        // - Tier3: $1M-5M (10 req/sec, 30 burst)
        // - Tier2: $5M-25M (20 req/sec, 50 burst)
        // - Tier1: Over $25M (30 req/sec, 100 burst)
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);
        
        // Create the Deribit JSON-RPC client
        let deribit_client = JsonRpcClient::new(
            "https://www.deribit.com/api/v2",
            http_client,
            rate_limiter,
        );
        
        // Get platform status
        // This will consume 500 credits from your rate limit
        let status = deribit_client.get_status().await?;
        
        // Handle the response
        match status.locked.as_str() {
            "false" => {
                println!("✅ Platform is fully operational");
                println!("🔓 No currencies are locked");
            }
            "partial" => {
                println!("⚠️  Platform is partially locked");
                println!("🔒 Locked currency indices: {:?}", status.locked_indices);
            }
            "true" => {
                println!("🚫 Platform is fully locked");
                println!("🔒 All currencies are locked: {:?}", status.locked_indices);
            }
            _ => {
                println!("❓ Unknown lock status: {}", status.locked);
            }
        }
        
        Ok(())
    }

    /// Example: Handle rate limiting properly
    ///
    /// This example demonstrates how to handle rate limits when making multiple requests.
    #[allow(dead_code)]
    async fn example_rate_limiting() -> Result<(), Box<dyn std::error::Error>> {
        let http_client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);
        let deribit_client = JsonRpcClient::new(
            "https://www.deribit.com/api/v2",
            http_client,
            rate_limiter,
        );

        // Make multiple requests with rate limiting
        for i in 1..=3 {
            println!("Making request {}/3...", i);
            
            // Check rate limits before making the request
            let rate_status = deribit_client.rate_limiter.get_status().await;
            println!("Available credits: {}", rate_status.available_credits);
            
            if rate_status.available_credits < 500 {
                println!("⚠️  Low on credits, waiting for refill...");
                // In a real application, you might want to wait or implement backoff
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
            
            // Make the request (this will automatically check and consume credits)
            match deribit_client.get_status().await {
                Ok(status) => {
                    println!("✅ Request {} successful: locked={}", i, status.locked);
                }
                Err(crate::deribit::DeribitError::RateLimitError(e)) => {
                    println!("⚠️  Rate limited: {}", e);
                    // Handle rate limiting appropriately
                    break;
                }
                Err(e) => {
                    println!("❌ Request {} failed: {}", i, e);
                    break;
                }
            }
        }
        
        Ok(())
    }

    /// Example: Handle different response scenarios
    ///
    /// This example shows how to handle different types of responses you might receive.
    #[allow(dead_code)]
    async fn example_response_handling() -> Result<(), Box<dyn std::error::Error>> {
        let http_client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier3);
        let deribit_client = JsonRpcClient::new(
            "https://www.deribit.com/api/v2",
            http_client,
            rate_limiter,
        );

        match deribit_client.get_status().await {
            Ok(status) => {
                // Success case
                println!("Platform status received successfully!");
                
                // Detailed status analysis
                let lock_description = match status.locked.as_str() {
                    "false" => "No restrictions",
                    "partial" => "Some currencies restricted",
                    "true" => "All currencies restricted",
                    other => &format!("Unknown status: {}", other),
                };
                
                println!("Lock status: {}", lock_description);
                
                if !status.locked_indices.is_empty() {
                    println!("Affected currency indices:");
                    for index in &status.locked_indices {
                        println!("  - Currency index: {}", index);
                    }
                } else {
                    println!("No specific currencies affected");
                }
            }
            Err(crate::deribit::DeribitError::ApiError { code, message }) => {
                // API error from Deribit
                println!("Deribit API error {}: {}", code, message);
                
                match code {
                    -32601 => println!("Method not found - check API documentation"),
                    -32602 => println!("Invalid parameters - check request format"),
                    -32603 => println!("Internal error - try again later"),
                    _ => println!("Other API error - check Deribit documentation"),
                }
            }
            Err(crate::deribit::DeribitError::RateLimitError(e)) => {
                // Rate limiting error
                println!("Rate limit exceeded: {}", e);
                println!("Please wait before making more requests");
            }
            Err(crate::deribit::DeribitError::HttpError(e)) => {
                // Network or HTTP error
                println!("Network error: {}", e);
                println!("Check your internet connection and Deribit service status");
            }
            Err(crate::deribit::DeribitError::JsonError(e)) => {
                // JSON parsing error
                println!("Response parsing error: {}", e);
                println!("This might indicate an API format change");
            }
            Err(crate::deribit::DeribitError::Error(msg)) => {
                // Generic error
                println!("General error: {}", msg);
            }
        }
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_examples_compile() {
        // This test ensures all the example functions compile correctly
        // without actually calling them (to avoid network requests in tests)
        
        println!("All example functions compile successfully");
        
        // You could call the examples here if you wanted to test them:
        // example_check_platform_status().await.unwrap();
        // example_rate_limiting().await.unwrap();  
        // example_response_handling().await.unwrap();
    }
}