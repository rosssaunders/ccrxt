//! Integration tests for Bitmart spot public REST API endpoints.
//!
//! These tests verify that the Bitmart spot public REST API client can successfully
//! communicate with the live API and receive valid responses.

use venues::bitmart::spot::public::rest::{
    GetCurrencyListRequest, GetDepthRequest, GetHistoryKlineRequest, GetLatestKlineRequest,
    GetRecentTradesRequest, GetTickerAllPairsRequest, GetTickerRequest,
    GetTradingPairDetailsRequest, GetTradingPairsListRequest, RestClient,
};

/// Helper function to create a test client
fn create_spot_test_client() -> RestClient {
    use std::sync::Arc;
    use venues::bitmart::rate_limit::RateLimiter;

    let http_client = Arc::new(rest::native::NativeHttpClient::default());
    let rate_limiter = RateLimiter::default();

    RestClient::new("https://api-cloud.bitmart.com", http_client, rate_limiter)
}

/// Test get currency list endpoint
#[tokio::test]
async fn test_get_currency_list() {
    let client = create_spot_test_client();
    let request = GetCurrencyListRequest {};

    let result = client.get_currency_list(request).await;
    assert!(
        result.is_ok(),
        "get_currency_list should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.currencies.is_empty(),
        "Should have at least one currency"
    );

    println!("Found {} currencies", response.currencies.len());
    if let Some(first_currency) = response.currencies.first() {
        println!(
            "First currency: {} (name: {}, withdraw: {}, deposit: {})",
            first_currency.id,
            first_currency.name,
            first_currency.withdraw_enabled,
            first_currency.deposit_enabled
        );
    }
}

/// Test get trading pairs list endpoint
#[tokio::test]
async fn test_get_trading_pairs_list() {
    let client = create_spot_test_client();
    let request = GetTradingPairsListRequest {};

    let result = client.get_trading_pairs_list(request).await;
    assert!(
        result.is_ok(),
        "get_trading_pairs_list should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.symbols.is_empty(),
        "Should have at least one trading pair"
    );

    println!("Found {} trading pairs", response.symbols.len());
    if let Some(first_pair) = response.symbols.first() {
        println!("First trading pair: {}", first_pair);
    }
}

/// Test get trading pair details endpoint
#[tokio::test]
async fn test_get_trading_pair_details() {
    let client = create_spot_test_client();
    let request = GetTradingPairDetailsRequest {};

    let result = client.get_trading_pair_details(request).await;
    assert!(
        result.is_ok(),
        "get_trading_pair_details should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.symbols.is_empty(),
        "Should have trading pair details"
    );

    let symbol_detail = &response.symbols[0];
    assert!(!symbol_detail.symbol.is_empty(), "Should have symbol");

    println!(
        "Trading pair details: {} (status: {}, precision: {})",
        symbol_detail.symbol, symbol_detail.trade_status, symbol_detail.price_min_precision
    );
}

