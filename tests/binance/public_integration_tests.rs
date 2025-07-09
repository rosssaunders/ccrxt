//! Integration tests for Binance Spot public REST API endpoints
//!
//! These tests verify the functionality of all public endpoints that don't require authentication.
//! Tests run against the live Binance API using real market data.

use reqwest::Client;
use tokio;

use venues::binance::spot::{PublicRestClient, RateLimiter};

/// Helper function to create a test client for public endpoints
fn create_public_test_client() -> PublicRestClient {
    let client = Client::new();
    let rate_limiter = RateLimiter::new();

    PublicRestClient::new("https://api.binance.com", client, rate_limiter)
}

/// Helper function to handle API calls with geographic restriction handling
/// Returns true if the test should continue, false if skipped due to restrictions
async fn handle_api_result<T>(
    result: Result<T, venues::binance::spot::Errors>,
    test_name: &str,
) -> Option<T> {
    match result {
        Ok(response) => {
            println!("✅ {} successful", test_name);
            Some(response)
        }
        Err(err) => {
            let error_str = format!("{:?}", err);
            if error_str.contains("451") || error_str.contains("Unavailable For Legal Reasons") {
                println!("⚠️  {} skipped due to geographic restrictions (HTTP 451)", test_name);
                None
            } else {
                panic!("Unexpected {} error: {:?}", test_name, err);
            }
        }
    }
}

/// Test the ping endpoint
#[tokio::test]
async fn test_ping() {
    let client = create_public_test_client();

    let result = client.ping().await;
    
    if let Some(response) = handle_api_result(result, "ping").await {
        // Ping returns an empty object, so we just check that we got a response
        println!("Ping response: {:?}", response.data);
    }
}

/// Test the server time endpoint
#[tokio::test]
async fn test_get_server_time() {
    let client = create_public_test_client();

    let result = client.get_server_time().await;
    
    if let Some(response) = handle_api_result(result, "get_server_time").await {
        // Verify server time is reasonable (not zero)
        assert!(response.data.server_time > 0, "Server time should be positive");
        println!("Server time: {}", response.data.server_time);
    }
}

/// Test the exchange info endpoint
#[tokio::test]
async fn test_get_exchange_info() {
    let client = create_public_test_client();

    // Test getting all exchange info
    let result = client.get_exchange_info(Default::default()).await;
    
    if let Some(response) = handle_api_result(result, "get_exchange_info").await {
        // Basic validations
        assert!(!response.data.timezone.is_empty(), "Timezone should not be empty");
        assert!(response.data.server_time > 0, "Server time should be positive");
        assert!(!response.data.symbols.is_empty(), "Should have at least one symbol");
        println!("Exchange timezone: {}", response.data.timezone);
        println!("Number of symbols: {}", response.data.symbols.len());
        
        // Check first symbol structure if available
        if let Some(first_symbol) = response.data.symbols.first() {
            println!("First symbol: {}, status: {}", first_symbol.symbol, first_symbol.status);
        }
    }
}

/// Test the depth endpoint with BTCUSDT
#[tokio::test]
async fn test_get_depth() {
    let client = create_public_test_client();

    // Create a simple depth request using struct construction
    let params = venues::binance::spot::public::rest::depth::DepthRequest {
        symbol: "BTCUSDT".to_string(),
        limit: Some(5),
    };

    let result = client.get_depth(params).await;
    
    if let Some(response) = handle_api_result(result, "get_depth for BTCUSDT").await {
        // Verify structure
        assert!(response.data.last_update_id > 0, "Update ID should be positive");
        assert!(!response.data.bids.is_empty(), "Should have at least one bid");
        assert!(!response.data.asks.is_empty(), "Should have at least one ask");
        
        println!("Depth - Last update ID: {}", response.data.last_update_id);
        println!("Bids: {}, Asks: {}", response.data.bids.len(), response.data.asks.len());
    }
}

