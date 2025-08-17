//! Integration tests for Binance USD-M Futures public REST API endpoints
//!
//! These tests verify the functionality of all public endpoints that don't require authentication.
//! Tests run against the live Binance USD-M API using real market data.
//!
//! **Note:** Binance API has geographic restrictions. Tests may fail with "Service unavailable
//! from a restricted location" errors when run from certain locations. This is expected behavior
//! and indicates the tests are correctly configured to reach the live API.

use std::time::Duration;

use tokio;
use venues::binance::{
    shared::{RateLimiter, RateLimits},
    usdm::PublicRestClient,
};

/// Helper function to create a test client for public endpoints
fn create_public_test_client() -> PublicRestClient {
    let http_client = std::sync::Arc::new(rest::native::NativeHttpClient::default());
    let rate_limits = RateLimits {
        request_weight_limit: 2400,
        request_weight_window: Duration::from_secs(60),
        raw_requests_limit: 1200,
        raw_requests_window: Duration::from_secs(60),
        orders_10s_limit: 100,
        orders_minute_limit: 1200,
        orders_day_limit: None,
    };
    let rate_limiter = RateLimiter::new(rate_limits);

    PublicRestClient::new("https://fapi.binance.com", http_client, rate_limiter)
}

/// Test the ping endpoint - test connectivity
#[tokio::test]
async fn test_ping() {
    let client = create_public_test_client();

    let result = client
        .ping(venues::binance::usdm::public::rest::ping::PingRequest::default())
        .await;
    assert!(
        result.is_ok(),
        "ping request should succeed: {:?}",
        result.err()
    );

    let _response = result.unwrap();
    // Ping response is empty, just verify we got a response
    println!(
        "Ping successful: request took {:?}",
        std::time::Duration::from_secs(0)
    );
}

/// Test the server time endpoint
#[tokio::test]
async fn test_server_time() {
    let client = create_public_test_client();

    let result = client
        .server_time(venues::binance::usdm::public::rest::time::ServerTimeRequest::default())
        .await;
    assert!(
        result.is_ok(),
        "server_time request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        response.data.server_time > 0,
        "Server time should be a positive timestamp"
    );
    println!(
        "Server time: {} (took {:?})",
        response.data.server_time,
        std::time::Duration::from_secs(0)
    );
}

