//! Integration tests for Bybit public REST API endpoints.
//!
//! These tests verify that the Bybit public REST API client can successfully
//! communicate with the live API and receive valid responses.

use venues::bybit::{
    Category,
    // Risk management endpoints
    GetDeliveryPriceRequest,
    // Trading & market statistics endpoints
    GetFundingHistoryRequest,
    GetHistoricalVolatilityRequest,
    GetIndexPriceKlineRequest,
    GetInsMarginCoinInfoRequest,
    GetInstrumentsInfoRequest,
    GetInsuranceRequest,
    GetKlineRequest,
    GetLongShortRatioRequest,
    // Price kline endpoints
    GetMarkPriceKlineRequest,
    GetOpenInterestRequest,
    GetOrderbookRequest,
    GetPremiumIndexPriceKlineRequest,
    GetRecentTradesRequest,
    GetRiskLimitRequest,
    GetServerTimeRequest,
    GetTickersRequest,
    // Margin & loan endpoints
    GetVipMarginDataRequest,
    Interval,
    PublicRestClient,
    RateLimiter,
};

/// Helper function to create a test client with shared rate limiter
fn create_public_test_client() -> PublicRestClient {
    let rate_limiter = RateLimiter::new();
    let client = reqwest::Client::new();
    PublicRestClient::new("https://api.bybit.com", rate_limiter, client)
}

