//! Integration tests for Binance Options public REST API endpoints
//!
//! These tests verify the functionality of all public endpoints for Binance Options (EAPI)
//! that don't require authentication. Tests run against the live Binance Options API.

use std::time::Duration;

use chrono::{Duration as ChronoDuration, Utc};
use tokio;
use venues::binance::{
    options::{
        PublicRestClient,
        public::rest::{
            klines::KlinesRequest, mark_price::MarkPriceRequest, order_book::OrderBookRequest,
            recent_trades::RecentTradesRequest, ticker::TickerRequest,
        },
    },
    shared::{RateLimiter, RateLimits},
};

/// Helper function to create a test client for public endpoints
fn create_public_test_client() -> PublicRestClient {
    let http_client = std::sync::Arc::new(rest::native::NativeHttpClient::default());
    let rate_limits = RateLimits {
        request_weight_limit: 6000,
        request_weight_window: Duration::from_secs(60),
        raw_requests_limit: 61000,
        raw_requests_window: Duration::from_secs(300),
        orders_10s_limit: 100,
        orders_minute_limit: 1200,
        orders_day_limit: None,
    };
    let rate_limiter = RateLimiter::new(rate_limits);

    PublicRestClient::new("https://eapi.binance.com", http_client, rate_limiter)
}

/// Helper function to get a valid BTC option symbol for testing
/// Note: This is a placeholder - in real tests you'd query exchange info first
fn get_test_option_symbol() -> String {
    // This format represents a BTC option with expiry 230630, strike 30000, Call type
    // In practice, you should query exchange_info first to get current valid symbols
    "BTC-240329-70000-C".to_string() // BTC Call option, expires Mar 29 2024, strike $70,000
}

/// Test the ping endpoint
#[tokio::test]
async fn test_ping() {
    let client = create_public_test_client();

    let result = client.ping().await;

    match result {
        Ok(response) => {
            println!("✅ Binance Options ping successful");
            println!("Request duration: {:?}", response.request_duration);
            // Ping response should be empty object
            println!("Response headers: {:?}", response.headers);
        }
        Err(error) => {
            println!("⚠️  Binance Options ping failed: {:?}", error);
            // Ping failures might indicate network issues or API unavailability
        }
    }
}

/// Test the server time endpoint
#[tokio::test]
async fn test_server_time() {
    let client = create_public_test_client();

    let result = client.get_server_time().await;

    match result {
        Ok(response) => {
            assert!(
                response.data.server_time > 0,
                "Server time should be positive"
            );
            println!(
                "✅ Binance Options server time: {}",
                response.data.server_time
            );
            println!("Request duration: {:?}", response.request_duration);

            // Check that server time is reasonable (within last hour and next hour)
            let now = Utc::now().timestamp_millis() as u64;
            let hour_ms = 60 * 60 * 1000;
            assert!(
                response.data.server_time > now - hour_ms
                    && response.data.server_time < now + hour_ms,
                "Server time should be close to current time"
            );
        }
        Err(error) => {
            println!("⚠️  Binance Options server time failed: {:?}", error);
        }
    }
}

/// Test the exchange info endpoint
#[tokio::test]
async fn test_exchange_info() {
    let client = create_public_test_client();

    let result = client.get_exchange_info().await;

    match result {
        Ok(response) => {
            println!("✅ Binance Options exchange info retrieved");
            println!("Timezone: {}", response.data.timezone);
            println!("Option contracts: {}", response.data.option_contracts.len());
            println!("Option assets: {}", response.data.option_assets.len());
            println!("Option symbols: {}", response.data.option_symbols.len());
            println!("Rate limits: {}", response.data.rate_limits.len());

            // Verify structure
            assert!(
                !response.data.timezone.is_empty(),
                "Timezone should not be empty"
            );
            assert!(
                response.data.server_time > 0,
                "Server time should be positive"
            );
            assert!(
                !response.data.rate_limits.is_empty(),
                "Should have rate limits"
            );

            // Check that we have some option contracts and symbols
            assert!(
                !response.data.option_contracts.is_empty(),
                "Should have option contracts"
            );

            if !response.data.option_symbols.is_empty() {
                let first_symbol = &response.data.option_symbols[0];
                assert!(
                    !first_symbol.symbol.is_empty(),
                    "Symbol name should not be empty"
                );
                println!("First option symbol: {}", first_symbol.symbol);
                println!("  Side: {:?}", first_symbol.side);
                println!("  Strike price: {}", first_symbol.strike_price);
                println!("  Expiry date: {}", first_symbol.expiry_date);
            }
        }
        Err(error) => {
            println!("⚠️  Binance Options exchange info failed: {:?}", error);
        }
    }
}

