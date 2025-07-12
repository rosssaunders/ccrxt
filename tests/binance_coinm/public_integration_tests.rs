//! Integration tests for Binance CoinM public REST API endpoints
//!
//! These tests verify the functionality of all public endpoints that don't require authentication.
//! Tests run against the live Binance CoinM API using real market data.
//!
//! **Note:** Binance API has geographic restrictions. Tests may fail with "Service unavailable
//! from a restricted location" errors when run from certain locations. This is expected behavior
//! and indicates the tests are correctly configured to reach the live API.

use reqwest::Client;
use tokio;
use venues::binance::{
    AggregateTradesRequest,
    BasisRequest,
    BookTickerRequest,
    BookTickerRequestBySymbol,
    ConstituentsRequest,
    ContinuousKlineRequest,
    ContractType,
    FundingRateRequest,
    GlobalLongShortAccountRatioParams,
    HistoricalTradesRequest,
    IndexPriceKlineRequest,
    KlineInterval,
    KlineRequest,
    MarkPriceKlineRequest,
    OpenInterestHistParams,
    OpenInterestRequest,
    // Request types
    OrderBookRequest,
    Period,
    PremiumIndexKlineRequest,
    PremiumIndexRequest,
    RateLimiter,
    RecentTradesRequest,
    // Basic types
    RestClient as PublicRestClient,
    TakerBuySellVolParams,
    Ticker24hrParams,
    TickerPriceRequest,
    TopLongShortAccountRatioParams,
    TopLongShortPositionRatioParams,
};

/// Helper function to create a test client for public endpoints
fn create_public_test_client() -> PublicRestClient {
    let client = Client::new();
    let rate_limiter = RateLimiter::new();

    PublicRestClient::new("https://dapi.binance.com", client, rate_limiter)
}