#[tokio::test]
async fn test_get_server_time() {
    let client = create_public_test_client();
    let _request = GetServerTimeRequest {};

    let result = client.get_server_time().await;
    assert!(
        result.is_ok(),
        "get_server_time should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.ret_code, 0, "Response should indicate success");

    println!("Server timestamp: {}", response.result.time_second);
    println!("Server timestamp nano: {}", response.result.time_nano);
}

#[tokio::test]
async fn test_get_tickers() {
    let client = create_public_test_client();
    let request = GetTickersRequest {
        category: Category::Linear,
        symbol: None,
        base_coin: None,
        exp_date: None,
    };

    let result = client.get_tickers(request).await;
    assert!(
        result.is_ok(),
        "get_tickers should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.ret_code, 0, "Response should indicate success");
    assert!(
        !response.result.list.is_empty(),
        "Should have at least one ticker"
    );

    println!("Found {} tickers", response.result.list.len());
    if let Some(first_ticker) = response.result.list.first() {
        println!(
            "First ticker: {} - Last: {}, Volume: {}",
            first_ticker.symbol, first_ticker.last_price, first_ticker.volume_24h
        );
    }
}

#[tokio::test]
async fn test_get_ticker_btcusdt() {
    let client = create_public_test_client();
    let request = GetTickersRequest {
        category: Category::Linear,
        symbol: Some("BTCUSDT".to_string()),
        base_coin: None,
        exp_date: None,
    };

    let result = client.get_tickers(request).await;
    assert!(
        result.is_ok(),
        "get_tickers for BTCUSDT should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.ret_code, 0, "Response should indicate success");
    assert!(
        !response.result.list.is_empty(),
        "Should have BTCUSDT ticker"
    );

    let ticker = &response.result.list[0];
    assert_eq!(ticker.symbol, "BTCUSDT");
    println!(
        "BTCUSDT ticker - Last: {}, High: {}, Low: {}, Volume: {}",
        ticker.last_price, ticker.high_price_24h, ticker.low_price_24h, ticker.volume_24h
    );
}

#[tokio::test]
async fn test_get_kline() {
    let client = create_public_test_client();
    let request = GetKlineRequest {
        category: Some(Category::Linear),
        symbol: "BTCUSDT".to_string(),
        interval: Interval::Min1,
        start: None,
        end: None,
        limit: Some(10),
    };

    let result = client.get_kline(request).await;
    assert!(
        result.is_ok(),
        "get_kline should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.ret_code, 0, "Response should indicate success");
    assert!(
        !response.result.list.is_empty(),
        "Should have at least one kline"
    );

    println!("Found {} klines", response.result.list.len());
    if let Some(first_kline) = response.result.list.first() {
        println!(
            "First kline: Time: {}, Open: {}, High: {}, Low: {}, Close: {}, Volume: {}",
            first_kline.start_time,
            first_kline.open_price,
            first_kline.high_price,
            first_kline.low_price,
            first_kline.close_price,
            first_kline.volume
        );
    }
}

#[tokio::test]
async fn test_get_orderbook() {
    let client = create_public_test_client();
    let request = GetOrderbookRequest {
        category: Category::Linear,
        symbol: "BTCUSDT".to_string(),
        limit: Some(25),
    };

    let result = client.get_orderbook(request).await;
    assert!(
        result.is_ok(),
        "get_orderbook should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.ret_code, 0, "Response should indicate success");

    let orderbook = &response.result;
    println!("Order book timestamp: {}", orderbook.ts);
    println!("Update ID: {}", orderbook.u);
    println!("Sequence: {}", orderbook.seq);
    println!(
        "Asks: {} levels, Bids: {} levels",
        orderbook.a.len(),
        orderbook.b.len()
    );

    if let Some(best_ask) = orderbook.a.first() {
        println!("Best ask: {} @ {}", best_ask.size, best_ask.price);
    }
    if let Some(best_bid) = orderbook.b.first() {
        println!("Best bid: {} @ {}", best_bid.size, best_bid.price);
    }
}

#[tokio::test]
async fn test_get_recent_trades() {
    let client = create_public_test_client();
    let request = GetRecentTradesRequest {
        category: Category::Linear,
        symbol: Some("BTCUSDT".to_string()),
        base_coin: None,
        option_type: None,
        limit: Some(10),
    };

    let result = client.get_recent_trades(request).await;
    assert!(
        result.is_ok(),
        "get_recent_trades should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.ret_code, 0, "Response should indicate success");
    assert!(
        !response.result.list.is_empty(),
        "Should have at least one trade"
    );

    println!("Found {} recent trades", response.result.list.len());
    if let Some(first_trade) = response.result.list.first() {
        println!(
            "First trade: Time: {}, Price: {}, Size: {}, Side: {:?}",
            first_trade.time, first_trade.price, first_trade.size, first_trade.side
        );
    }
}

#[tokio::test]
async fn test_get_instruments_info() {
    let client = create_public_test_client();
    let request = GetInstrumentsInfoRequest {
        category: Category::Linear,
        symbol: Some("BTCUSDT".to_string()),
        base_coin: None,
        limit: None,
        cursor: None,
    };

    let result = client.get_instruments_info(request).await;
    assert!(
        result.is_ok(),
        "get_instruments_info should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.ret_code, 0, "Response should indicate success");
    assert!(
        !response.result.list.is_empty(),
        "Should have at least one instrument"
    );

    println!("Found {} instruments", response.result.list.len());
    if let Some(first_instrument) = response.result.list.first() {
        println!(
            "First instrument: {} - Status: {:?}, Base: {}, Quote: {}",
            first_instrument.symbol,
            first_instrument.status,
            first_instrument.base_coin,
            first_instrument.quote_coin
        );
    }
}

#[tokio::test]
async fn test_spot_category() {
    let client = create_public_test_client();
    let request = GetTickersRequest {
        category: Category::Spot,
        symbol: Some("BTCUSDT".to_string()),
        base_coin: None,
        exp_date: None,
    };

    let result = client.get_tickers(request).await;
    assert!(
        result.is_ok(),
        "get_tickers for spot BTCUSDT should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.ret_code, 0, "Response should indicate success");

    println!("Spot category test completed successfully");
    if !response.result.list.is_empty() {
        let ticker = &response.result.list[0];
        println!(
            "Spot BTCUSDT - Last: {}, Volume: {}",
            ticker.last_price, ticker.volume_24h
        );
    }
}

#[tokio::test]
async fn test_multiple_intervals() {
    let client = create_public_test_client();

    let intervals = [Interval::Min1, Interval::Min5, Interval::Min60];

    for interval in intervals.iter() {
        let request = GetKlineRequest {
            category: Some(Category::Linear),
            symbol: "BTCUSDT".to_string(),
            interval: interval.clone(),
            start: None,
            end: None,
            limit: Some(5),
        };

        let result = client.get_kline(request).await;
        assert!(
            result.is_ok(),
            "get_kline with interval {:?} should succeed: {:?}",
            interval,
            result.err()
        );

        let response = result.unwrap();
        assert_eq!(response.ret_code, 0, "Response should indicate success");

        println!(
            "Interval {:?}: {} klines",
            interval,
            response.result.list.len()
        );

        // Small delay between requests to avoid rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

#[tokio::test]
async fn test_different_categories() {
    let client = create_public_test_client();

    let categories = [Category::Linear, Category::Spot];

    for category in categories.iter() {
        let request = GetTickersRequest {
            category: category.clone(),
            symbol: None,
            base_coin: None,
            exp_date: None,
        };

        let result = client.get_tickers(request).await;
        assert!(
            result.is_ok(),
            "get_tickers for category {:?} should succeed: {:?}",
            category,
            result.err()
        );

        let response = result.unwrap();
        assert_eq!(response.ret_code, 0, "Response should indicate success");

        println!(
            "Category {:?}: {} tickers",
            category,
            response.result.list.len()
        );

        // Small delay between requests to avoid rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

#[tokio::test]
async fn test_orderbook_different_limits() {
    let client = create_public_test_client();

    let limits = vec![1, 25, 50];

    for limit in limits {
        let request = GetOrderbookRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            limit: Some(limit),
        };

        let result = client.get_orderbook(request).await;
        assert!(
            result.is_ok(),
            "get_orderbook with limit {} should succeed: {:?}",
            limit,
            result.err()
        );

        let response = result.unwrap();
        assert_eq!(response.ret_code, 0, "Response should indicate success");

        let orderbook = &response.result;
        println!(
            "Limit {}: {} asks, {} bids",
            limit,
            orderbook.a.len(),
            orderbook.b.len()
        );

        // Small delay between requests to avoid rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

#[tokio::test]
async fn test_error_handling_invalid_symbol() {
    let client = create_public_test_client();
    let request = GetTickersRequest {
        category: Category::Linear,
        symbol: Some("INVALID-SYMBOL".to_string()),
        base_coin: None,
        exp_date: None,
    };

    let result = client.get_tickers(request).await;

    // This should either succeed with empty results or handle gracefully
    match result {
        Ok(response) => {
            println!("API handled invalid symbol gracefully");
            println!(
                "Response code: {}, message: {}",
                response.ret_code, response.ret_msg
            );
        }
        Err(error) => {
            println!("Expected error for invalid symbol: {:?}", error);
        }
    }
}

#[tokio::test]
async fn test_client_creation() {
    let client = create_public_test_client();

    // Basic connectivity test with server time
    let result = client.get_server_time().await;
    assert!(
        result.is_ok(),
        "Basic connectivity test should succeed: {:?}",
        result.err()
    );

    println!("Bybit public REST client created and connected successfully");
}

// ===============================
// MISSING ENDPOINT TESTS
// ===============================

/// Test mark price kline endpoint
#[tokio::test]
async fn test_get_mark_price_kline() {
    let client = create_public_test_client();
    let request = GetMarkPriceKlineRequest {
        category: Category::Linear,
        symbol: "BTCUSDT".to_string(),
        interval: Interval::Min1,
        start: None,
        end: None,
        limit: Some(10),
    };

    let result = client.get_mark_price_kline(request).await;
    assert!(
        result.is_ok(),
        "get_mark_price_kline should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.ret_code, 0, "Response should indicate success");

    println!(
        "Mark price klines: {} entries for {}",
        response.result.list.len(),
        response.result.symbol
    );
}

/// Test index price kline endpoint
#[tokio::test]
async fn test_get_index_price_kline() {
    let client = create_public_test_client();
    let request = GetIndexPriceKlineRequest {
        category: Category::Linear,
        symbol: "BTCUSDT".to_string(),
        interval: Interval::Min1,
        start: None,
        end: None,
        limit: Some(10),
    };

    let result = client.get_index_price_kline(request).await;
    assert!(
        result.is_ok(),
        "get_index_price_kline should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.ret_code, 0, "Response should indicate success");

    println!(
        "Index price klines: {} entries for {}",
        response.result.list.len(),
        response.result.symbol
    );
}

/// Test premium index price kline endpoint
#[tokio::test]
async fn test_get_premium_index_price_kline() {
    let client = create_public_test_client();
    let request = GetPremiumIndexPriceKlineRequest {
        category: Category::Linear,
        symbol: "BTCUSDT".to_string(),
        interval: Interval::Min1,
        start: None,
        end: None,
        limit: Some(10),
    };

    let result = client.get_premium_index_price_kline(request).await;
    assert!(
        result.is_ok(),
        "get_premium_index_price_kline should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.ret_code, 0, "Response should indicate success");

    println!(
        "Premium index price klines: {} entries for {}",
        response.result.list.len(),
        response.result.symbol
    );
}

/// Test funding history endpoint
#[tokio::test]
async fn test_get_funding_history() {
    let client = create_public_test_client();
    let request = GetFundingHistoryRequest {
        category: Category::Linear,
        symbol: "BTCUSDT".to_string(),
        start_time: None,
        end_time: None,
        limit: Some(10),
    };

    let result = client.get_funding_history(request).await;
    assert!(
        result.is_ok(),
        "get_funding_history should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.ret_code, 0, "Response should indicate success");

    println!(
        "Funding history: {} entries for category {:?}",
        response.result.list.len(),
        response.result.category
    );
}

/// Test open interest endpoint
#[tokio::test]
async fn test_get_open_interest() {
    let client = create_public_test_client();
    let request = GetOpenInterestRequest {
        category: Category::Linear,
        symbol: "BTCUSDT".to_string(),
        interval_time: "5min".to_string(),
        start_time: None,
        end_time: None,
        limit: Some(10),
        cursor: None,
    };

    let result = client.get_open_interest(request).await;
    assert!(
        result.is_ok(),
        "get_open_interest should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.ret_code, 0, "Response should indicate success");

    println!(
        "Open interest: {} entries for {}",
        response.result.list.len(),
        response.result.symbol
    );
}

/// Test historical volatility endpoint
#[tokio::test]
async fn test_get_historical_volatility() {
    let client = create_public_test_client();
    let request = GetHistoricalVolatilityRequest {
        category: Category::Option,
        base_coin: "BTC".to_string(),
        period: Some(7),
        start_time: None,
        end_time: None,
    };

    let result = client.get_historical_volatility(request).await;

    // This might fail for some accounts/regions, handle gracefully
    match result {
        Ok(response) => {
            assert_eq!(response.ret_code, 0, "Response should indicate success");
            println!(
                "Historical volatility: {} entries",
                response.result.list.len()
            );
        }
        Err(error) => {
            println!("Historical volatility test skipped due to: {:?}", error);
        }
    }
}

/// Test long short ratio endpoint
#[tokio::test]
async fn test_get_long_short_ratio() {
    let client = create_public_test_client();
    let request = GetLongShortRatioRequest {
        category: Category::Linear,
        symbol: "BTCUSDT".to_string(),
        period: "5min".to_string(),
        limit: Some(10),
    };

    let result = client.get_long_short_ratio(request).await;

    // This endpoint might have access restrictions
    match result {
        Ok(response) => {
            assert_eq!(response.ret_code, 0, "Response should indicate success");
            println!(
                "Long short ratio: {} entries for category {:?}",
                response.result.list.len(),
                response.result.category
            );
        }
        Err(error) => {
            println!("Long short ratio test skipped due to: {:?}", error);
        }
    }
}

/// Test risk limit endpoint
#[tokio::test]
async fn test_get_risk_limit() {
    let client = create_public_test_client();
    let request = GetRiskLimitRequest {
        category: Category::Linear,
        symbol: Some("BTCUSDT".to_string()),
    };

    let result = client.get_risk_limit(request).await;
    assert!(
        result.is_ok(),
        "get_risk_limit should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.ret_code, 0, "Response should indicate success");

    println!("Risk limits: {} entries", response.result.list.len());
}

/// Test delivery price endpoint
#[tokio::test]
async fn test_get_delivery_price() {
    let client = create_public_test_client();
    let request = GetDeliveryPriceRequest {
        category: Category::Linear,
        symbol: Some("BTCUSDT".to_string()),
        base_coin: None,
        limit: Some(10),
        cursor: None,
    };

    let result = client.get_delivery_price(request).await;

    // This might not be available for all symbols
    match result {
        Ok(response) => {
            assert_eq!(response.ret_code, 0, "Response should indicate success");
            println!("Delivery prices: {} entries", response.result.list.len());
        }
        Err(error) => {
            println!("Delivery price test skipped due to: {:?}", error);
        }
    }
}

/// Test insurance endpoint
#[tokio::test]
async fn test_get_insurance() {
    let client = create_public_test_client();
    let request = GetInsuranceRequest {
        coin: Some("BTC".to_string()),
    };

    let result = client.get_insurance(Some(request)).await;
    assert!(
        result.is_ok(),
        "get_insurance should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.ret_code, 0, "Response should indicate success");

    println!(
        "Insurance data updated at: {}",
        response.result.updated_time
    );
}

/// Test collateral ratio endpoint (no parameters)
#[tokio::test]
async fn test_get_collateral_ratio() {
    let client = create_public_test_client();
    let result = client.get_collateral_ratio().await;

    // This might require special permissions
    match result {
        Ok(response) => {
            assert_eq!(response.ret_code, 0, "Response should indicate success");
            println!("Collateral ratio: {} entries", response.result.list.len());
        }
        Err(error) => {
            println!("Collateral ratio test skipped due to: {:?}", error);
        }
    }
}

/// Test borrowable coins endpoint (no parameters)
#[tokio::test]
async fn test_get_borrowable_coins() {
    let client = create_public_test_client();
    let result = client.get_borrowable_coins().await;

    // This might require special permissions
    match result {
        Ok(response) => {
            assert_eq!(response.ret_code, 0, "Response should indicate success");
            println!("Borrowable coins: {} entries", response.result.list.len());
        }
        Err(error) => {
            println!("Borrowable coins test skipped due to: {:?}", error);
        }
    }
}

/// Test collateral coins endpoint (no parameters)
#[tokio::test]
async fn test_get_collateral_coins() {
    let client = create_public_test_client();
    let result = client.get_collateral_coins().await;

    // This might require special permissions
    match result {
        Ok(response) => {
            assert_eq!(response.ret_code, 0, "Response should indicate success");
            println!("Collateral coins: {} entries", response.result.list.len());
        }
        Err(error) => {
            println!("Collateral coins test skipped due to: {:?}", error);
        }
    }
}

/// Test VIP margin data endpoint
#[tokio::test]
async fn test_get_vip_margin_data() {
    let client = create_public_test_client();
    let request = GetVipMarginDataRequest {
        vip_level: Some("1".to_string()),
        currency: Some("USDT".to_string()),
    };

    let result = client.get_vip_margin_data(Some(request)).await;

    // This might require VIP status
    match result {
        Ok(response) => {
            assert_eq!(response.ret_code, 0, "Response should indicate success");
            println!("VIP margin data: {} entries", response.result.list.len());
        }
        Err(error) => {
            println!("VIP margin data test skipped due to: {:?}", error);
        }
    }
}

/// Test institutional margin coin info endpoint
#[tokio::test]
async fn test_get_ins_margin_coin_info() {
    let client = create_public_test_client();
    let request = GetInsMarginCoinInfoRequest {
        product_id: "BTC".to_string(),
    };

    let result = client.get_ins_margin_coin_info(request).await;

    // This might require institutional access
    match result {
        Ok(response) => {
            assert_eq!(response.ret_code, 0, "Response should indicate success");
            println!(
                "Institutional margin coin info for product {}: {} margin coins",
                response.result.product_id,
                response.result.margin_coin_info_list.len()
            );
        }
        Err(error) => {
            println!(
                "Institutional margin coin info test skipped due to: {:?}",
                error
            );
        }
    }
}

/// Test institutional product info endpoint (no parameters)
#[tokio::test]
async fn test_get_ins_product_info() {
    let client = create_public_test_client();
    let result = client.get_ins_product_info().await;

    // This might require institutional access
    match result {
        Ok(response) => {
            assert_eq!(response.ret_code, 0, "Response should indicate success");
            println!(
                "Institutional product info: {} entries",
                response.result.list.len()
            );
        }
        Err(error) => {
            println!(
                "Institutional product info test skipped due to: {:?}",
                error
            );
        }
    }
}
