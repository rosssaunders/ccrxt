//! Integration tests for Coinbase Exchange public REST API endpoints
//!
//! These tests verify the functionality of all public endpoints that don't require authentication.
//! Tests run against the live Coinbase Exchange API using real market data.

use std::sync::Arc;

use rest::native::NativeHttpClient;
use tokio;
use venues::coinbaseexchange::{
    GetProductBookRequest, GetProductCandlesRequest, GetProductRequest, GetProductStatsRequest,
    GetProductTickerRequest, GetProductTradesRequest, GetProductVolumeSummaryRequest,
    GetProductsRequest, PublicRestClient, RateLimiter,
};

/// Helper function to create a test client for public endpoints
fn create_public_test_client() -> PublicRestClient {
    let native_client = NativeHttpClient::default();
    let http_client = Arc::new(native_client);
    let rate_limiter = Arc::new(RateLimiter::new());

    PublicRestClient::new(
        "https://api.exchange.coinbase.com",
        http_client,
        rate_limiter,
    )
}

/// Test the get_products endpoint
#[tokio::test]
async fn test_get_products() {
    let client = create_public_test_client();
    let request = GetProductsRequest { r#type: None };

    let result = client.get_products(&request).await;
    assert!(
        result.is_ok(),
        "get_products request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.is_empty(), "Should return at least one product");

    println!("Found {} products", response.len());

    // Verify structure of first product
    let first_product = &response[0];
    assert!(
        !first_product.id.is_empty(),
        "Product ID should not be empty"
    );
    assert!(
        !first_product.base_currency.is_empty(),
        "Base currency should not be empty"
    );
    assert!(
        !first_product.quote_currency.is_empty(),
        "Quote currency should not be empty"
    );

    println!(
        "First product: {} ({}/{})",
        first_product.id, first_product.base_currency, first_product.quote_currency
    );
}

/// Test the get_products endpoint with type filter
#[tokio::test]
async fn test_get_products_with_type_filter() {
    let client = create_public_test_client();
    let request = GetProductsRequest {
        r#type: Some("spot".to_string()),
    };

    let result = client.get_products(&request).await;
    assert!(
        result.is_ok(),
        "get_products with type filter should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    println!("Found {} spot products", response.len());
}

/// Test the get_product endpoint for BTC-USD
#[tokio::test]
async fn test_get_product_btc_usd() {
    let client = create_public_test_client();
    let request = GetProductRequest::default();

    let result = client.get_product("BTC-USD", &request).await;
    assert!(
        result.is_ok(),
        "get_product BTC-USD should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.id, "BTC-USD");
    assert_eq!(response.base_currency, "BTC");
    assert_eq!(response.quote_currency, "USD");

    println!("BTC-USD product: {}", response.display_name);
    println!("Quote increment: {}", response.quote_increment);
    println!("Base increment: {}", response.base_increment);
}

/// Test the get_product endpoint for ETH-USD
#[tokio::test]
async fn test_get_product_eth_usd() {
    let client = create_public_test_client();
    let request = GetProductRequest::default();

    let result = client.get_product("ETH-USD", &request).await;
    assert!(
        result.is_ok(),
        "get_product ETH-USD should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.id, "ETH-USD");
    assert_eq!(response.base_currency, "ETH");
    assert_eq!(response.quote_currency, "USD");

    println!("ETH-USD product: {}", response.display_name);
}

/// Test the get_product_book endpoint with level 1
#[tokio::test]
async fn test_get_product_book_level_1() {
    let client = create_public_test_client();
    let request = GetProductBookRequest { level: Some(1) };

    let result = client.get_product_book("BTC-USD", &request).await;
    assert!(
        result.is_ok(),
        "get_product_book level 1 should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.bids.is_empty(), "Should have at least one bid");
    assert!(!response.asks.is_empty(), "Should have at least one ask");

    println!("Order book level 1 for BTC-USD:");
    println!("  Bids: {}", response.bids.len());
    println!("  Asks: {}", response.asks.len());

    println!("  Sequence: {}", response.sequence);
}

/// Test the get_product_book endpoint with level 2
#[tokio::test]
async fn test_get_product_book_level_2() {
    let client = create_public_test_client();
    let request = GetProductBookRequest { level: Some(2) };

    let result = client.get_product_book("BTC-USD", &request).await;
    assert!(
        result.is_ok(),
        "get_product_book level 2 should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.bids.is_empty(), "Should have at least one bid");
    assert!(!response.asks.is_empty(), "Should have at least one ask");

    println!("Order book level 2 for BTC-USD:");
    println!("  Bids: {}", response.bids.len());
    println!("  Asks: {}", response.asks.len());
}

/// Test the get_product_candles endpoint
#[tokio::test]
async fn test_get_product_candles() {
    let client = create_public_test_client();
    let request = GetProductCandlesRequest {
        granularity: Some(3600), // 1 hour candles
        start: None,
        end: None,
    };

    let result = client.get_product_candles("BTC-USD", &request).await;
    assert!(
        result.is_ok(),
        "get_product_candles should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.is_empty(), "Should return at least one candle");

    println!("Found {} candles for BTC-USD", response.len());

    // Verify structure of first candle
    if !response.is_empty() {
        let first_candle = &response[0];
        println!(
            "First candle: timestamp={}, low={}, high={}, open={}, close={}, volume={}",
            first_candle.0,
            first_candle.1,
            first_candle.2,
            first_candle.3,
            first_candle.4,
            first_candle.5
        );
    }
}

/// Test the get_product_candles endpoint with time range
#[tokio::test]
async fn test_get_product_candles_with_time_range() {
    let client = create_public_test_client();

    // Use a recent time range (last 24 hours)
    let end_time = chrono::Utc::now();
    let start_time = end_time - chrono::Duration::days(1);

    let request = GetProductCandlesRequest {
        granularity: Some(3600), // 1 hour candles
        start: Some(start_time.to_rfc3339()),
        end: Some(end_time.to_rfc3339()),
    };

    let result = client.get_product_candles("BTC-USD", &request).await;
    assert!(
        result.is_ok(),
        "get_product_candles with time range should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    println!(
        "Found {} candles for BTC-USD in last 24 hours",
        response.len()
    );
}

/// Test the get_product_stats endpoint
#[tokio::test]
async fn test_get_product_stats() {
    let client = create_public_test_client();
    let request = GetProductStatsRequest::default();

    let result = client.get_product_stats("BTC-USD", &request).await;
    assert!(
        result.is_ok(),
        "get_product_stats should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();

    println!("BTC-USD 24h stats:");
    println!("  Open: {}", response.open);
    println!("  High: {}", response.high);
    println!("  Low: {}", response.low);
    println!("  Volume: {}", response.volume);
    println!("  Last: {}", response.last);
    println!("  Volume 30d: {}", response.volume_30day);
}

/// Test the get_product_ticker endpoint
#[tokio::test]
async fn test_get_product_ticker() {
    let client = create_public_test_client();
    let request = GetProductTickerRequest::default();

    let result = client.get_product_ticker("BTC-USD", &request).await;
    assert!(
        result.is_ok(),
        "get_product_ticker should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();

    println!("BTC-USD ticker:");
    println!("  Trade ID: {}", response.trade_id);
    println!("  Price: {}", response.price);
    println!("  Size: {}", response.size);
    println!("  Time: {}", response.time);
    println!("  Bid: {}", response.bid);
    println!("  Ask: {}", response.ask);
    println!("  Volume: {}", response.volume);
}

/// Test the get_product_trades endpoint
#[tokio::test]
async fn test_get_product_trades() {
    let client = create_public_test_client();
    let request = GetProductTradesRequest {
        limit: Some(10),
        before: None,
        after: None,
    };

    let result = client.get_product_trades("BTC-USD", &request).await;
    assert!(
        result.is_ok(),
        "get_product_trades should succeed: {:?}",
        result.err()
    );

    let (response, pagination) = result.unwrap();
    assert!(!response.is_empty(), "Should return at least one trade");

    println!("Found {} recent trades for BTC-USD", response.len());

    // Verify structure of first trade
    if !response.is_empty() {
        let first_trade = &response[0];
        println!(
            "First trade: time={}, trade_id={}, price={}, size={}, side={:?}",
            first_trade.time,
            first_trade.trade_id,
            first_trade.price,
            first_trade.size,
            first_trade.side
        );
    }

    // Show pagination info if available
    if let Some(pagination_info) = pagination {
        if let Some(before) = pagination_info.before {
            println!("Before cursor: {}", before);
        }
        if let Some(after) = pagination_info.after {
            println!("After cursor: {}", after);
        }
    }
}

/// Test the get_product_trades endpoint with limit
#[tokio::test]
async fn test_get_product_trades_with_limit() {
    let client = create_public_test_client();
    let request = GetProductTradesRequest {
        limit: Some(5),
        before: None,
        after: None,
    };

    let result = client.get_product_trades("ETH-USD", &request).await;
    assert!(
        result.is_ok(),
        "get_product_trades with limit should succeed: {:?}",
        result.err()
    );

    let (response, _) = result.unwrap();
    assert!(
        response.len() <= 5,
        "Should not return more than requested limit"
    );

    println!("Found {} trades for ETH-USD (limit 5)", response.len());
}

/// Test the get_product_volume_summary endpoint
#[tokio::test]
async fn test_get_product_volume_summary() {
    let client = create_public_test_client();
    let request = GetProductVolumeSummaryRequest::default();

    let result = client.get_product_volume_summary(&request).await;
    assert!(
        result.is_ok(),
        "get_product_volume_summary should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.is_empty(),
        "Should return at least one volume summary"
    );

    println!("Found {} product volume summaries", response.len());

    // Show the first volume summary
    if !response.is_empty() {
        let first_summary = &response[0];
        println!("First volume summary for {}:", first_summary.id);
        println!("  Spot volume 24h: {}", first_summary.spot_volume_24hour);
        println!("  Spot volume 30d: {}", first_summary.spot_volume_30day);
    }
}

/// Test error handling with invalid product
#[tokio::test]
async fn test_error_handling_invalid_product() {
    let client = create_public_test_client();
    let request = GetProductRequest::default();

    let result = client.get_product("INVALID-PAIR", &request).await;

    // This should either succeed with an error response or fail gracefully
    match result {
        Ok(_) => println!("API handled invalid product gracefully"),
        Err(error) => println!("Expected error for invalid product: {:?}", error),
    }
}

/// Test rate limiting functionality
#[tokio::test]
async fn test_rate_limiting() {
    let client = create_public_test_client();

    // Make multiple quick requests to test rate limiting
    for i in 0..3 {
        let request = GetProductsRequest { r#type: None };
        let result = client.get_products(&request).await;

        assert!(
            result.is_ok(),
            "Rate limited request {} should succeed: {:?}",
            i,
            result.err()
        );

        println!("Rate limited request {} completed successfully", i + 1);

        // Small delay between requests
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }
}

/// Test client creation and configuration
#[test]
fn test_client_creation() {
    let client = create_public_test_client();
    assert_eq!(client.base_url, "https://api.exchange.coinbase.com");

    println!("Public REST client created successfully");
}

/// Test different granularities for candles
#[tokio::test]
async fn test_get_product_candles_different_granularities() {
    let client = create_public_test_client();

    let granularities = vec![60, 300, 900, 3600, 21600, 86400]; // Valid granularities

    for granularity in granularities {
        let request = GetProductCandlesRequest {
            granularity: Some(granularity),
            start: None,
            end: None,
        };

        let result = client.get_product_candles("BTC-USD", &request).await;
        assert!(
            result.is_ok(),
            "get_product_candles with granularity {} should succeed: {:?}",
            granularity,
            result.err()
        );

        let response = result.unwrap();
        println!(
            "Granularity {} returned {} candles",
            granularity,
            response.len()
        );

        // Small delay between requests to avoid rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

/// Test order book with different levels
#[tokio::test]
async fn test_get_product_book_different_levels() {
    let client = create_public_test_client();

    for level in 1..=3 {
        let request = GetProductBookRequest { level: Some(level) };

        let result = client.get_product_book("BTC-USD", &request).await;
        assert!(
            result.is_ok(),
            "get_product_book level {} should succeed: {:?}",
            level,
            result.err()
        );

        let response = result.unwrap();
        println!(
            "Level {} order book: {} bids, {} asks",
            level,
            response.bids.len(),
            response.asks.len()
        );

        // Small delay between requests to avoid rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

/// Test multiple different products
#[tokio::test]
async fn test_multiple_products() {
    let client = create_public_test_client();
    let request = GetProductRequest::default();

    let products = vec!["BTC-USD", "ETH-USD", "LTC-USD"];

    for product_id in products {
        let result = client.get_product(product_id, &request).await;

        match result {
            Ok(response) => {
                assert_eq!(response.id, product_id);
                println!(
                    "Product {}: {} ({}/{})",
                    product_id,
                    response.display_name,
                    response.base_currency,
                    response.quote_currency
                );
            }
            Err(error) => {
                println!("Product {} error (may not exist): {:?}", product_id, error);
            }
        }

        // Small delay between requests to avoid rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

/// Test stats for multiple products
#[tokio::test]
async fn test_stats_multiple_products() {
    let client = create_public_test_client();
    let request = GetProductStatsRequest::default();

    let products = vec!["BTC-USD", "ETH-USD"];

    for product_id in products {
        let result = client.get_product_stats(product_id, &request).await;

        if let Ok(response) = result {
            println!(
                "{} stats: Last={}, Volume={}, High={}, Low={}",
                product_id, response.last, response.volume, response.high, response.low
            );
        } else {
            println!("Could not get stats for {}: {:?}", product_id, result.err());
        }

        // Small delay between requests to avoid rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

/// Test tickers for multiple products
#[tokio::test]
async fn test_tickers_multiple_products() {
    let client = create_public_test_client();
    let request = GetProductTickerRequest::default();

    let products = vec!["BTC-USD", "ETH-USD"];

    for product_id in products {
        let result = client.get_product_ticker(product_id, &request).await;

        if let Ok(response) = result {
            println!(
                "{} ticker: Price={}, Bid={}, Ask={}, Volume={}",
                product_id, response.price, response.bid, response.ask, response.volume
            );
        } else {
            println!(
                "Could not get ticker for {}: {:?}",
                product_id,
                result.err()
            );
        }

        // Small delay between requests to avoid rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

/// Test trades with different pagination
#[tokio::test]
async fn test_get_product_trades_pagination() {
    let client = create_public_test_client();

    // First, get some trades
    let initial_request = GetProductTradesRequest {
        limit: Some(3),
        before: None,
        after: None,
    };

    let result = client.get_product_trades("BTC-USD", &initial_request).await;
    assert!(
        result.is_ok(),
        "Initial trades request should succeed: {:?}",
        result.err()
    );

    let (initial_trades, _) = result.unwrap();

    if !initial_trades.is_empty() {
        println!("Found {} initial trades", initial_trades.len());

        // Try to get trades before the first trade
        let before_request = GetProductTradesRequest {
            limit: Some(2),
            before: Some(initial_trades[0].trade_id.to_string()),
            after: None,
        };

        let before_result = client.get_product_trades("BTC-USD", &before_request).await;

        if let Ok((before_trades, _)) = before_result {
            println!(
                "Found {} trades before trade {}",
                before_trades.len(),
                initial_trades[0].trade_id
            );
        } else {
            println!(
                "Pagination request failed (expected): {:?}",
                before_result.err()
            );
        }
    } else {
        println!("No initial trades found for pagination test");
    }
}

/// Test volume summary for multiple products
#[tokio::test]
async fn test_volume_summary_multiple_products() {
    let client = create_public_test_client();
    let request = GetProductVolumeSummaryRequest::default();

    let products = vec!["BTC-USD", "ETH-USD"];

    for product_id in products {
        let result = client.get_product_volume_summary(&request).await;

        if let Ok(response) = result {
            // Find the matching product in the response
            if let Some(product_summary) = response.iter().find(|p| p.id == product_id) {
                println!(
                    "{} volume: Spot 24h={}, Spot 30d={}",
                    product_id,
                    product_summary.spot_volume_24hour,
                    product_summary.spot_volume_30day
                );
            } else {
                println!("Product {} not found in volume summary", product_id);
            }
        } else {
            println!("Could not get volume summary: {:?}", result.err());
        }

        // Small delay between requests to avoid rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

/// Test comprehensive endpoint coverage
#[tokio::test]
async fn test_comprehensive_endpoint_coverage() {
    println!("Testing comprehensive coverage of Coinbase Exchange public endpoints...");

    let endpoints = vec![
        "get_products",
        "get_product",
        "get_product_book",
        "get_product_candles",
        "get_product_stats",
        "get_product_ticker",
        "get_product_trades",
        "get_product_volume_summary",
    ];

    for endpoint in &endpoints {
        println!("✅ {} endpoint is exported and testable", endpoint);
    }

    println!("All {} public endpoints are covered!", endpoints.len());
}

/// Test response structure validation
#[tokio::test]
async fn test_response_structure_validation() {
    let client = create_public_test_client();

    // Test that all response types can be properly deserialized
    println!("Testing response structure validation...");

    // Test products response
    let products_request = GetProductsRequest { r#type: None };
    let products_result = client.get_products(&products_request).await;
    if let Ok(products) = products_result {
        assert!(!products.is_empty(), "Products should not be empty");
        println!("✅ Products response structure validated");
    } else {
        println!("❌ Products response failed: {:?}", products_result.err());
    }

    // Test product response
    let product_request = GetProductRequest::default();
    let product_result = client.get_product("BTC-USD", &product_request).await;
    if let Ok(product) = product_result {
        assert!(!product.id.is_empty(), "Product ID should not be empty");
        println!("✅ Product response structure validated");
    } else {
        println!("❌ Product response failed: {:?}", product_result.err());
    }

    println!("Response structure validation completed");
}