/// Test get ticker endpoint
#[tokio::test]
async fn test_get_ticker() {
    let client = create_spot_test_client();
    let request = GetTickerRequest {
        symbol: "BTC_USDT".to_string(),
    };

    let result = client.get_ticker(request).await;
    assert!(
        result.is_ok(),
        "get_ticker should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.symbol, "BTC_USDT", "Should have correct symbol");
    assert!(!response.last.is_empty(), "Should have last price");

    println!(
        "Ticker for {}: price={}, volume={}, change={}%",
        response.symbol, response.last, response.v_24h, response.fluctuation
    );
}

/// Test get ticker all pairs endpoint
#[tokio::test]
async fn test_get_ticker_all_pairs() {
    let client = create_spot_test_client();
    let request = GetTickerAllPairsRequest {};

    let result = client.get_ticker_all_pairs(request).await;
    assert!(
        result.is_ok(),
        "get_ticker_all_pairs should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.0.is_empty(), "Should have at least one ticker");

    println!("Found {} tickers", response.0.len());
    if let Some(first_ticker) = response.0.first() {
        // Ticker data is array format: [symbol, last_price, quote_volume, ...]
        if !first_ticker.is_empty() {
            println!("First ticker: {}", first_ticker[0]);
        }
    }
}

/// Test get depth endpoint
#[tokio::test]
async fn test_get_depth() {
    let client = create_spot_test_client();
    let request = GetDepthRequest {
        symbol: "BTC_USDT".to_string(),
        limit: Some(10),
    };

    let result = client.get_depth(request).await;
    assert!(
        result.is_ok(),
        "get_depth should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.bids.is_empty(), "Should have buy orders");
    assert!(!response.asks.is_empty(), "Should have sell orders");

    println!(
        "Order book for {}: {} bids, {} asks (timestamp: {})",
        response.symbol,
        response.bids.len(),
        response.asks.len(),
        response.ts
    );
}

/// Test get recent trades endpoint
#[tokio::test]
async fn test_get_recent_trades() {
    let client = create_spot_test_client();
    let request = GetRecentTradesRequest {
        symbol: "BTC_USDT".to_string(),
        limit: Some(10),
    };

    let result = client.get_recent_trades(request).await;
    assert!(
        result.is_ok(),
        "get_recent_trades should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.0.is_empty(), "Should have recent trades");

    println!("Recent trades: {} trades", response.0.len());

    if let Some(first_trade) = response.0.first() {
        // Trade data is array format: [symbol, timestamp, price, size, side]
        if first_trade.len() >= 5 {
            println!(
                "First trade: price={}, amount={}, side={}",
                first_trade[2], first_trade[3], first_trade[4]
            );
        }
    }
}

/// Test get latest kline endpoint
#[tokio::test]
async fn test_get_latest_kline() {
    let client = create_spot_test_client();
    let request = GetLatestKlineRequest {
        symbol: "BTC_USDT".to_string(),
        before: None,
        after: None,
        step: Some(60), // 1 minute
        limit: Some(10),
    };

    let result = client.get_latest_kline(request).await;
    assert!(
        result.is_ok(),
        "get_latest_kline should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.0.is_empty(), "Should have kline data");

    println!("Latest klines: {} candles", response.0.len());

    if let Some(first_kline) = response.0.first() {
        // Kline data is array format: [timestamp, open, high, low, close, volume, quote_volume]
        if first_kline.len() >= 7 {
            println!(
                "First kline: timestamp={}, open={}, close={}",
                first_kline[0], first_kline[1], first_kline[4]
            );
        }
    }
}

/// Test get history kline endpoint
#[tokio::test]
async fn test_get_history_kline() {
    let client = create_spot_test_client();
    let request = GetHistoryKlineRequest {
        symbol: "BTC_USDT".to_string(),
        before: Some(1641081600), // 2022-01-02 00:00:00 UTC
        after: Some(1640995200),  // 2022-01-01 00:00:00 UTC
        step: Some(60),           // 1 hour (60 minutes)
        limit: Some(10),
    };

    let result = client.get_history_kline(request).await;
    assert!(
        result.is_ok(),
        "get_history_kline should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    println!("Historical klines: {} candles", response.0.len());

    if !response.0.is_empty() {
        let first_kline = &response.0[0];
        // Kline data is array format: [timestamp, open, high, low, close, volume, quote_volume]
        if first_kline.len() >= 5 {
            println!(
                "First historical kline: timestamp={}, open={}, close={}",
                first_kline[0], first_kline[1], first_kline[4]
            );
        }
    }
}

/// Test client creation and basic functionality
#[tokio::test]
async fn test_spot_client_creation() {
    let _client = create_spot_test_client();
    println!("✓ Bitmart spot client creation successful");
}

/// Test error handling with invalid symbol
#[tokio::test]
async fn test_error_handling_invalid_symbol() {
    let client = create_spot_test_client();
    let request = GetTickerRequest {
        symbol: "INVALID_SYMBOL_XYZ".to_string(),
    };

    let result = client.get_ticker(request).await;
    // This should either fail or return empty results
    if result.is_err() {
        println!("Expected error for invalid symbol: {:?}", result.err());
    } else {
        println!("API returned response for invalid symbol (may be valid behavior)");
    }
}

/// Test rate limiting behavior
#[tokio::test]
async fn test_rate_limiting() {
    let client = create_spot_test_client();

    // Make multiple rapid requests to test rate limiting
    let mut results = Vec::new();

    for i in 0..5 {
        let request = GetCurrencyListRequest {};
        let result = client.get_currency_list(request).await;
        results.push(result);
        println!(
            "Request {}: {:?}",
            i + 1,
            if results[i].is_ok() {
                "Success"
            } else {
                "Failed"
            }
        );
    }

    // At least some requests should succeed with reasonable rate limiting
    let successful_count = results.iter().filter(|r| r.is_ok()).count();
    assert!(
        successful_count >= 3,
        "At least 3 out of 5 requests should succeed"
    );

    println!(
        "Rate limiting test: {}/5 requests succeeded",
        successful_count
    );
}
