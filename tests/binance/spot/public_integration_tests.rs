//! Integration tests for Binance Spot public REST API endpoints
//!
//! These tests verify the functionality of all public endpoints that don't require authentication.
//! Tests run against the live Binance API using real market data.

use reqwest::Client;
use tokio;
// Import types from top-level venue exports as required by integration test standards
use venues::binance::spot::{
    AggTradesRequest, AvgPriceRequest, DepthRequest, Errors, HistoricalTradesRequest,
    KlinesRequest, PublicRestClient, RateLimiter, Ticker24hrRequest, TickerBookRequest,
    TickerPriceRequest, TickerRequest, TickerTradingDayRequest, TradesRequest, UiKlinesRequest,
};

/// Helper function to create a test client for public endpoints
fn create_public_test_client() -> PublicRestClient {
    let client = Client::new();
    let rate_limiter = RateLimiter::new();

    PublicRestClient::new("https://api.binance.com", client, rate_limiter)
}

/// Helper function to check if an error is due to geographic restrictions
/// Returns true if the error is due to geo-restrictions, false otherwise
fn is_geo_restricted(err: &Errors) -> bool {
    let error_str = format!("{:?}", err);
    error_str.contains("451") || error_str.contains("Unavailable For Legal Reasons")
}

/// Macro to standardize handling API results with geo-restriction checks
macro_rules! handle_result {
    ($result:expr, $endpoint_name:expr) => {
        match $result {
            Ok(response) => {
                println!("✅ {} successful", $endpoint_name);
                Some(response)
            }
            Err(err) => {
                if is_geo_restricted(&err) {
                    println!(
                        "⚠️ {} skipped due to geographic restrictions (HTTP 451)",
                        $endpoint_name
                    );
                    None
                } else {
                    assert!(false, "{} should succeed: {:?}", $endpoint_name, err);
                    None
                }
            }
        }
    };
}

/// Test the ping endpoint
#[tokio::test]
async fn test_ping() {
    let client = create_public_test_client();

    let result = client.ping().await;

    assert!(result.is_ok(), "ping should succeed: {:?}", result.err());

    let response = result.unwrap();
    println!("Ping response: {:?}", response.data);
}