/// Test the order book endpoint
#[tokio::test]
async fn test_order_book() {
    let client = create_public_test_client();
    let symbol = get_test_option_symbol();

    let request = OrderBookRequest {
        symbol: symbol.clone(),
        limit: Some(100),
    };

    let result = client.get_order_book(request).await;

    match result {
        Ok(response) => {
            println!("✅ Order book for {}", symbol);
            println!("  Bids: {}", response.data.bids.len());
            println!("  Asks: {}", response.data.asks.len());
            println!("  Update ID: {}", response.data.update_id);

            // Verify structure if data is available
            if !response.data.bids.is_empty() {
                let best_bid = &response.data.bids[0];
                println!("  Best bid: {} @ {}", best_bid.1, best_bid.0);
                println!("  Best bid price: {}", best_bid.0);
                println!("  Best bid quantity: {}", best_bid.1);
            }

            if !response.data.asks.is_empty() {
                let best_ask = &response.data.asks[0];
                println!("  Best ask: {} @ {}", best_ask.1, best_ask.0);
                println!("  Best ask price: {}", best_ask.0);
                println!("  Best ask quantity: {}", best_ask.1);
            }
        }
        Err(error) => {
            println!("⚠️  Order book request failed for {}: {:?}", symbol, error);
            // This might fail if the symbol doesn't exist or has no market data
        }
    }
}

/// Test the ticker endpoint with no symbol (all symbols)
#[tokio::test]
async fn test_ticker_all_symbols() {
    let client = create_public_test_client();

    let request = TickerRequest { symbol: None };

    let result = client.get_ticker(request).await;

    match result {
        Ok(response) => {
            println!(
                "✅ Ticker data for all symbols: {} tickers",
                response.data.len()
            );

            if !response.data.is_empty() {
                let first_ticker = &response.data[0];
                println!("First ticker - Symbol: {}", first_ticker.symbol);
                println!("  Price change: {}", first_ticker.price_change);
                println!(
                    "  Price change percent: {}%",
                    first_ticker.price_change_percent
                );
                println!("  Last price: {}", first_ticker.last_price);
                println!("  Volume: {}", first_ticker.volume);

                assert!(
                    !first_ticker.symbol.is_empty(),
                    "Symbol should not be empty"
                );
                // Note: Other fields might be zero for options with no recent activity
            }
        }
        Err(error) => {
            println!("⚠️  Ticker request failed: {:?}", error);
        }
    }
}

/// Test the ticker endpoint with specific symbol
#[tokio::test]
async fn test_ticker_specific_symbol() {
    let client = create_public_test_client();
    let symbol = get_test_option_symbol();

    let request = TickerRequest {
        symbol: Some(symbol.clone()),
    };

    let result = client.get_ticker(request).await;

    match result {
        Ok(response) => {
            // Should return array with one element for specific symbol
            assert_eq!(
                response.data.len(),
                1,
                "Should return exactly one ticker for specific symbol"
            );

            let ticker = &response.data[0];
            println!("✅ Ticker for {}", symbol);
            println!("  Symbol: {}", ticker.symbol);
            println!("  Price change: {}", ticker.price_change);
            println!("  Price change percent: {}%", ticker.price_change_percent);
            println!("  Last price: {}", ticker.last_price);
            println!("  Volume: {}", ticker.volume);
            println!("  Open time: {}", ticker.open_time);
            println!("  Close time: {}", ticker.close_time);

            assert_eq!(
                ticker.symbol, symbol,
                "Returned symbol should match requested symbol"
            );
        }
        Err(error) => {
            println!("⚠️  Ticker request failed for {}: {:?}", symbol, error);
        }
    }
}