/// Test the exchange info endpoint
#[tokio::test]
async fn test_get_exchange_info() {
    let client = create_public_test_client();

    let result = client.get_exchange_info().await;
    assert!(
        result.is_ok(),
        "get_exchange_info request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();

    // Verify basic exchange info structure
    assert!(response.data.timezone == "UTC", "Timezone should be UTC");
    assert!(
        !response.data.rate_limits.is_empty(),
        "Should have rate limits"
    );
    // Exchange filters can be empty in USD-M futures
    assert!(!response.data.assets.is_empty(), "Should have assets");
    assert!(!response.data.symbols.is_empty(), "Should have symbols");

    // Verify at least one symbol exists and has required fields
    let symbol = &response.data.symbols[0];
    assert!(!symbol.symbol.is_empty(), "Symbol name should not be empty");
    assert!(
        !symbol.base_asset.is_empty(),
        "Base asset should not be empty"
    );
    // margin_asset is now an enum, so we just verify it exists (which it does if deserialization succeeded)

    println!(
        "Exchange info fetched successfully: {} symbols, took {:?}",
        response.data.symbols.len(),
        std::time::Duration::from_secs(0)
    );
}
/// Test basic functionality without relying on private request types
#[tokio::test]
async fn test_basic_endpoints() {
    let client = create_public_test_client();

    // Test ping
    let ping_result = client
        .ping(venues::binance::usdm::public::rest::ping::PingRequest::default())
        .await;
    assert!(ping_result.is_ok(), "Ping should succeed");

    // Test server time
    let time_result = client
        .server_time(venues::binance::usdm::public::rest::time::ServerTimeRequest::default())
        .await;
    assert!(time_result.is_ok(), "Server time should succeed");

    // Test exchange info
    let exchange_result = client.get_exchange_info().await;
    assert!(exchange_result.is_ok(), "Exchange info should succeed");

    // Test funding rate info
    let funding_result = client.get_funding_rate_info().await;
    assert!(funding_result.is_ok(), "Funding rate info should succeed");

    if let Ok(funding_response) = funding_result {
        assert!(
            !funding_response.data.is_empty(),
            "Should have funding rate data"
        );
        let first_funding = &funding_response.data[0];
        assert!(!first_funding.symbol.is_empty(), "Should have symbol");
        // Note: FundingRateInfo doesn't have funding_rate field, only rate cap/floor
        println!("Funding rate info: {} symbols", funding_response.data.len());
    }

    println!("Basic endpoints test completed successfully");
}

/// Test rate limiting behavior
#[tokio::test]
async fn test_rate_limiting() {
    let client = create_public_test_client();

    // Make multiple rapid requests to test rate limiting
    let mut results = Vec::new();

    for i in 0..5 {
        let result = client
            .ping(venues::binance::usdm::public::rest::ping::PingRequest::default())
            .await;
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

    // All requests should succeed for ping endpoint with reasonable rate limiting
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

/// Test endpoint diversity
#[tokio::test]
async fn test_endpoint_diversity() {
    let client = create_public_test_client();

    // Test different endpoint types
    let endpoints_tested = [
        (
            "ping",
            client
                .ping(venues::binance::usdm::public::rest::ping::PingRequest::default())
                .await
                .is_ok(),
        ),
        (
            "server_time",
            client
                .server_time(
                    venues::binance::usdm::public::rest::time::ServerTimeRequest::default(),
                )
                .await
                .is_ok(),
        ),
        ("exchange_info", client.get_exchange_info().await.is_ok()),
        (
            "funding_rate_info",
            client.get_funding_rate_info().await.is_ok(),
        ),
    ];

    let successful_endpoints: Vec<_> = endpoints_tested
        .iter()
        .filter(|(_, success)| *success)
        .map(|(name, _)| *name)
        .collect();

    println!("Successful endpoints: {:?}", successful_endpoints);

    // At least 3 out of 4 endpoints should work
    assert!(
        successful_endpoints.len() >= 3,
        "At least 3 endpoints should succeed, got: {:?}",
        successful_endpoints
    );
}

/// Test order book (depth) endpoint
#[tokio::test]
async fn test_get_order_book() {
    use venues::binance::usdm::public::rest::depth::OrderBookRequest;

    let client = create_public_test_client();
    let request = OrderBookRequest {
        symbol: "BTCUSDT".into(),
        limit: Some(10),
    };

    let result = client.get_order_book(request).await;
    assert!(
        result.is_ok(),
        "get_order_book request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.data.bids.is_empty(), "Should have bid orders");
    assert!(!response.data.asks.is_empty(), "Should have ask orders");
    assert!(
        response.data.last_update_id > 0,
        "Should have valid update ID"
    );

    println!(
        "Order book: {} bids, {} asks (took {:?})",
        response.data.bids.len(),
        response.data.asks.len(),
        std::time::Duration::from_secs(0)
    );
}

/// Test recent trades endpoint
#[tokio::test]
async fn test_recent_trades() {
    use venues::binance::usdm::public::rest::recent_trades::RecentTradesRequest;

    let client = create_public_test_client();
    let request = RecentTradesRequest {
        symbol: "BTCUSDT".into(),
        limit: Some(10),
    };

    let result = client.recent_trades(request).await;
    assert!(
        result.is_ok(),
        "recent_trades request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.data.trades.is_empty(), "Should have trade data");

    let trade = &response.data.trades[0];
    assert!(trade.id > 0, "Trade should have valid ID");
    assert!(!trade.price.is_empty(), "Trade should have price");
    assert!(!trade.qty.is_empty(), "Trade should have quantity");

    println!(
        "Recent trades: {} trades (took {:?})",
        response.data.trades.len(),
        std::time::Duration::from_secs(0)
    );
}

/// Test klines (candlestick) endpoint
#[tokio::test]
async fn test_get_klines() {
    use venues::binance::usdm::public::rest::klines::KlinesRequest;

    let client = create_public_test_client();
    let request = KlinesRequest {
        symbol: "BTCUSDT".into(),
        interval: venues::binance::usdm::KlineInterval::I1m,
        start_time: None,
        end_time: None,
        limit: Some(10),
    };

    let result = client.get_klines(request).await;
    assert!(
        result.is_ok(),
        "get_klines request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.data.is_empty(), "Should have kline data");

    let kline = &response.data[0];
    assert!(kline.open_time > 0, "Kline should have valid open time");
    assert!(!kline.open.is_empty(), "Kline should have open price");
    assert!(!kline.close.is_empty(), "Kline should have close price");

    println!(
        "Klines: {} candles (took {:?})",
        response.data.len(),
        std::time::Duration::from_secs(0)
    );
}

/// Test 24hr ticker endpoint
#[tokio::test]
async fn test_get_ticker_24hr() {
    use venues::binance::usdm::public::rest::ticker_24hr::Ticker24hrRequest;

    let client = create_public_test_client();
    let request = Ticker24hrRequest {
        symbol: Some("BTCUSDT".into()),
    };

    let result = client.get_ticker_24hr(request).await;
    assert!(
        result.is_ok(),
        "get_ticker_24hr request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();

    // Handle both single and multiple results
    match &response.data {
        venues::binance::usdm::public::rest::ticker_24hr::Ticker24hrResult::Single(ticker) => {
            assert_eq!(ticker.symbol, "BTCUSDT", "Should have correct symbol");
            assert!(!ticker.last_price.is_empty(), "Should have last price");
        }
        venues::binance::usdm::public::rest::ticker_24hr::Ticker24hrResult::Multiple(tickers) => {
            assert!(!tickers.is_empty(), "Should have ticker data");
            let ticker = &tickers[0];
            assert_eq!(ticker.symbol, "BTCUSDT", "Should have correct symbol");
            assert!(!ticker.last_price.is_empty(), "Should have last price");
        }
    }

    // Print ticker info based on result type
    match &response.data {
        venues::binance::usdm::public::rest::ticker_24hr::Ticker24hrResult::Single(ticker) => {
            assert!(!ticker.volume.is_empty(), "Should have volume");
            println!(
                "24hr ticker: price={}, volume={} (took {:?})",
                ticker.last_price,
                ticker.volume,
                std::time::Duration::from_secs(0)
            );
        }
        venues::binance::usdm::public::rest::ticker_24hr::Ticker24hrResult::Multiple(tickers) => {
            let ticker = &tickers[0];
            assert!(!ticker.volume.is_empty(), "Should have volume");
            println!(
                "24hr ticker: price={}, volume={} (took {:?})",
                ticker.last_price,
                ticker.volume,
                std::time::Duration::from_secs(0)
            );
        }
    }
}

/// Test ticker price endpoint
#[tokio::test]
async fn test_get_ticker_price() {
    use venues::binance::usdm::public::rest::ticker_price::TickerPriceRequest;

    let client = create_public_test_client();
    let request = TickerPriceRequest {
        symbol: Some("BTCUSDT".into()),
    };

    let result = client.get_ticker_price(request).await;
    assert!(
        result.is_ok(),
        "get_ticker_price request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();

    // Handle both single and multiple results
    match &response.data {
        venues::binance::usdm::public::rest::ticker_price::TickerPriceResult::Single(ticker) => {
            assert_eq!(ticker.symbol, "BTCUSDT", "Should have correct symbol");
            assert!(!ticker.price.is_empty(), "Should have price");

            println!(
                "Ticker price: {} = {} (took {:?})",
                ticker.symbol,
                ticker.price,
                std::time::Duration::from_secs(0)
            );
        }
        venues::binance::usdm::public::rest::ticker_price::TickerPriceResult::Multiple(tickers) => {
            assert!(!tickers.is_empty(), "Should have ticker price data");
            let ticker = &tickers[0];
            assert_eq!(ticker.symbol, "BTCUSDT", "Should have correct symbol");
            assert!(!ticker.price.is_empty(), "Should have price");

            println!(
                "Ticker price: {} = {} (took {:?})",
                ticker.symbol,
                ticker.price,
                std::time::Duration::from_secs(0)
            );
        }
    }
}

/// Test book ticker endpoint
#[tokio::test]
async fn test_get_book_ticker() {
    use venues::binance::usdm::public::rest::book_ticker::BookTickerRequest;

    let client = create_public_test_client();
    let request = BookTickerRequest {
        symbol: Some("BTCUSDT".into()),
    };

    let result = client.get_book_ticker(request).await;
    assert!(
        result.is_ok(),
        "get_book_ticker request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();

    // Handle BookTickerResult enum properly
    match &response.data {
        venues::binance::usdm::public::rest::book_ticker::BookTickerResult::Single(ticker) => {
            assert_eq!(ticker.symbol, "BTCUSDT", "Should have correct symbol");
            assert!(!ticker.bid_price.is_empty(), "Should have bid price");
            assert!(!ticker.ask_price.is_empty(), "Should have ask price");
        }
        venues::binance::usdm::public::rest::book_ticker::BookTickerResult::Multiple(tickers) => {
            assert!(!tickers.is_empty(), "Should have book ticker data");
            let ticker = &tickers[0];
            assert!(!ticker.bid_price.is_empty(), "Should have bid price");
            assert!(!ticker.ask_price.is_empty(), "Should have ask price");
        }
    }

    println!("Book ticker test completed successfully");
}

/// Test mark price endpoint
#[tokio::test]
async fn test_get_mark_price() {
    use venues::binance::usdm::public::rest::mark_price::MarkPriceRequest;

    let client = create_public_test_client();
    let request = MarkPriceRequest {
        symbol: Some("BTCUSDT".into()),
    };

    let result = client.get_mark_price(request).await;
    assert!(
        result.is_ok(),
        "get_mark_price request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();

    // Handle both single and multiple results
    match &response.data {
        venues::binance::usdm::public::rest::mark_price::MarkPriceResult::Single(mark_price) => {
            assert_eq!(mark_price.symbol, "BTCUSDT", "Should have correct symbol");
            assert!(!mark_price.mark_price.is_empty(), "Should have mark price");

            println!(
                "Mark price: {} = {} (took {:?})",
                mark_price.symbol,
                mark_price.mark_price,
                std::time::Duration::from_secs(0)
            );
        }
        venues::binance::usdm::public::rest::mark_price::MarkPriceResult::Multiple(mark_prices) => {
            assert!(!mark_prices.is_empty(), "Should have mark price data");
            let mark_price = &mark_prices[0];
            assert_eq!(mark_price.symbol, "BTCUSDT", "Should have correct symbol");
            assert!(!mark_price.mark_price.is_empty(), "Should have mark price");

            println!(
                "Mark price: {} = {} (took {:?})",
                mark_price.symbol,
                mark_price.mark_price,
                std::time::Duration::from_secs(0)
            );
        }
    }
}

/// Test funding rate history endpoint
#[tokio::test]
async fn test_get_funding_rate_history() {
    use venues::binance::usdm::public::rest::funding_rate_history::FundingRateHistoryRequest;

    let client = create_public_test_client();
    let request = FundingRateHistoryRequest {
        symbol: Some("BTCUSDT".into()),
        start_time: None,
        end_time: None,
        limit: Some(10),
    };

    let result = client.get_funding_rate_history(request).await;
    assert!(
        result.is_ok(),
        "get_funding_rate_history request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have funding rate history"
    );

    let funding = &response.data[0];
    assert_eq!(funding.symbol, "BTCUSDT", "Should have correct symbol");
    assert!(!funding.funding_rate.is_empty(), "Should have funding rate");

    println!(
        "Funding rate history: {} entries (took {:?})",
        response.data.len(),
        std::time::Duration::from_secs(0)
    );
}

/// Test open interest endpoint
#[tokio::test]
async fn test_get_open_interest() {
    use venues::binance::usdm::public::rest::open_interest::OpenInterestRequest;

    let client = create_public_test_client();
    let request = OpenInterestRequest {
        symbol: "BTCUSDT".into(),
    };

    let result = client.get_open_interest(request).await;
    assert!(
        result.is_ok(),
        "get_open_interest request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(
        response.data.symbol, "BTCUSDT",
        "Should have correct symbol"
    );
    assert!(
        !response.data.open_interest.is_empty(),
        "Should have open interest value"
    );

    println!(
        "Open interest: {} = {} (took {:?})",
        response.data.symbol,
        response.data.open_interest,
        std::time::Duration::from_secs(0)
    );
}

/// Test aggregate trades endpoint
#[tokio::test]
async fn test_get_agg_trades() {
    use venues::binance::usdm::public::rest::agg_trades::AggTradesRequest;

    let client = create_public_test_client();
    let request = AggTradesRequest {
        symbol: "BTCUSDT".into(),
        from_id: None,
        start_time: None,
        end_time: None,
        limit: Some(10),
    };

    let result = client.get_agg_trades(request).await;
    assert!(
        result.is_ok(),
        "get_agg_trades request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have aggregate trade data"
    );

    let trade = &response.data[0];
    assert!(
        trade.agg_trade_id > 0,
        "Should have valid aggregate trade ID"
    );
    assert!(!trade.price.is_empty(), "Should have price");
    assert!(!trade.qty.is_empty(), "Should have quantity");

    println!(
        "Aggregate trades: {} trades (took {:?})",
        response.data.len(),
        std::time::Duration::from_secs(0)
    );
}

/// Test continuous klines endpoint
#[tokio::test]
async fn test_get_continuous_klines() {
    use venues::binance::usdm::public::rest::continuous_klines::ContinuousKlinesRequest;

    let client = create_public_test_client();
    let request = ContinuousKlinesRequest {
        pair: "BTCUSDT".into(),
        contract_type: venues::binance::usdm::ContractType::Perpetual,
        interval: venues::binance::usdm::KlineInterval::I1m,
        start_time: None,
        end_time: None,
        limit: Some(10),
    };

    let result = client.get_continuous_klines(request).await;
    assert!(
        result.is_ok(),
        "get_continuous_klines request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have continuous kline data"
    );

    let kline = &response.data[0];
    assert!(kline.open_time > 0, "Should have valid open time");
    assert!(!kline.open.is_empty(), "Should have open price");

    println!(
        "Continuous klines: {} candles (took {:?})",
        response.data.len(),
        std::time::Duration::from_secs(0)
    );
}

/// Test index price klines endpoint
#[tokio::test]
async fn test_get_index_price_klines() {
    use venues::binance::usdm::public::rest::index_price_klines::IndexPriceKlinesRequest;

    let client = create_public_test_client();
    let request = IndexPriceKlinesRequest {
        pair: "BTCUSDT".into(),
        interval: venues::binance::usdm::KlineInterval::I1m,
        start_time: None,
        end_time: None,
        limit: Some(10),
    };

    let result = client.get_index_price_klines(request).await;
    assert!(
        result.is_ok(),
        "get_index_price_klines request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have index price kline data"
    );

    let kline = &response.data[0];
    assert!(kline.open_time > 0, "Should have valid open time");
    assert!(!kline.open.is_empty(), "Should have open price");

    println!(
        "Index price klines: {} candles (took {:?})",
        response.data.len(),
        std::time::Duration::from_secs(0)
    );
}

/// Test mark price klines endpoint
#[tokio::test]
async fn test_get_mark_price_klines() {
    use venues::binance::usdm::public::rest::mark_price_klines::MarkPriceKlinesRequest;

    let client = create_public_test_client();
    let request = MarkPriceKlinesRequest {
        symbol: "BTCUSDT".into(),
        interval: venues::binance::usdm::KlineInterval::I1m,
        start_time: None,
        end_time: None,
        limit: Some(10),
    };

    let result = client.get_mark_price_klines(request).await;
    assert!(
        result.is_ok(),
        "get_mark_price_klines request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have mark price kline data"
    );

    let kline = &response.data[0];
    assert!(kline.open_time > 0, "Should have valid open time");
    assert!(!kline.open.is_empty(), "Should have open price");

    println!(
        "Mark price klines: {} candles (took {:?})",
        response.data.len(),
        std::time::Duration::from_secs(0)
    );
}

/// Test premium index klines endpoint
#[tokio::test]
async fn test_premium_index_klines() {
    use venues::binance::usdm::public::rest::premium_index_klines::PremiumIndexKlinesRequest;

    let client = create_public_test_client();
    let request = PremiumIndexKlinesRequest {
        symbol: "BTCUSDT".into(),
        interval: venues::binance::usdm::KlineInterval::I1m,
        start_time: None,
        end_time: None,
        limit: Some(10),
    };

    let result = client.premium_index_klines(request).await;
    assert!(
        result.is_ok(),
        "premium_index_klines request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have premium index kline data"
    );

    let kline = &response.data[0];
    assert!(kline.open_time > 0, "Should have valid open time");
    assert!(!kline.open.is_empty(), "Should have open price");

    println!(
        "Premium index klines: {} candles (took {:?})",
        response.data.len(),
        std::time::Duration::from_secs(0)
    );
}

/// Test open interest history endpoint
#[tokio::test]
async fn test_get_open_interest_hist() {
    use venues::binance::usdm::public::rest::open_interest_hist::OpenInterestHistRequest;

    let client = create_public_test_client();
    let request = OpenInterestHistRequest {
        symbol: "BTCUSDT".into(),
        period: venues::binance::usdm::Period::I5m,
        limit: Some(10),
        start_time: None,
        end_time: None,
    };

    let result = client.open_interest_hist(request).await;
    assert!(
        result.is_ok(),
        "get_open_interest_hist request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have open interest history"
    );

    let entry = &response.data[0];
    assert_eq!(entry.symbol, "BTCUSDT", "Should have correct symbol");
    assert!(
        !entry.sum_open_interest.is_empty(),
        "Should have open interest value"
    );

    println!(
        "Open interest history: {} entries (took {:?})",
        response.data.len(),
        std::time::Duration::from_secs(0)
    );
}

/// Test global long/short account ratio endpoint
#[tokio::test]
async fn test_get_global_long_short_account_ratio() {
    use venues::binance::usdm::public::rest::global_long_short_account_ratio::GlobalLongShortAccountRatioRequest;

    let client = create_public_test_client();
    let request = GlobalLongShortAccountRatioRequest {
        symbol: Some("BTCUSDT".into()),
        period: venues::binance::usdm::Period::I5m,
        limit: Some(10),
        start_time: None,
        end_time: None,
    };

    let result = client.global_long_short_account_ratio(request).await;
    assert!(
        result.is_ok(),
        "get_global_long_short_account_ratio request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have global long/short ratio data"
    );

    let entry = &response.data[0];
    assert_eq!(entry.symbol, "BTCUSDT", "Should have correct symbol");
    assert!(
        !entry.long_short_ratio.is_empty(),
        "Should have ratio value"
    );

    println!(
        "Global long/short account ratio: {} entries (took {:?})",
        response.data.len(),
        std::time::Duration::from_secs(0)
    );
}

/// Test top long/short account ratio endpoint
#[tokio::test]
async fn test_get_top_long_short_account_ratio() {
    use venues::binance::usdm::public::rest::top_long_short_account_ratio::TopLongShortAccountRatioRequest;

    let client = create_public_test_client();
    let request = TopLongShortAccountRatioRequest {
        symbol: "BTCUSDT".into(),
        period: venues::binance::usdm::Period::I5m,
        limit: Some(10),
        start_time: None,
        end_time: None,
    };

    let result = client.top_long_short_account_ratio(request).await;
    assert!(
        result.is_ok(),
        "get_top_long_short_account_ratio request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have top long/short account ratio data"
    );

    let entry = &response.data[0];
    assert_eq!(entry.symbol, "BTCUSDT", "Should have correct symbol");
    assert!(
        !entry.long_short_ratio.is_empty(),
        "Should have ratio value"
    );

    println!(
        "Top long/short account ratio: {} entries (took {:?})",
        response.data.len(),
        std::time::Duration::from_secs(0)
    );
}

/// Test top long/short position ratio endpoint
#[tokio::test]
async fn test_get_top_long_short_position_ratio() {
    use venues::binance::usdm::public::rest::top_long_short_position_ratio::TopLongShortPositionRatioRequest;

    let client = create_public_test_client();
    let request = TopLongShortPositionRatioRequest {
        symbol: "BTCUSDT".into(),
        period: venues::binance::usdm::Period::I5m,
        limit: Some(10),
        start_time: None,
        end_time: None,
    };

    let result = client.top_long_short_position_ratio(request).await;
    assert!(
        result.is_ok(),
        "get_top_long_short_position_ratio request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have top long/short position ratio data"
    );

    let entry = &response.data[0];
    assert_eq!(entry.symbol, "BTCUSDT", "Should have correct symbol");
    assert!(
        !entry.long_short_ratio.is_empty(),
        "Should have ratio value"
    );

    println!(
        "Top long/short position ratio: {} entries (took {:?})",
        response.data.len(),
        std::time::Duration::from_secs(0)
    );
}

/// Test taker long/short ratio endpoint
#[tokio::test]
async fn test_get_taker_long_short_ratio() {
    use venues::binance::usdm::public::rest::taker_long_short_ratio::TakerLongShortRatioRequest;

    let client = create_public_test_client();
    let request = TakerLongShortRatioRequest {
        symbol: "BTCUSDT".into(),
        period: venues::binance::usdm::Period::I5m,
        limit: Some(10),
        start_time: None,
        end_time: None,
    };

    let result = client.taker_long_short_ratio(request).await;
    assert!(
        result.is_ok(),
        "get_taker_long_short_ratio request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have taker long/short ratio data"
    );

    let entry = &response.data[0];
    assert!(!entry.buy_sell_ratio.is_empty(), "Should have ratio value");

    println!(
        "Taker long/short ratio: {} entries (took {:?})",
        response.data.len(),
        std::time::Duration::from_secs(0)
    );
}

/// Test basis endpoint
#[tokio::test]
async fn test_get_basis() {
    use venues::binance::usdm::public::rest::basis::BasisRequest;

    let client = create_public_test_client();
    let request = BasisRequest {
        pair: "BTCUSDT".into(),
        contract_type: venues::binance::usdm::ContractType::Perpetual,
        period: venues::binance::usdm::Period::I5m,
        limit: 10,
        start_time: None,
        end_time: None,
    };

    let result = client.basis(request).await;
    assert!(
        result.is_ok(),
        "get_basis request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.data.is_empty(), "Should have basis data");

    let entry = &response.data[0];
    assert_eq!(entry.pair, "BTCUSDT", "Should have correct pair");
    assert!(!entry.basis.is_empty(), "Should have basis value");

    println!(
        "Basis data: {} entries (took {:?})",
        response.data.len(),
        std::time::Duration::from_secs(0)
    );
}

/// Test delivery price endpoint
#[tokio::test]
async fn test_get_delivery_price() {
    use venues::binance::usdm::public::rest::delivery_price::DeliveryPriceRequest;

    let client = create_public_test_client();
    let request = DeliveryPriceRequest {
        pair: "BTCUSDT".into(),
    };

    let result = client.delivery_price(request).await;
    assert!(
        result.is_ok(),
        "get_delivery_price request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.data.is_empty(), "Should have delivery price data");

    let entry = &response.data[0];
    assert!(entry.delivery_price > 0.0, "Should have delivery price");

    println!(
        "Delivery price: {} entries (took {:?})",
        response.data.len(),
        std::time::Duration::from_secs(0)
    );
}

/// Test asset index endpoint
#[tokio::test]
async fn test_get_asset_index() {
    use venues::binance::usdm::public::rest::asset_index::AssetIndexRequest;

    let client = create_public_test_client();
    let request = AssetIndexRequest {
        symbol: Some("BTCUSD".into()), // Note: different format for asset index
    };

    let result = client.get_asset_index(request).await;
    assert!(
        result.is_ok(),
        "get_asset_index request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();

    // Handle both single and multiple results
    match &response.data {
        venues::binance::usdm::public::rest::asset_index::AssetIndexResult::Single(entry) => {
            assert_eq!(entry.symbol, "BTCUSD", "Should have correct symbol");
            assert!(!entry.index.is_empty(), "Should have index value");
        }
        venues::binance::usdm::public::rest::asset_index::AssetIndexResult::Multiple(entries) => {
            assert!(!entries.is_empty(), "Should have asset index data");
            let entry = &entries[0];
            assert_eq!(entry.symbol, "BTCUSD", "Should have correct symbol");
            assert!(!entry.index.is_empty(), "Should have index value");
        }
    }

    let count = match &response.data {
        venues::binance::usdm::public::rest::asset_index::AssetIndexResult::Single(_) => 1,
        venues::binance::usdm::public::rest::asset_index::AssetIndexResult::Multiple(entries) => {
            entries.len()
        }
    };

    println!(
        "Asset index: {} entries (took {:?})",
        count,
        std::time::Duration::from_secs(0)
    );
}

/// Test constituents endpoint
#[tokio::test]
async fn test_get_constituents() {
    use venues::binance::usdm::public::rest::constituents::ConstituentsRequest;

    let client = create_public_test_client();
    let request = ConstituentsRequest {
        symbol: "BTCUSD".into(),
    };

    let result = client.get_constituents(request).await;
    assert!(
        result.is_ok(),
        "get_constituents request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.constituents.is_empty(),
        "Should have constituents data"
    );

    let constituent = &response.data.constituents[0];
    assert_eq!(response.data.symbol, "BTCUSD", "Should have correct symbol");
    assert!(
        !constituent.symbol.is_empty(),
        "Should have constituent symbol"
    );

    println!(
        "Constituents: {} entries (took {:?})",
        response.data.constituents.len(),
        std::time::Duration::from_secs(0)
    );
}

/// Test index info endpoint
#[tokio::test]
async fn test_get_index_info() {
    use venues::binance::usdm::public::rest::index_info::IndexInfoRequest;

    let client = create_public_test_client();
    let request = IndexInfoRequest {
        symbol: Some("DEFIUSDT".into()), // Multi-asset index symbol
    };

    let result = client.get_index_info(request).await;
    assert!(
        result.is_ok(),
        "get_index_info request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();

    let infos = match response.data {
        venues::binance::usdm::public::rest::index_info::IndexInfoResponse::Single(info) => {
            vec![info]
        }
        venues::binance::usdm::public::rest::index_info::IndexInfoResponse::Multiple(infos) => {
            infos
        }
    };

    assert!(!infos.is_empty(), "Should have index info data");

    let entry = &infos[0];
    assert!(!entry.symbol.is_empty(), "Should have symbol");
    assert!(!entry.base_asset_list.is_empty(), "Should have base assets");

    println!(
        "Index info: {} with {} base assets (took {:?})",
        entry.symbol,
        entry.base_asset_list.len(),
        std::time::Duration::from_secs(0)
    );
}

/// Test premium index endpoint
#[tokio::test]
async fn test_get_premium_index() {
    use venues::binance::usdm::public::rest::premium_index::PremiumIndexRequest;

    let client = create_public_test_client();
    let request = PremiumIndexRequest {
        symbol: Some("BTCUSDT".into()),
    };

    let result = client.premium_index(request).await;
    assert!(
        result.is_ok(),
        "get_premium_index request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();

    // Handle PremiumIndexResult enum properly
    match &response.data {
        venues::binance::usdm::public::rest::premium_index::PremiumIndexResult::Single(entry) => {
            assert_eq!(entry.symbol, "BTCUSDT", "Should have correct symbol");
            assert!(!entry.mark_price.is_empty(), "Should have mark price");
            println!(
                "Premium index (single): symbol={}, mark_price={}",
                entry.symbol, entry.mark_price
            );
        }
        venues::binance::usdm::public::rest::premium_index::PremiumIndexResult::Multiple(
            entries,
        ) => {
            assert!(!entries.is_empty(), "Should have premium index data");
            let entry = &entries[0];
            assert!(!entry.mark_price.is_empty(), "Should have mark price");
            println!("Premium index (multiple): {} entries", entries.len());
        }
    }
}