/// Test the server time endpoint
#[tokio::test]
async fn test_get_server_time() {
    let client = create_public_test_client();

    let result = client.get_server_time().await;

    assert!(
        result.is_ok(),
        "get_server_time should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    // Avoid assertions on dynamic data, just print it
    println!("Server time: {}", response.data.server_time);
}

/// Test the exchange info endpoint
#[tokio::test]
async fn test_get_exchange_info() {
    let client = create_public_test_client();

    // Test getting all exchange info
    let result = client.get_exchange_info(Default::default()).await;

    match result {
        Ok(response) => {
            // Verify structure but avoid assertions on dynamic data
            assert!(
                !response.data.timezone.is_empty(),
                "Timezone should not be empty"
            );
            println!("Exchange timezone: {}", response.data.timezone);
            println!("Number of symbols: {}", response.data.symbols.len());

            // Check first symbol structure if available
            if let Some(first_symbol) = response.data.symbols.first() {
                println!("First symbol: {}", first_symbol.symbol);
            }
        }
        Err(err) => {
            if is_geo_restricted(&err) {
                println!("⚠️ Test skipped due to geographic restrictions (HTTP 451)");
            } else {
                assert!(false, "get_exchange_info should succeed: {:?}", err);
            }
        }
    }
}

/// Test the depth endpoint with BTCUSDT
#[tokio::test]
async fn test_get_depth() {
    let client = create_public_test_client();

    // Create a simple depth request using struct construction
    let params = DepthRequest {
        symbol: "BTCUSDT".to_string(),
        limit: Some(5),
    };

    let result = client.get_depth(params).await;

    assert!(
        result.is_ok(),
        "get_depth should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    // Verify structure but avoid assertions on dynamic values
    assert!(
        !response.data.bids.is_empty(),
        "Should have at least one bid"
    );
    assert!(
        !response.data.asks.is_empty(),
        "Should have at least one ask"
    );

    println!("Depth - Last update ID: {}", response.data.last_update_id);
    println!(
        "Bids: {}, Asks: {}",
        response.data.bids.len(),
        response.data.asks.len()
    );
}

/// Test the recent trades endpoint
#[tokio::test]
async fn test_get_recent_trades() {
    let client = create_public_test_client();

    let params = TradesRequest {
        symbol: "BTCUSDT".to_string(),
        limit: Some(5),
    };

    let result = client.get_recent_trades(params).await;

    match result {
        Ok(response) => {
            // Verify structure
            assert!(!response.data.is_empty(), "Should have at least one trade");

            println!("Recent trades count: {}", response.data.len());

            // Check first trade structure, but don't assert on dynamic values
            if let Some(first_trade) = response.data.first() {
                println!(
                    "First trade ID: {}, price: {}",
                    first_trade.id, first_trade.price
                );
            }
        }
        Err(err) => {
            if is_geo_restricted(&err) {
                println!("⚠️ Test skipped due to geographic restrictions (HTTP 451)");
            } else {
                assert!(false, "get_recent_trades should succeed: {:?}", err);
            }
        }
    }
}

/// Test the average price endpoint
#[tokio::test]
async fn test_get_avg_price() {
    let client = create_public_test_client();

    let params = AvgPriceRequest {
        symbol: "BTCUSDT".to_string(),
    };

    let result = client.get_avg_price(params).await;

    if let Some(response) = handle_result!(result, "get_avg_price for BTCUSDT") {
        // Verify structure but avoid assertions on dynamic data like prices
        assert!(response.data.mins == 5, "Should be 5-minute average");

        println!(
            "Average price: {} ({}min)",
            response.data.price, response.data.mins
        );
    }
}

/// Test the 24hr ticker endpoint
#[tokio::test]
async fn test_get_24hr_ticker() {
    let client = create_public_test_client();

    let params = Ticker24hrRequest {
        symbol: Some("BTCUSDT".to_string()),
        symbols: None,
        ticker_type: None,
    };

    let result = client.get_24hr_ticker(Some(params)).await;

    if let Some(_response) = handle_result!(result, "get_24hr_ticker for BTCUSDT") {
        // The result could be a single ticker or array
        println!("24hr ticker response received");
    }
}

/// Test the klines endpoint
#[tokio::test]
async fn test_get_klines() {
    let client = create_public_test_client();

    let params = KlinesRequest {
        symbol: "BTCUSDT".to_string(),
        interval: "1m".to_string(),
        start_time: None,
        end_time: None,
        time_zone: None,
        limit: Some(5),
    };

    let result = client.get_klines(params).await;

    if let Some(response) = handle_result!(result, "get_klines for BTCUSDT") {
        // Verify structure
        assert!(!response.data.is_empty(), "Should have at least one kline");

        println!("Klines count: {}", response.data.len());

        // Check first kline structure (tuple) but don't assert on dynamic values
        if let Some(first_kline) = response.data.first() {
            println!(
                "First kline - Open time: {}, Open: {}, High: {}, Low: {}, Close: {}",
                first_kline.0, first_kline.1, first_kline.2, first_kline.3, first_kline.4
            );
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

        if handle_result!(result, &format!("rate_limiting_ping_{}", i)).is_some() {
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

    let params = DepthRequest {
        symbol: "INVALIDTEST".to_string(),
        limit: Some(5),
    };

    let result = client.get_depth(params).await;

    match result {
        Ok(_) => {
            println!("⚠️ Unexpected success with invalid symbol");
            assert!(false, "Expected error for invalid symbol but got success");
        }
        Err(err) => {
            if is_geo_restricted(&err) {
                println!("⚠️ Error handling test skipped due to geographic restrictions");
            } else {
                println!(
                    "✅ Error handling test - Got expected error for invalid symbol: {:?}",
                    err
                );
            }
        }
    }
}

/// Test the historical trades endpoint
#[tokio::test]
async fn test_get_historical_trades() {
    let client = create_public_test_client();

    let params = HistoricalTradesRequest {
        symbol: "BTCUSDT".to_string(),
        limit: Some(5),
        from_id: None,
    };

    let result = client.get_historical_trades(params).await;

    if let Some(response) = handle_result!(result, "get_historical_trades for BTCUSDT") {
        // Verify structure
        assert!(
            !response.data.is_empty(),
            "Should have at least one historical trade"
        );

        println!("Historical trades count: {}", response.data.len());

        // Check first trade structure but don't assert on dynamic values
        if let Some(first_trade) = response.data.first() {
            println!(
                "First historical trade ID: {}, price: {}",
                first_trade.id, first_trade.price
            );
        }
    }
}

/// Test the aggregate trades endpoint
#[tokio::test]
async fn test_get_agg_trades() {
    let client = create_public_test_client();

    let params = AggTradesRequest {
        symbol: "BTCUSDT".to_string(),
        from_id: None,
        start_time: None,
        end_time: None,
        limit: Some(5),
    };

    let result = client.get_agg_trades(params).await;

    if let Some(response) = handle_result!(result, "get_agg_trades for BTCUSDT") {
        // Verify structure
        assert!(
            !response.data.is_empty(),
            "Should have at least one aggregate trade"
        );

        println!("Aggregate trades count: {}", response.data.len());

        // Check first trade structure but don't assert on dynamic values
        if let Some(first_trade) = response.data.first() {
            println!(
                "First agg trade ID: {}, price: {}",
                first_trade.agg_trade_id, first_trade.price
            );
        }
    }
}

/// Test the UI klines endpoint
#[tokio::test]
async fn test_get_ui_klines() {
    let client = create_public_test_client();

    let params = UiKlinesRequest {
        symbol: "BTCUSDT".to_string(),
        interval: "1m".to_string(),
        start_time: None,
        end_time: None,
        time_zone: None,
        limit: Some(5),
    };

    let result = client.get_ui_klines(params).await;

    if let Some(response) = handle_result!(result, "get_ui_klines for BTCUSDT") {
        // Verify structure
        assert!(
            !response.data.is_empty(),
            "Should have at least one UI kline"
        );

        println!("UI Klines count: {}", response.data.len());

        // Check first kline structure (tuple) but don't assert on dynamic values
        if let Some(first_kline) = response.data.first() {
            println!(
                "First UI kline - Open time: {}, Open: {}, High: {}, Low: {}, Close: {}",
                first_kline.0, first_kline.1, first_kline.2, first_kline.3, first_kline.4
            );
        }
    }
}

/// Test the price ticker endpoint
#[tokio::test]
async fn test_get_price_ticker() {
    let client = create_public_test_client();

    let params = TickerPriceRequest {
        symbol: Some("BTCUSDT".to_string()),
        symbols: None,
    };

    let result = client.get_price_ticker(Some(params)).await;

    if let Some(_response) = handle_result!(result, "get_price_ticker for BTCUSDT") {
        // The result could be a single price ticker or array
        println!("Price ticker response received");
    }
}

/// Test the book ticker endpoint
#[tokio::test]
async fn test_get_book_ticker() {
    let client = create_public_test_client();

    let params = TickerBookRequest {
        symbol: Some("BTCUSDT".to_string()),
        symbols: None,
    };

    let result = client.get_book_ticker(Some(params)).await;

    if let Some(_response) = handle_result!(result, "get_book_ticker for BTCUSDT") {
        // The result could be a single book ticker or array
        println!("Book ticker response received");
    }
}

/// Test the symbol ticker endpoint
#[tokio::test]
async fn test_get_ticker() {
    let client = create_public_test_client();

    let params = TickerRequest {
        symbol: Some("BTCUSDT".to_string()),
        symbols: None,
        window_size: None,
        ticker_type: None,
    };

    let result = client.get_ticker(params).await;

    if let Some(_response) = handle_result!(result, "get_ticker for BTCUSDT") {
        // The result could be a single ticker or array
        println!("Symbol ticker response received");
    }
}

/// Test the trading day ticker endpoint
#[tokio::test]
async fn test_get_trading_day_ticker() {
    let client = create_public_test_client();

    let params = TickerTradingDayRequest {
        symbol: Some("BTCUSDT".to_string()),
        symbols: None,
        time_zone: None,
        ticker_type: None,
    };

    let result = client.get_trading_day_ticker(params).await;

    if let Some(_response) = handle_result!(result, "get_trading_day_ticker for BTCUSDT") {
        // The result could be a single trading day ticker or array
        println!("Trading day ticker response received");
    }
}

// Note: The comprehensive workflow test 'test_multiple_endpoints_sequence' has been removed
// according to the integration test standards which require one endpoint per test.