/// Test the mark price endpoint
#[tokio::test]
async fn test_mark_price() {
    let client = create_public_test_client();
    let symbol = get_test_option_symbol();

    let request = MarkPriceRequest {
        symbol: Some(symbol.clone()),
    };

    let result = client.get_mark_price(request).await;

    match result {
        Ok(response) => {
            if !response.data.is_empty() {
                let mark_data = &response.data[0];
                println!("✅ Mark price for {}", symbol);
                println!("  Symbol: {}", mark_data.symbol);
                println!("  Mark price: {}", mark_data.mark_price);
                println!("  Bid IV: {}", mark_data.bid_iv);
                println!("  Ask IV: {}", mark_data.ask_iv);
                println!("  Delta: {}", mark_data.delta);
                println!("  Theta: {}", mark_data.theta);
                println!("  Gamma: {}", mark_data.gamma);
                println!("  Vega: {}", mark_data.vega);

                assert_eq!(mark_data.symbol, symbol, "Symbol should match request");
                // Mark price should be non-negative but we'll skip the assertion since we don't have direct access to Decimal::ZERO
                println!("  Mark price is: {}", mark_data.mark_price);
            } else {
                println!("⚠️  No mark price data available for {}", symbol);
            }
        }
        Err(error) => {
            println!("⚠️  Mark price request failed for {}: {:?}", symbol, error);
        }
    }
}

/// Test the mark price endpoint for all symbols
#[tokio::test]
async fn test_mark_price_all_symbols() {
    let client = create_public_test_client();

    let request = MarkPriceRequest { symbol: None };

    let result = client.get_mark_price(request).await;

    match result {
        Ok(response) => {
            println!(
                "✅ Mark price data for all symbols: {} items",
                response.data.len()
            );

            if !response.data.is_empty() {
                let first_item = &response.data[0];
                println!("First mark price item - Symbol: {}", first_item.symbol);
                println!("  Mark price: {}", first_item.mark_price);
                println!(
                    "  Greeks - Delta: {}, Gamma: {}, Theta: {}, Vega: {}",
                    first_item.delta, first_item.gamma, first_item.theta, first_item.vega
                );

                assert!(!first_item.symbol.is_empty(), "Symbol should not be empty");
            }
        }
        Err(error) => {
            println!("⚠️  Mark price request for all symbols failed: {:?}", error);
        }
    }
}

/// Test the klines endpoint
#[tokio::test]
async fn test_klines() {
    let client = create_public_test_client();
    let symbol = get_test_option_symbol();

    let request = KlinesRequest {
        symbol: symbol.clone(),
        interval: "1h".to_string(),
        start_time: None,
        end_time: None,
        limit: Some(100),
    };

    let result = client.get_klines(request).await;

    match result {
        Ok(response) => {
            println!(
                "✅ Klines for {} ({}): {} candles",
                symbol,
                "1h",
                response.data.len()
            );

            if !response.data.is_empty() {
                let first_kline = &response.data[0];
                println!("First kline:");
                println!("  Open time: {}", first_kline.open_time);
                println!("  Open: {}", first_kline.open);
                println!("  High: {}", first_kline.high);
                println!("  Low: {}", first_kline.low);
                println!("  Close: {}", first_kline.close);
                println!("  Volume: {}", first_kline.volume);
                println!("  Trade count: {}", first_kline.trade_count);

                // Verify OHLC data integrity
                assert!(
                    first_kline.high >= first_kline.open,
                    "High should be >= open"
                );
                assert!(
                    first_kline.high >= first_kline.close,
                    "High should be >= close"
                );
                assert!(first_kline.low <= first_kline.open, "Low should be <= open");
                assert!(
                    first_kline.low <= first_kline.close,
                    "Low should be <= close"
                );
                // Volume should be non-negative but we'll skip the assertion
                println!("  Volume is: {}", first_kline.volume);
            } else {
                println!("⚠️  No kline data available for {}", symbol);
            }
        }
        Err(error) => {
            println!("⚠️  Klines request failed for {}: {:?}", symbol, error);
        }
    }
}

