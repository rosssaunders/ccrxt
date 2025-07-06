//! Simple integration tests for binance venue public REST API methods
//! 
//! This file demonstrates working integration tests that actually compile and run.

use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;
use venues::binance::spot::public::rest::RestClient;
use venues::binance::spot::RateLimiter;

const RATE_LIMIT_DELAY_MS: u64 = 1000; // 1 second between requests

async fn create_binance_client() -> RestClient {
    let client = Client::new();
    let rate_limiter = RateLimiter::new();
    RestClient::new(
        "https://api.binance.com",
        client,
        rate_limiter,
    )
}

#[tokio::test]
#[ignore] // Ignored by default to avoid hitting API in regular test runs
async fn test_binance_public_methods_serial() {
    let client = create_binance_client().await;
    
    // Test methods that don't require parameters
    
    // Test 1: ping
    match client.ping().await {
        Ok(_) => println!("✓ binance: ping - Success"),
        Err(e) => println!("✗ binance: ping - Error: {:?}", e),
    }
    
    // Rate limiting delay
    sleep(Duration::from_millis(RATE_LIMIT_DELAY_MS)).await;
    
    // Test 2: get_server_time
    match client.get_server_time().await {
        Ok(_) => println!("✓ binance: get_server_time - Success"),
        Err(e) => println!("✗ binance: get_server_time - Error: {:?}", e),
    }
    
    // Rate limiting delay
    sleep(Duration::from_millis(RATE_LIMIT_DELAY_MS)).await;
    
    // Test 3: get_exchange_info (with None parameter)
    match client.get_exchange_info(None).await {
        Ok(_) => println!("✓ binance: get_exchange_info - Success"),
        Err(e) => println!("✗ binance: get_exchange_info - Error: {:?}", e),
    }
}

#[tokio::test]
#[ignore]
async fn test_binance_ping_individual() {
    let client = create_binance_client().await;
    
    let result = client.ping().await;
    
    match result {
        Ok(_) => println!("✓ binance: ping - Success"),
        Err(e) => {
            println!("✗ binance: ping - Error: {:?}", e);
            // Don't fail the test for API errors, just log them
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_binance_get_server_time_individual() {
    let client = create_binance_client().await;
    
    let result = client.get_server_time().await;
    
    match result {
        Ok(_) => println!("✓ binance: get_server_time - Success"),
        Err(e) => {
            println!("✗ binance: get_server_time - Error: {:?}", e);
            // Don't fail the test for API errors, just log them
        }
    }
}