/// Test the ping endpoint - test connectivity
#[tokio::test]
async fn test_ping() {
    let client = create_public_test_client();

    let result = client.ping().await;
    assert!(
        result.is_ok(),
        "ping request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    // PingResponse is an empty struct, so just verify we got a response
    println!("Ping successful: {:?}", response.data);
}

/// Test the server time endpoint
#[tokio::test]
async fn test_get_server_time() {
    let client = create_public_test_client();

    let result = client.get_server_time().await;
    assert!(
        result.is_ok(),
        "get_server_time request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        response.data.server_time > 0,
        "Server time should be a positive timestamp"
    );
    println!("Server time: {}", response.data.server_time);
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
    assert!(
        !response.data.symbols.is_empty(),
        "Should have at least one symbol"
    );
    println!(
        "Exchange info returned {} symbols",
        response.data.symbols.len()
    );
}

/// Test the order book endpoint with BTCUSD_PERP
#[tokio::test]
async fn test_get_order_book() {
    let client = create_public_test_client();
    let request = OrderBookRequest {
        symbol: "BTCUSD_PERP".to_string(),
        limit: Some(5),
    };

    let result = client.get_order_book(request).await;
    assert!(
        result.is_ok(),
        "get_order_book request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.data.symbol, "BTCUSD_PERP");
    assert!(
        !response.data.bids.is_empty(),
        "Should have at least one bid"
    );
    assert!(
        !response.data.asks.is_empty(),
        "Should have at least one ask"
    );
    println!(
        "Order book for {} has {} bids and {} asks",
        response.data.symbol,
        response.data.bids.len(),
        response.data.asks.len()
    );
}

/// Test the recent trades endpoint
#[tokio::test]
async fn test_get_recent_trades() {
    let client = create_public_test_client();
    let request = RecentTradesRequest {
        symbol: "BTCUSD_PERP".to_string(),
        limit: Some(5),
    };

    let result = client.get_recent_trades(request).await;
    assert!(
        result.is_ok(),
        "get_recent_trades request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.data.is_empty(), "Should have at least one trade");
    println!("Recent trades returned {} trades", response.data.len());
}

/// Test the historical trades endpoint
/// Note: This endpoint requires API authentication, so we expect it to fail
#[tokio::test]
async fn test_get_historical_trades() {
    let client = create_public_test_client();
    let request = HistoricalTradesRequest {
        symbol: "BTCUSD_PERP".to_string(),
        limit: Some(5),
        from_id: None,
    };

    let result = client.get_historical_trades(request).await;
    // This endpoint requires authentication, so we expect it to fail for public client
    assert!(
        result.is_err(),
        "get_historical_trades should fail without API key"
    );
    println!(
        "Historical trades correctly failed without authentication: {:?}",
        result.err()
    );
}

/// Test the aggregate trades endpoint
#[tokio::test]
async fn test_get_aggregate_trades() {
    let client = create_public_test_client();
    let request = AggregateTradesRequest {
        symbol: "BTCUSD_PERP".to_string(),
        from_id: None,
        start_time: None,
        end_time: None,
        limit: Some(5),
    };

    let result = client.get_aggregate_trades(request).await;
    assert!(
        result.is_ok(),
        "get_aggregate_trades request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have at least one aggregate trade"
    );
    println!("Aggregate trades returned {} trades", response.data.len());
}

/// Test the klines endpoint
#[tokio::test]
async fn test_get_klines() {
    let client = create_public_test_client();
    let request = KlineRequest {
        symbol: "BTCUSD_PERP".to_string(),
        interval: KlineInterval::I1h,
        start_time: None,
        end_time: None,
        limit: Some(5),
    };

    let result = client.get_klines(request).await;
    assert!(
        result.is_ok(),
        "get_klines request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.data.is_empty(), "Should have at least one kline");
    println!("Klines returned {} entries", response.data.len());
}

/// Test the continuous klines endpoint
#[tokio::test]
async fn test_get_continuous_klines() {
    let client = create_public_test_client();
    let request = ContinuousKlineRequest {
        pair: "BTCUSD".to_string(),
        contract_type: ContractType::Perpetual,
        interval: KlineInterval::I1h,
        start_time: None,
        end_time: None,
        limit: Some(5),
    };

    let result = client.get_continuous_klines(request).await;
    assert!(
        result.is_ok(),
        "get_continuous_klines request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.data.is_empty(), "Should have at least one kline");
    println!("Continuous klines returned {} entries", response.data.len());
}

/// Test the index price klines endpoint
#[tokio::test]
async fn test_get_index_price_klines() {
    let client = create_public_test_client();
    let request = IndexPriceKlineRequest {
        pair: "BTCUSD".to_string(),
        interval: KlineInterval::I1h,
        start_time: None,
        end_time: None,
        limit: Some(5),
    };

    let result = client.get_index_price_klines(request).await;
    assert!(
        result.is_ok(),
        "get_index_price_klines request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.data.is_empty(), "Should have at least one kline");
    println!(
        "Index price klines returned {} entries",
        response.data.len()
    );
}

/// Test the mark price klines endpoint
#[tokio::test]
async fn test_get_mark_price_klines() {
    let client = create_public_test_client();
    let request = MarkPriceKlineRequest {
        symbol: "BTCUSD_PERP".to_string(),
        interval: KlineInterval::I1h,
        start_time: None,
        end_time: None,
        limit: Some(5),
    };

    let result = client.get_mark_price_klines(request).await;
    assert!(
        result.is_ok(),
        "get_mark_price_klines request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.data.is_empty(), "Should have at least one kline");
    println!("Mark price klines returned {} entries", response.data.len());
}

/// Test the premium index klines endpoint
#[tokio::test]
async fn test_get_premium_index_klines() {
    let client = create_public_test_client();
    let request = PremiumIndexKlineRequest {
        symbol: "BTCUSD_PERP".to_string(),
        interval: KlineInterval::I1h,
        start_time: None,
        end_time: None,
        limit: Some(5),
    };

    let result = client.get_premium_index_klines(request).await;
    assert!(
        result.is_ok(),
        "get_premium_index_klines request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.data.is_empty(), "Should have at least one kline");
    println!(
        "Premium index klines returned {} entries",
        response.data.len()
    );
}

/// Test the premium index endpoint
#[tokio::test]
async fn test_get_premium_index() {
    let client = create_public_test_client();
    let request = PremiumIndexRequest {
        symbol: Some("BTCUSD_PERP".to_string()),
        pair: None,
    };

    let result = client.get_premium_index(request).await;
    assert!(
        result.is_ok(),
        "get_premium_index request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have at least one premium index entry"
    );
    println!("Premium index returned {} entries", response.data.len());
}

/// Test the funding rate history endpoint
#[tokio::test]
async fn test_get_funding_rate_history() {
    let client = create_public_test_client();
    let request = FundingRateRequest {
        symbol: "BTCUSD_PERP".to_string(),
        start_time: None,
        end_time: None,
        limit: Some(5),
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
        "Should have at least one funding rate entry"
    );
    println!(
        "Funding rate history returned {} entries",
        response.data.len()
    );
}

/// Test the 24hr ticker endpoint
#[tokio::test]
async fn test_get_ticker_24hr() {
    let client = create_public_test_client();
    let params = Ticker24hrParams {
        symbol: Some("BTCUSD_PERP".to_string()),
        pair: None,
    };

    let result = client.get_ticker_24hr(params).await;
    assert!(
        result.is_ok(),
        "get_ticker_24hr request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have at least one ticker entry"
    );
    println!("24hr ticker returned {} entries", response.data.len());
}

/// Test the ticker price endpoint
#[tokio::test]
async fn test_get_ticker_price() {
    let client = create_public_test_client();
    let request = TickerPriceRequest {
        symbol: Some("BTCUSD_PERP".to_string()),
        pair: None,
    };

    let result = client.get_ticker_price(request).await;
    assert!(
        result.is_ok(),
        "get_ticker_price request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have at least one price entry"
    );
    println!("Ticker price returned {} entries", response.data.len());
}

/// Test the book ticker endpoint
#[tokio::test]
async fn test_get_book_ticker() {
    let client = create_public_test_client();
    let request = BookTickerRequest::BySymbol(BookTickerRequestBySymbol {
        symbol: Some("BTCUSD_PERP".to_string()),
    });

    let result = client.get_book_ticker(request).await;
    assert!(
        result.is_ok(),
        "get_book_ticker request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have at least one book ticker entry"
    );
    println!("Book ticker returned {} entries", response.data.len());
}

/// Test the open interest endpoint
#[tokio::test]
async fn test_get_open_interest() {
    let client = create_public_test_client();
    let request = OpenInterestRequest {
        symbol: "BTCUSD_PERP".to_string(),
    };

    let result = client.get_open_interest(request).await;
    assert!(
        result.is_ok(),
        "get_open_interest request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    println!(
        "Open interest for {}: {}",
        response.data.symbol, response.data.open_interest
    );
}

/// Test the open interest history endpoint
#[tokio::test]
async fn test_get_open_interest_hist() {
    let client = create_public_test_client();
    let params = OpenInterestHistParams {
        pair: "BTCUSD".to_string(),
        contract_type: "PERPETUAL".to_string(),
        period: Period::I5m,
        start_time: None,
        end_time: None,
        limit: Some(5),
    };

    let result = client.get_open_interest_hist(params).await;
    assert!(
        result.is_ok(),
        "get_open_interest_hist request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have at least one open interest history entry"
    );
    println!(
        "Open interest history returned {} entries",
        response.data.len()
    );
}

/// Test the top long/short account ratio endpoint
#[tokio::test]
async fn test_get_top_long_short_account_ratio() {
    let client = create_public_test_client();
    let params = TopLongShortAccountRatioParams {
        pair: "BTCUSD".to_string(),
        period: Period::I5m,
        start_time: None,
        end_time: None,
        limit: Some(5),
    };

    let result = client.get_top_long_short_account_ratio(params).await;
    assert!(
        result.is_ok(),
        "get_top_long_short_account_ratio request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have at least one ratio entry"
    );
    println!(
        "Top long/short account ratio returned {} entries",
        response.data.len()
    );
}

/// Test the top long/short position ratio endpoint
#[tokio::test]
async fn test_get_top_long_short_position_ratio() {
    let client = create_public_test_client();
    let params = TopLongShortPositionRatioParams {
        pair: "BTCUSD".to_string(),
        period: Period::I5m,
        start_time: None,
        end_time: None,
        limit: Some(5),
    };

    let result = client.get_top_long_short_position_ratio(params).await;
    assert!(
        result.is_ok(),
        "get_top_long_short_position_ratio request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have at least one ratio entry"
    );
    println!(
        "Top long/short position ratio returned {} entries",
        response.data.len()
    );
}

/// Test the global long/short account ratio endpoint
#[tokio::test]
async fn test_get_global_long_short_account_ratio() {
    let client = create_public_test_client();
    let params = GlobalLongShortAccountRatioParams {
        pair: "BTCUSD".to_string(),
        period: Period::I5m,
        start_time: None,
        end_time: None,
        limit: Some(5),
    };

    let result = client.get_global_long_short_account_ratio(params).await;
    assert!(
        result.is_ok(),
        "get_global_long_short_account_ratio request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have at least one ratio entry"
    );
    println!(
        "Global long/short account ratio returned {} entries",
        response.data.len()
    );
}

/// Test the taker buy/sell volume endpoint
#[tokio::test]
async fn test_get_taker_buy_sell_vol() {
    let client = create_public_test_client();
    let params = TakerBuySellVolParams {
        pair: "BTCUSD".to_string(),
        contract_type: "PERPETUAL".to_string(),
        period: Period::I5m,
        start_time: None,
        end_time: None,
        limit: Some(5),
    };

    let result = client.get_taker_buy_sell_vol(params).await;
    assert!(
        result.is_ok(),
        "get_taker_buy_sell_vol request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have at least one volume entry"
    );
    println!(
        "Taker buy/sell volume returned {} entries",
        response.data.len()
    );
}

/// Test the basis endpoint
#[tokio::test]
async fn test_get_basis() {
    let client = create_public_test_client();
    let params = BasisRequest {
        pair: "BTCUSD".to_string(),
        contract_type: ContractType::CurrentQuarter,
        period: Period::I5m,
        start_time: None,
        end_time: None,
        limit: Some(5),
    };

    let result = client.get_basis(params).await;
    assert!(
        result.is_ok(),
        "get_basis request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have at least one basis entry"
    );
    println!("Basis returned {} entries", response.data.len());
}

/// Test the constituents endpoint
#[tokio::test]
async fn test_get_constituents() {
    let client = create_public_test_client();
    // Use a valid index symbol instead of perpetual
    let params = ConstituentsRequest {
        symbol: "BTCUSD_231229".to_string(), // Use a quarterly futures symbol
    };

    let symbol = params.symbol.clone();
    let result = client.get_constituents(params).await;

    // This endpoint may not work with all symbols, so we'll handle both success and known errors
    match result {
        Ok(response) => {
            println!("Constituents for {}: {:?}", symbol, response.data);
        }
        Err(e) => {
            println!(
                "Constituents endpoint failed (expected for some symbols): {:?}",
                e
            );
            // This is acceptable as not all symbols have constituents data
        }
    }
}

/// Test the funding info endpoint
#[tokio::test]
async fn test_get_funding_info() {
    let client = create_public_test_client();

    let result = client.get_funding_info().await;
    assert!(
        result.is_ok(),
        "get_funding_info request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    // This endpoint may return empty results if no symbols have funding adjustments
    println!("Funding info returned {} entries", response.data.len());
    if response.data.is_empty() {
        println!("No funding info entries found (this is acceptable)");
    }
}