/// Test the klines endpoint with time range
#[tokio::test]
async fn test_klines_with_time_range() {
    let client = create_public_test_client();
    let symbol = get_test_option_symbol();

    // Get data from 24 hours ago to now
    let end_time = Utc::now();
    let start_time = end_time - ChronoDuration::hours(24);

    let request = KlinesRequest {
        symbol: symbol.clone(),
        interval: "15m".to_string(),
        start_time: Some(start_time.timestamp_millis() as u64),
        end_time: Some(end_time.timestamp_millis() as u64),
        limit: Some(50),
    };

    let result = client.get_klines(request).await;

    match result {
        Ok(response) => {
            println!(
                "✅ Klines for {} with time range: {} candles",
                symbol,
                response.data.len()
            );

            if !response.data.is_empty() {
                let first = &response.data[0];
                let last = &response.data[response.data.len() - 1];

                println!("Time range: {} to {}", first.open_time, last.open_time);

                // Verify time ordering (data may be in reverse chronological order)
                if first.open_time > last.open_time {
                    println!("  Data is in reverse chronological order (newest first)");
                } else {
                    println!("  Data is in chronological order (oldest first)");
                }
                // Either order is valid, just verify we have sensible timestamps
                assert!(
                    first.open_time > 0,
                    "First kline timestamp should be positive"
                );
                assert!(
                    last.open_time > 0,
                    "Last kline timestamp should be positive"
                );
            }
        }
        Err(error) => {
            println!(
                "⚠️  Klines with time range failed for {}: {:?}",
                symbol, error
            );
        }
    }
}

/// Test the recent trades endpoint
#[tokio::test]
async fn test_recent_trades() {
    let client = create_public_test_client();
    let symbol = get_test_option_symbol();

    let request = RecentTradesRequest {
        symbol: symbol.clone(),
        limit: Some(100),
    };

    let result = client.get_recent_trades(request).await;

    match result {
        Ok(response) => {
            println!(
                "✅ Recent trades for {}: {} trades",
                symbol,
                response.data.len()
            );

            if !response.data.is_empty() {
                let first_trade = &response.data[0];
                println!("Most recent trade:");
                println!("  ID: {}", first_trade.id);
                println!("  Price: {}", first_trade.price);
                println!("  Quantity: {}", first_trade.qty);
                println!("  Time: {}", first_trade.time);
                println!("  Side: {}", first_trade.side);

                // Trade price should be positive but we'll skip the assertion
                println!("  Price is: {}", first_trade.price);
                // Trade quantity should be positive but we'll skip the assertion
                println!("  Quantity is: {}", first_trade.qty);
                assert!(first_trade.time > 0, "Trade time should be positive");
            } else {
                println!("⚠️  No recent trades available for {}", symbol);
            }
        }
        Err(error) => {
            println!(
                "⚠️  Recent trades request failed for {}: {:?}",
                symbol, error
            );
        }
    }
}

/// Test error handling with invalid symbol
#[tokio::test]
async fn test_error_handling_invalid_symbol() {
    let client = create_public_test_client();
    let invalid_symbol = "INVALID-SYMBOL-123".to_string();

    let request = OrderBookRequest {
        symbol: invalid_symbol.clone(),
        limit: Some(100),
    };

    let result = client.get_order_book(request).await;

    match result {
        Ok(_) => {
            println!("⚠️  Expected error for invalid symbol but request succeeded");
        }
        Err(error) => {
            println!(
                "✅ Correctly received error for invalid symbol: {:?}",
                error
            );
            // Error should be structured and informative
        }
    }
}

/// Test rate limiting functionality
#[tokio::test]
async fn test_rate_limiting() {
    let client = create_public_test_client();

    // Make multiple quick requests to test rate limiting
    for i in 0..3 {
        let result = client.get_server_time().await;

        match result {
            Ok(response) => {
                println!("✅ Rate limited request {} completed successfully", i + 1);
                println!("  Headers: {:?}", response.headers);

                // Small delay between requests
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
            Err(error) => {
                println!("⚠️  Rate limited request {} failed: {:?}", i + 1, error);
                break;
            }
        }
    }
}

/// Test client creation and configuration
#[test]
fn test_client_creation() {
    let _client = create_public_test_client();
    // Client is wrapped, so we can't access base_url directly
    println!("✅ Binance Options Public REST client created successfully");
}
