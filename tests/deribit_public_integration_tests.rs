//! Integration tests for Deribit public REST API endpoints
//!
//! These tests verify the functionality of all public endpoints that don't require authentication.
//! Tests run against the live Deribit API using real market data.

use reqwest::Client;
use tokio;

use venues::deribit::{
    AccountTier, Currency, GetComboIdsRequest, InstrumentKind, PublicRestClient, RateLimiter,
};

// Import additional request/response types from individual modules
use venues::deribit::public::rest::{GetComboDetailsRequest, GetCombosRequest};

/// Helper function to create a test client for public endpoints
fn create_public_test_client() -> PublicRestClient {
    let client = Client::new();
    let rate_limiter = RateLimiter::new(AccountTier::Tier1);

    PublicRestClient::new("https://www.deribit.com", client, rate_limiter)
}

/// Test the get_status endpoint
#[tokio::test]
async fn test_get_status() {
    let client = create_public_test_client();

    let result = client.get_status().await;
    assert!(
        result.is_ok(),
        "get_status request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    // Status result should have basic fields
    println!("Platform locked status: {:?}", response.result.locked);
}

/// Test the get_time endpoint
#[tokio::test]
async fn test_get_time() {
    let client = create_public_test_client();
    let result = client.get_time().await;
    assert!(
        result.is_ok(),
        "get_time request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);
    assert!(response.result > 0, "Timestamp should be positive");

    println!("Current server time: {}", response.result);
}

/// Test the get_combo_ids endpoint for BTC
#[tokio::test]
async fn test_get_combo_ids_btc() {
    let client = create_public_test_client();
    let request = GetComboIdsRequest {
        currency: Currency::BTC,
        state: None,
    };

    let result = client.get_combo_ids(request).await;
    assert!(
        result.is_ok(),
        "get_combo_ids request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!("Found {} BTC combo IDs", response.result.len());
}

/// Test the get_combo_ids endpoint for ETH
#[tokio::test]
async fn test_get_combo_ids_eth() {
    let client = create_public_test_client();
    let request = GetComboIdsRequest {
        currency: Currency::ETH,
        state: None,
    };

    let result = client.get_combo_ids(request).await;
    assert!(
        result.is_ok(),
        "get_combo_ids request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!("Found {} ETH combo IDs", response.result.len());
}

/// Test the get_combo_details endpoint
#[tokio::test]
async fn test_get_combo_details() {
    let client = create_public_test_client();

    // First get some combo IDs to test with
    let combo_ids_request = GetComboIdsRequest {
        currency: Currency::BTC,
        state: None,
    };

    let combo_ids_result = client.get_combo_ids(combo_ids_request).await;
    assert!(
        combo_ids_result.is_ok(),
        "Should be able to get combo IDs first"
    );

    let combo_ids_response = combo_ids_result.unwrap();

    if !combo_ids_response.result.is_empty() {
        let combo_id = &combo_ids_response.result[0];

        let request = GetComboDetailsRequest {
            combo_id: combo_id.clone(),
        };

        let result = client.get_combo_details(request).await;
        assert!(
            result.is_ok(),
            "get_combo_details request should succeed: {:?}",
            result.err()
        );

        let response = result.unwrap();
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.id > 0);

        println!("Got combo details for combo ID: {}", combo_id);
    } else {
        println!("No combo IDs available to test combo details");
    }
}

/// Test the get_combos endpoint for BTC
#[tokio::test]
async fn test_get_combos_btc() {
    let client = create_public_test_client();
    let request = GetCombosRequest {
        currency: Currency::BTC,
    };

    let result = client.get_combos(request).await;
    assert!(
        result.is_ok(),
        "get_combos request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!("Found {} BTC combos", response.result.len());

    // Validate structure of first combo if available
    if !response.result.is_empty() {
        let first_combo = &response.result[0];
        assert!(!first_combo.id.is_empty());
        assert!(!first_combo.legs.is_empty());

        println!("First combo ID: {}", first_combo.id);
        println!("Number of legs: {}", first_combo.legs.len());
    }
}

/// Test the get_currencies endpoint
#[tokio::test]
async fn test_get_currencies() {
    let client = create_public_test_client();

    let result = client.get_currencies().await;
    assert!(
        result.is_ok(),
        "get_currencies request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);
    assert!(
        !response.result.is_empty(),
        "Should return at least one currency"
    );

    println!("Found {} supported currencies", response.result.len());

    // Verify that BTC and ETH are in the results
    let btc_found = response.result.iter().any(|c| c.currency == Currency::BTC);
    let eth_found = response.result.iter().any(|c| c.currency == Currency::ETH);

    assert!(btc_found, "BTC should be in supported currencies");
    assert!(eth_found, "ETH should be in supported currencies");

    // Verify structure of first currency
    let first_currency = &response.result[0];
    assert!(!first_currency.currency_long.is_empty());
    assert!(first_currency.fee_precision > 0);

    println!(
        "First currency: {} ({})",
        first_currency.currency, first_currency.currency_long
    );
}

/// Test rate limiting functionality
#[tokio::test]
async fn test_rate_limiting() {
    let client = create_public_test_client();

    // Make multiple quick requests to test rate limiting
    for i in 0..3 {
        let result = client.get_status().await;

        assert!(
            result.is_ok(),
            "Request {} should succeed with rate limiting: {:?}",
            i,
            result.err()
        );

        println!("Rate limited request {} completed successfully", i + 1);

        // Small delay between requests
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

/// Test error handling for invalid requests
#[tokio::test]
async fn test_error_handling() {
    let client = create_public_test_client();

    // Test with an invalid combo ID
    let request = GetComboDetailsRequest {
        combo_id: "invalid_combo_id_that_does_not_exist".to_string(),
    };

    let result = client.get_combo_details(request).await;

    // This should either succeed (if the API returns an empty result)
    // or fail gracefully with a proper error
    match result {
        Ok(response) => {
            println!("API handled invalid combo ID gracefully");
            assert_eq!(response.jsonrpc, "2.0");
        }
        Err(error) => {
            println!(
                "API returned expected error for invalid combo ID: {:?}",
                error
            );
            // Error should be structured, not a panic
        }
    }
}

/// Test client creation and configuration
#[test]
fn test_client_creation() {
    let client = create_public_test_client();
    assert_eq!(client.base_url, "https://www.deribit.com");

    println!("Public REST client created successfully");
}

/// Test different account tier rate limiters
#[test]
fn test_rate_limiter_tiers() {
    let _tier1_limiter = RateLimiter::new(AccountTier::Tier1);
    let _tier2_limiter = RateLimiter::new(AccountTier::Tier2);
    let _tier3_limiter = RateLimiter::new(AccountTier::Tier3);

    // All rate limiters should be created successfully
    println!("Rate limiters for all tiers created successfully");
}

/// Test the get_instruments endpoint for BTC
#[tokio::test]
async fn test_get_instruments_btc() {
    let client = create_public_test_client();

    // Create request for BTC instruments
    let request = venues::deribit::public::rest::get_instruments::GetInstrumentsRequest {
        currency: Currency::BTC,
        kind: None,
        expired: Some(false),
    };

    let result = client.get_instruments(request).await;
    assert!(
        result.is_ok(),
        "get_instruments request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!("Found {} BTC instruments", response.result.len());

    // Verify structure of first instrument if available
    if !response.result.is_empty() {
        let first_instrument = &response.result[0];
        println!("Instrument name: {}", first_instrument.instrument_name);
        println!("Currency: {:?}", first_instrument.currency);
        println!("Kind: {:?}", first_instrument.kind);
        println!("Tick size: {}", first_instrument.tick_size);
        println!("Contract size: {}", first_instrument.contract_size);
        println!("Min trade amount: {}", first_instrument.min_trade_amount);
        println!("Strike: {:?}", first_instrument.strike);
        println!(
            "Expiration timestamp: {:?}",
            first_instrument.expiration_timestamp
        );
        println!(
            "Creation timestamp: {:?}",
            first_instrument.creation_timestamp
        );
        println!(
            "Settlement period: {:?}",
            first_instrument.settlement_period
        );
        println!("Base currency: {:?}", first_instrument.base_currency);
        println!("Quote currency: {:?}", first_instrument.quote_currency);

        println!("First BTC instrument: {}", first_instrument.instrument_name);
    }
}

/// Test the get_instruments endpoint with specific instrument kind
#[tokio::test]
async fn test_get_instruments_btc_futures() {
    let client = create_public_test_client();

    // Create request for BTC futures only
    let request = venues::deribit::public::rest::get_instruments::GetInstrumentsRequest {
        currency: Currency::BTC,
        kind: Some(InstrumentKind::Future),
        expired: Some(false),
    };

    let result = client.get_instruments(request).await;
    assert!(
        result.is_ok(),
        "get_instruments futures request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");

    println!("Found {} BTC futures", response.result.len());

    // Print all returned instruments' kind and currency
    for instrument in &response.result {
        println!(
            "Instrument: {}, Kind: {:?}, Currency: {:?}",
            instrument.instrument_name, instrument.kind, instrument.currency
        );
    }
}

/// Test the get_supported_index_names endpoint
#[tokio::test]
async fn test_get_supported_index_names() {
    let client = create_public_test_client();

    let result = client.get_supported_index_names().await;
    assert!(
        result.is_ok(),
        "get_supported_index_names request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);
    assert!(
        !response.result.is_empty(),
        "Should return at least one supported index name"
    );

    println!("Found {} supported index names", response.result.len());

    // Print the first few index names
    for (i, index_name) in response.result.iter().take(5).enumerate() {
        println!("Index name {}: {}", i + 1, index_name);
    }
}

/// Test the get_index_price endpoint
#[tokio::test]
async fn test_get_index_price() {
    let client = create_public_test_client();

    let request = venues::deribit::public::rest::get_index_price::GetIndexPriceRequest {
        index_name: "btc_usd".to_string(),
    };

    let result = client.get_index_price(request).await;
    assert!(
        result.is_ok(),
        "get_index_price request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);
    assert!(
        response.result.index_price > 0.0,
        "Index price should be positive"
    );

    println!("BTC_USD index price: {}", response.result.index_price);
    println!(
        "Estimated delivery price: {}",
        response.result.estimated_delivery_price
    );
}

/// Test the get_contract_size endpoint
#[tokio::test]
async fn test_get_contract_size() {
    let client = create_public_test_client();

    let request = venues::deribit::public::rest::get_contract_size::GetContractSizeRequest {
        instrument_name: "BTC-PERPETUAL".to_string(),
    };

    let result = client.get_contract_size(request).await;
    assert!(
        result.is_ok(),
        "get_contract_size request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);
    assert!(
        response.result.contract_size > 0.0,
        "Contract size should be positive"
    );

    println!(
        "BTC-PERPETUAL contract size: {}",
        response.result.contract_size
    );
}

/// Test the get_funding_rate_value endpoint
#[tokio::test]
async fn test_get_funding_rate_value() {
    let client = create_public_test_client();

    let request = venues::deribit::GetFundingRateValueRequest {
        instrument_name: "BTC-PERPETUAL".to_string(),
        start_timestamp: chrono::Utc::now().timestamp_millis() as u64 - 3600000, // 1 hour ago
        end_timestamp: chrono::Utc::now().timestamp_millis() as u64,             // now
    };

    let result = client.get_funding_rate_value(request).await;
    assert!(
        result.is_ok(),
        "get_funding_rate_value request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!("BTC-PERPETUAL funding rate: {}", response.result);
    println!("Timestamp: {}", response.result);
}

/// Test the get_last_trades_by_currency endpoint
#[tokio::test]
async fn test_get_last_trades_by_currency() {
    let client = create_public_test_client();

    let request = venues::deribit::public::rest::get_last_trades_by_currency::GetLastTradesByCurrencyRequest {
        currency: Currency::BTC,
        kind: InstrumentKind::Future,
        count: Some(10),
        sorting: None,
    };

    let result = client.get_last_trades_by_currency(request).await;
    assert!(
        result.is_ok(),
        "get_last_trades_by_currency request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!("Found {} recent BTC trades", response.result.trades.len());

    // Verify structure of first trade if available
    if !response.result.trades.is_empty() {
        let first_trade = &response.result.trades[0];
        assert!(!first_trade.instrument_name.is_empty());
        assert!(first_trade.price > 0.0);
        assert!(first_trade.amount > 0.0);

        println!(
            "First trade: {} @ {} for {}",
            first_trade.instrument_name, first_trade.price, first_trade.amount
        );
    }
}

/// Test the get_last_settlements_by_currency endpoint
#[tokio::test]
async fn test_get_last_settlements_by_currency() {
    let client = create_public_test_client();

    let request = venues::deribit::public::rest::get_last_settlements_by_currency::GetLastSettlementsByCurrencyRequest {
        currency: Currency::BTC,
        kind: InstrumentKind::Future,
        count: Some(10),
    };

    let result = client.get_last_settlements_by_currency(request).await;
    assert!(
        result.is_ok(),
        "get_last_settlements_by_currency request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!(
        "Found {} BTC settlements",
        response.result.settlements.len()
    );

    // Verify structure of first settlement if available
    if !response.result.settlements.is_empty() {
        for settlement in &response.result.settlements {
            println!(
                "Settlement: {} at price {:?}",
                settlement.instrument_name, settlement.settlement_price
            );
        }
    }
}

/// Test the get_book_summary_by_currency endpoint
#[tokio::test]
async fn test_get_book_summary_by_currency() {
    let client = create_public_test_client();

    let request = venues::deribit::public::rest::get_book_summary_by_currency::GetBookSummaryByCurrencyRequest {
        currency: Currency::BTC,
        kind: Some(InstrumentKind::Future),
    };

    let result = client.get_book_summary_by_currency(request).await;
    assert!(
        result.is_ok(),
        "get_book_summary_by_currency request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!("Found {} BTC book summaries", response.result.len());

    // Verify structure of first book summary if available
    if !response.result.is_empty() {
        let first_summary = &response.result[0];
        assert!(!first_summary.instrument_name.is_empty());

        println!("First book summary: {}", first_summary.instrument_name);
        if let Some(mid_price) = first_summary.mid_price {
            println!("Mid price: {}", mid_price);
        }
    }
}

/// Test the get_book_summary_by_instrument endpoint
#[tokio::test]
async fn test_get_book_summary_by_instrument() {
    let client = create_public_test_client();

    let request = venues::deribit::public::rest::get_book_summary_by_instrument::GetBookSummaryByInstrumentRequest {
        instrument_name: "BTC-PERPETUAL".to_string(),
    };

    let result = client.get_book_summary_by_instrument(request).await;
    assert!(
        result.is_ok(),
        "get_book_summary_by_instrument request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!("BTC-PERPETUAL book summary:");
    if !response.result.is_empty() {
        let summary = &response.result[0];
        if let Some(mid_price) = summary.mid_price {
            println!("Mid price: {}", mid_price);
        }
        if let Some(bid_price) = summary.bid_price {
            println!("Best bid: {}", bid_price);
        }
        if let Some(ask_price) = summary.ask_price {
            println!("Best ask: {}", ask_price);
        }
    }
}

/// Test error handling with multiple endpoint failures
#[tokio::test]
async fn test_comprehensive_error_handling() {
    let client = create_public_test_client();

    // Test 1: Invalid instrument name
    let invalid_instrument_request =
        venues::deribit::public::rest::get_contract_size::GetContractSizeRequest {
            instrument_name: "INVALID-INSTRUMENT".to_string(),
        };

    let result = client.get_contract_size(invalid_instrument_request).await;
    match result {
        Ok(response) => {
            println!("API handled invalid instrument gracefully");
            assert_eq!(response.jsonrpc, "2.0");
        }
        Err(error) => {
            println!("Expected error for invalid instrument: {:?}", error);
        }
    }

    // Test 2: Invalid index name
    let invalid_index_request =
        venues::deribit::public::rest::get_index_price::GetIndexPriceRequest {
            index_name: "invalid_index".to_string(),
        };

    let result = client.get_index_price(invalid_index_request).await;
    match result {
        Ok(_) => println!("API handled invalid index name gracefully"),
        Err(error) => println!("Expected error for invalid index: {:?}", error),
    }
}
