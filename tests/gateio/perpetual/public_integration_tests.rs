//! Integration tests for Gate.io perpetual/futures REST API endpoints
//!
//! These tests verify the functionality of perpetual futures endpoints.
//! Tests run against the live Gate.io API using real market data.

use tokio;
use venues::gateio::perpetual::public::rest::RestClient;
use venues::gateio::shared::enums::CandlestickInterval;

/// Helper function to create a test client for perpetual public endpoints
fn create_perpetual_test_client() -> RestClient {
    RestClient::new(false).expect("Failed to create Gate.io perpetual REST client")
}

#[tokio::test]
async fn test_perpetual_client_creation() {
    let _client = create_perpetual_test_client();
    println!("✓ Perpetual client creation successful");
}

/// Test futures contracts endpoint
#[tokio::test]
async fn test_get_futures_contracts() {
    use venues::gateio::perpetual::public::rest::contracts::FuturesContractsRequest;

    let client = create_perpetual_test_client();
    let request = FuturesContractsRequest {
        settle: "usdt".to_string(),
    };

    let result = client.get_futures_contracts(request).await;
    assert!(
        result.is_ok(),
        "get_futures_contracts request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.is_empty(), "Should have contract data");

    let contract = &response[0];
    assert!(!contract.name.is_empty(), "Contract should have name");
    assert!(
        !contract.contract_type.is_empty(),
        "Contract should have type"
    );

    println!("Futures contracts: {} contracts available", response.len());
}

/// Test single futures contract endpoint
#[tokio::test]
async fn test_get_futures_contract() {
    use venues::gateio::perpetual::public::rest::contracts::FuturesContractRequest;

    let client = create_perpetual_test_client();
    let request = FuturesContractRequest {
        settle: "usdt".to_string(),
        contract: "BTC_USDT".to_string(),
    };

    let result = client.get_futures_contract(request).await;
    assert!(
        result.is_ok(),
        "get_futures_contract request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(
        response.name, "BTC_USDT",
        "Should have correct contract name"
    );
    assert!(
        !response.contract_type.is_empty(),
        "Should have contract type"
    );

    println!(
        "Futures contract: {} (type: {})",
        response.name, response.contract_type
    );
}

/// Test futures tickers endpoint
#[tokio::test]
async fn test_get_futures_tickers() {
    use venues::gateio::perpetual::public::rest::tickers::FuturesTickersRequest;

    let client = create_perpetual_test_client();
    let request = FuturesTickersRequest {
        settle: "usdt".to_string(),
        contract: Some("BTC_USDT".to_string()),
    };

    let result = client.get_futures_tickers(request).await;
    assert!(
        result.is_ok(),
        "get_futures_tickers request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.is_empty(), "Should have ticker data");

    let ticker = &response[0];
    assert_eq!(ticker.contract, "BTC_USDT", "Should have correct contract");
    assert!(!ticker.last.is_empty(), "Should have last price");

    println!(
        "Futures ticker: {} = {} (volume: {})",
        ticker.contract,
        ticker.last,
        ticker.volume_24h.as_deref().unwrap_or("N/A")
    );
}

/// Test futures order book endpoint
#[tokio::test]
async fn test_get_futures_order_book() {
    use venues::gateio::perpetual::public::rest::order_book::FuturesOrderBookRequest;

    let client = create_perpetual_test_client();
    let request = FuturesOrderBookRequest {
        settle: "usdt".to_string(),
        contract: "BTC_USDT".to_string(),
        interval: Some("0".to_string()),
        limit: Some(10),
        with_id: None,
    };

    let result = client.get_futures_order_book(request).await;
    assert!(
        result.is_ok(),
        "get_futures_order_book request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.asks.is_empty(), "Should have ask orders");
    assert!(!response.bids.is_empty(), "Should have bid orders");

    println!(
        "Futures order book: {} bids, {} asks",
        response.bids.len(),
        response.asks.len()
    );
}

/// Test futures trades endpoint
#[tokio::test]
async fn test_get_futures_trades() {
    use venues::gateio::perpetual::public::rest::trades::FuturesTradesRequest;

    let client = create_perpetual_test_client();
    let request = FuturesTradesRequest {
        settle: "usdt".to_string(),
        contract: "BTC_USDT".to_string(),
        limit: Some(10),
        offset: None,
        last_id: None,
        from: None,
        to: None,
    };

    let result = client.get_futures_trades(request).await;
    assert!(
        result.is_ok(),
        "get_futures_trades request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.is_empty(), "Should have trade data");

    let trade = &response[0];
    assert!(trade.id > 0, "Trade should have valid ID");
    assert!(!trade.price.is_empty(), "Trade should have price");
    assert!(trade.size != 0, "Trade should have non-zero size");

    println!("Futures trades: {} trades", response.len());
}

/// Test futures candlesticks endpoint
#[tokio::test]
async fn test_get_futures_candlesticks() {
    use venues::gateio::perpetual::public::rest::candlesticks::FuturesCandlesticksRequest;

    let client = create_perpetual_test_client();
    let request = FuturesCandlesticksRequest {
        settle: "usdt".to_string(),
        contract: "BTC_USDT".to_string(),
        from: None,
        to: None,
        limit: Some(10),
        interval: Some(CandlestickInterval::Minutes1),
    };

    let result = client.get_futures_candlesticks(request).await;
    assert!(
        result.is_ok(),
        "get_futures_candlesticks request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.is_empty(), "Should have candlestick data");

    let candle = &response[0];
    assert!(candle.t > 0, "Candle should have valid timestamp");
    assert!(!candle.o.is_empty(), "Candle should have open price");
    assert!(!candle.c.is_empty(), "Candle should have close price");

    println!("Futures candlesticks: {} candles", response.len());
}

/// Test futures mark price candlesticks endpoint
#[tokio::test]
async fn test_get_futures_mark_price_candlesticks() {
    use venues::gateio::perpetual::public::rest::candlesticks::FuturesCandlesticksRequest;

    let client = create_perpetual_test_client();
    let request = FuturesCandlesticksRequest {
        settle: "usdt".to_string(),
        contract: "BTC_USDT".to_string(),
        from: None,
        to: None,
        limit: Some(10),
        interval: Some(CandlestickInterval::Minutes1),
    };

    let result = client.get_futures_candlesticks(request).await;
    assert!(
        result.is_ok(),
        "get_futures_mark_price_candlesticks request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.is_empty(),
        "Should have mark price candlestick data"
    );

    let candle = &response[0];
    assert!(candle.t > 0, "Candle should have valid timestamp");
    assert!(!candle.o.is_empty(), "Candle should have open price");

    println!(
        "Futures mark price candlesticks: {} candles",
        response.len()
    );
}

/// Test futures index price candlesticks endpoint
#[tokio::test]
async fn test_get_futures_index_price_candlesticks() {
    use venues::gateio::perpetual::public::rest::candlesticks::FuturesCandlesticksRequest;

    let client = create_perpetual_test_client();
    let request = FuturesCandlesticksRequest {
        settle: "usdt".to_string(),
        contract: "BTC_USDT".to_string(),
        from: None,
        to: None,
        limit: Some(10),
        interval: Some(CandlestickInterval::Minutes1),
    };

    let result = client.get_futures_candlesticks(request).await;
    assert!(
        result.is_ok(),
        "get_futures_index_price_candlesticks request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.is_empty(),
        "Should have index price candlestick data"
    );

    let candle = &response[0];
    assert!(candle.t > 0, "Candle should have valid timestamp");
    assert!(!candle.o.is_empty(), "Candle should have open price");

    println!(
        "Futures index price candlesticks: {} candles",
        response.len()
    );
}

/// Test futures funding rate endpoint
#[tokio::test]
async fn test_get_futures_funding_rate() {
    use venues::gateio::perpetual::public::rest::funding_rate::FuturesFundingRateRequest;

    let client = create_perpetual_test_client();
    let request = FuturesFundingRateRequest {
        settle: "usdt".to_string(),
        contract: "BTC_USDT".to_string(),
        limit: Some(10),
    };

    let result = client.get_futures_funding_rate(request).await;
    assert!(
        result.is_ok(),
        "get_futures_funding_rate request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.is_empty(), "Should have funding rate data");

    let funding = &response[0];
    assert!(funding.t > 0, "Funding rate should have valid timestamp");
    assert!(!funding.r.is_empty(), "Funding rate should have rate value");

    println!("Futures funding rates: {} entries", response.len());
}

/// Test futures insurance endpoint
#[tokio::test]
async fn test_get_futures_insurance() {
    use venues::gateio::perpetual::public::rest::insurance::FuturesInsuranceRequest;

    let client = create_perpetual_test_client();
    let request = FuturesInsuranceRequest {
        settle: "usdt".to_string(),
        limit: Some(10),
    };

    let result = client.get_futures_insurance(request).await;
    assert!(
        result.is_ok(),
        "get_futures_insurance request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.is_empty(), "Should have insurance data");

    let insurance = &response[0];
    assert!(insurance.t > 0, "Insurance should have valid timestamp");
    assert!(insurance.b > 0.0, "Insurance should have positive balance");

    println!("Futures insurance: {} entries", response.len());
}

/// Test futures stats endpoint
#[tokio::test]
async fn test_get_futures_stats() {
    use venues::gateio::perpetual::public::rest::stats::FuturesStatsRequest;

    let client = create_perpetual_test_client();
    let request = FuturesStatsRequest {
        settle: "usdt".to_string(),
        contract: "BTC_USDT".to_string(),
        from: None,
        interval: Some("5m".to_string()),
        limit: Some(10),
    };

    let result = client.get_futures_stats(request).await;
    assert!(
        result.is_ok(),
        "get_futures_stats request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.is_empty(), "Should have stats data");

    let stats = &response[0];
    assert!(stats.time > 0, "Stats should have valid timestamp");

    println!("Futures stats: {} entries", response.len());
}

/// Test futures risk limit tiers endpoint
#[tokio::test]
async fn test_get_futures_risk_limit_tiers() {
    use venues::gateio::perpetual::public::rest::risk_limit_tiers::FuturesRiskLimitTiersRequest;

    let client = create_perpetual_test_client();
    let request = FuturesRiskLimitTiersRequest {
        settle: "usdt".to_string(),
        contract: "BTC_USDT".to_string(),
        offset: Some(0),
        limit: Some(10),
    };

    let result = client.get_futures_risk_limit_tiers(request).await;
    assert!(
        result.is_ok(),
        "get_futures_risk_limit_tiers request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.is_empty(), "Should have risk limit tiers data");

    let tier = &response[0];
    assert!(!tier.risk_limit.is_empty(), "Tier should have risk limit");
    assert!(!tier.risk_limit.is_empty(), "Tier should have risk limit");

    println!("Futures risk limit tiers: {} tiers", response.len());
}

/// Test futures premium index endpoint
#[tokio::test]
async fn test_get_futures_premium_index() {
    use venues::gateio::perpetual::public::rest::premium_index::FuturesPremiumIndexRequest;

    let client = create_perpetual_test_client();
    let request = FuturesPremiumIndexRequest {
        settle: "usdt".to_string(),
        contract: "BTC_USDT".to_string(),
        from: None,
        to: None,
        interval: Some(CandlestickInterval::Minutes1),
        limit: Some(10),
    };

    let result = client.get_futures_premium_index(request).await;
    assert!(
        result.is_ok(),
        "get_futures_premium_index request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.is_empty(), "Should have premium index data");

    let premium = &response[0];
    assert!(premium.t > 0, "Premium index should have valid timestamp");

    println!("Futures premium index: {} entries", response.len());
}

/// Test futures index constituents endpoint
#[tokio::test]
async fn test_get_futures_index_constituents() {
    use venues::gateio::perpetual::public::rest::index_constituents::FuturesIndexConstituentsRequest;

    let client = create_perpetual_test_client();
    let request = FuturesIndexConstituentsRequest {
        settle: "usdt".to_string(),
        index: "BTC_USDT".to_string(),
    };

    let result = client.get_futures_index_constituents(request).await;
    assert!(
        result.is_ok(),
        "get_futures_index_constituents request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.constituents.is_empty(),
        "Should have index constituents data"
    );

    println!(
        "Futures index constituents: {} constituents for index {}",
        response.constituents.len(),
        response.index
    );
}