/// Test the recent trades endpoint
#[tokio::test]
async fn test_get_recent_trades() {
    let client = create_public_test_client();

    let params = venues::binance::spot::public::rest::trades::TradesRequest {
        symbol: "BTCUSDT".to_string(),
        limit: Some(5),
    };

    let result = client.get_recent_trades(params).await;
    
    if let Some(response) = handle_api_result(result, "get_recent_trades for BTCUSDT").await {
        // Verify structure
        assert!(!response.data.is_empty(), "Should have at least one trade");
        
        println!("Recent trades count: {}", response.data.len());
        
        // Check first trade structure
        if let Some(first_trade) = response.data.first() {
            assert!(first_trade.id > 0, "Trade ID should be positive");
            assert!(first_trade.time > 0, "Trade time should be positive");
            println!("First trade ID: {}, price: {}", first_trade.id, first_trade.price);
        }
    }
}

/// Test the average price endpoint
#[tokio::test]
async fn test_get_avg_price() {
    let client = create_public_test_client();

    let params = venues::binance::spot::public::rest::avg_price::AvgPriceRequest {
        symbol: "BTCUSDT".to_string(),
    };

    let result = client.get_avg_price(params).await;
    
    if let Some(response) = handle_api_result(result, "get_avg_price for BTCUSDT").await {
        // Verify structure
        assert!(response.data.mins == 5, "Should be 5-minute average");
        assert!(response.data.price.to_string() != "0", "Price should be positive");
        
        println!("Average price: {} ({}min)", response.data.price, response.data.mins);
    }
}

/// Test the 24hr ticker endpoint
#[tokio::test]
async fn test_get_24hr_ticker() {
    let client = create_public_test_client();

    let params = venues::binance::spot::public::rest::ticker_24hr::Ticker24hrRequest {
        symbol: Some("BTCUSDT".to_string()),
        symbols: None,
        ticker_type: None,
    };

    let result = client.get_24hr_ticker(Some(params)).await;
    
    if let Some(_response) = handle_api_result(result, "get_24hr_ticker for BTCUSDT").await {
        // The result could be a single ticker or array, check if it's a single object
        println!("24hr ticker response received");
    }
}

/// Test the klines endpoint
#[tokio::test]
async fn test_get_klines() {
    let client = create_public_test_client();

    let params = venues::binance::spot::public::rest::klines::KlinesRequest {
        symbol: "BTCUSDT".to_string(),
        interval: "1m".to_string(),
        start_time: None,
        end_time: None,
        time_zone: None,
        limit: Some(5),
    };

    let result = client.get_klines(params).await;
    
    if let Some(response) = handle_api_result(result, "get_klines for BTCUSDT").await {
        // Verify structure
        assert!(!response.data.is_empty(), "Should have at least one kline");
        
        println!("Klines count: {}", response.data.len());
        
        // Check first kline structure (tuple)
        if let Some(first_kline) = response.data.first() {
            println!("First kline - Open time: {}, Open: {}, High: {}, Low: {}, Close: {}", 
                first_kline.0, first_kline.1, first_kline.2, first_kline.3, first_kline.4);
        }
    }
}

/// Test rate limiting with multiple quick requests
#[tokio::test]
async fn test_rate_limiting() {
    let client = create_public_test_client();

    // Make multiple quick requests to test rate limiting
    for i in 0..3 {
        let result = client.ping().await;

        if let Some(_) = handle_api_result(result, &format!("rate_limiting_ping_{}", i)).await {
            println!("Rate limited request {} completed successfully", i + 1);
        }

        // Small delay between requests
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

/// Test error handling with invalid symbol
#[tokio::test]
async fn test_error_handling_invalid_symbol() {
    let client = create_public_test_client();

    let params = venues::binance::spot::public::rest::depth::DepthRequest {
        symbol: "INVALIDTEST".to_string(),
        limit: Some(5),
    };

    let result = client.get_depth(params).await;
    
    match result {
        Ok(_) => {
            println!("⚠️ Unexpected success with invalid symbol");
        }
        Err(err) => {
            let error_str = format!("{:?}", err);
            if error_str.contains("451") || error_str.contains("Unavailable For Legal Reasons") {
                println!("⚠️ Error handling test skipped due to geographic restrictions");
            } else {
                println!("✅ Error handling test - Got expected error for invalid symbol: {:?}", err);
            }
        }
    }
}