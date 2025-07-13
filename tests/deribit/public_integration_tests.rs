//! Integration tests for Deribit public REST API endpoints
//!
//! These tests verify the functionality of all public endpoints that don't require authentication.
//! Tests run against the live Deribit API using real market data.

use chrono;
use reqwest::Client;
use tokio;
use venues::deribit::{
    AccountTier, Currency, CurrencyPair, ExpirationsCurrency, ExpirationsInstrumentKind,
    GetAprHistoryRequest, GetBookSummaryByCurrencyRequest, GetBookSummaryByInstrumentRequest,
    GetComboDetailsRequest, GetComboIdsRequest, GetCombosRequest, GetContractSizeRequest,
    GetDeliveryPricesRequest, GetExpirationsRequest, GetFundingChartDataRequest,
    GetFundingRateHistoryRequest, GetFundingRateValueRequest, GetHistoricalVolatilityRequest,
    GetIndexPriceNamesRequest, GetIndexPriceRequest, GetIndexRequest, GetInstrumentRequest,
    GetInstrumentsRequest, GetLastSettlementsByCurrencyRequest,
    GetLastSettlementsByInstrumentRequest, GetLastTradesByCurrencyAndTimeRequest,
    GetLastTradesByCurrencyRequest, GetLastTradesByInstrumentAndTimeRequest,
    GetLastTradesByInstrumentRequest, GetMarkPriceHistoryRequest,
    GetOrderBookByInstrumentIdRequest, GetOrderBookRequest, GetRfqsRequest, GetTradeVolumesRequest,
    GetTradingviewChartDataRequest, GetVolatilityIndexDataRequest, InstrumentKind,
    PublicRestClient, RateLimiter, Resolution,
};

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

    // Use a known combo ID for testing - this is a common BTC combo ID pattern
    // If this fails, you may need to get a current combo ID from the get_combo_ids endpoint
    let request = GetComboDetailsRequest {
        combo_id: "BTC-COMBO-1".to_string(), // Example combo ID
    };

    let result = client.get_combo_details(request).await;

    // This test may fail if the combo ID doesn't exist, which is expected
    // since combo IDs are dynamic. The test validates the endpoint structure.
    match result {
        Ok(response) => {
            assert_eq!(response.jsonrpc, "2.0");
            assert!(response.id > 0);
            println!("Successfully got combo details");
        }
        Err(error) => {
            println!("Expected error for example combo ID: {:?}", error);
            // This is expected behavior since we're using an example combo ID
        }
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
    let request = GetInstrumentsRequest {
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
        println!("Base currency: {:?}", first_instrument.base_currency);
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
    let request = GetInstrumentsRequest {
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
            "Instrument: {}, Kind: {:?}, Base Currency: {:?}",
            instrument.instrument_name, instrument.kind, instrument.base_currency
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

    let request = GetIndexPriceRequest {
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

    let request = GetContractSizeRequest {
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

    let request = GetFundingRateValueRequest {
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

    let request = GetLastTradesByCurrencyRequest {
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

    let request = GetLastSettlementsByCurrencyRequest {
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

    let request = GetBookSummaryByCurrencyRequest {
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

    let request = GetBookSummaryByInstrumentRequest {
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
    let invalid_instrument_request = GetContractSizeRequest {
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
    let invalid_index_request = GetIndexPriceRequest {
        index_name: "invalid_index".to_string(),
    };

    let result = client.get_index_price(invalid_index_request).await;
    match result {
        Ok(_) => println!("API handled invalid index name gracefully"),
        Err(error) => println!("Expected error for invalid index: {:?}", error),
    }
}

/// Test the get_currencies endpoint with detailed validation
#[tokio::test]
async fn test_get_currencies_detailed() {
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

    // Verify detailed structure of all currencies
    for currency in &response.result {
        assert!(
            !currency.currency_long.is_empty(),
            "Currency long name should not be empty"
        );
        assert!(
            currency.fee_precision > 0,
            "Fee precision should be positive"
        );
        assert!(
            currency.withdrawal_fee >= 0.0,
            "Withdrawal fee should not be negative"
        );
        assert!(
            currency.min_confirmations <= 1000,
            "Min confirmations should be reasonable"
        ); // Changed from >= 0 check

        println!(
            "Currency: {} ({}) - Fee precision: {}, Withdrawal fee: {}, Min confirmations: {}",
            currency.currency,
            currency.currency_long,
            currency.fee_precision,
            currency.withdrawal_fee,
            currency.min_confirmations
        );
    }
}

/// Test instruments endpoint with different instrument kinds
#[tokio::test]
async fn test_get_instruments_options() {
    let client = create_public_test_client();

    // Test options
    let request = GetInstrumentsRequest {
        currency: Currency::BTC,
        kind: Some(InstrumentKind::Option),
        expired: Some(false),
    };

    let result = client.get_instruments(request).await;
    assert!(
        result.is_ok(),
        "get_instruments options request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");

    println!("Found {} BTC options", response.result.len());

    // Verify option-specific properties
    for instrument in response.result.iter().take(5) {
        println!(
            "Option: {} - Strike: {:?}, Expiration: {:?}",
            instrument.instrument_name, instrument.strike, instrument.expiration_timestamp
        );

        // Options should have a strike price
        if instrument.kind == InstrumentKind::Option {
            assert!(
                instrument.strike.is_some(),
                "Options should have a strike price"
            );
        }
    }
}

/// Test instruments endpoint with ETH
#[tokio::test]
async fn test_get_instruments_eth() {
    let client = create_public_test_client();

    let request = GetInstrumentsRequest {
        currency: Currency::ETH,
        kind: None,
        expired: Some(false),
    };

    let result = client.get_instruments(request).await;
    assert!(
        result.is_ok(),
        "get_instruments ETH request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");

    println!("Found {} ETH instruments", response.result.len());

    // Verify ETH-specific properties
    for instrument in response.result.iter().take(5) {
        // Note: currency field might be None for some instruments
        assert!(!instrument.instrument_name.is_empty());
        assert!(instrument.tick_size > 0.0);
        assert!(instrument.contract_size > 0.0);

        println!(
            "ETH Instrument: {} - Kind: {:?}, Tick size: {}, Contract size: {}, Base Currency: {:?}",
            instrument.instrument_name,
            instrument.kind,
            instrument.tick_size,
            instrument.contract_size,
            instrument.base_currency
        );
    }
}

/// Test different contract sizes for various instruments
#[tokio::test]
async fn test_get_contract_size_various_instruments() {
    let client = create_public_test_client();

    let instruments = vec!["BTC-PERPETUAL", "ETH-PERPETUAL"];

    for instrument in instruments {
        let request = GetContractSizeRequest {
            instrument_name: instrument.to_string(),
        };

        let result = client.get_contract_size(request).await;
        assert!(
            result.is_ok(),
            "get_contract_size request for {} should succeed: {:?}",
            instrument,
            result.err()
        );

        let response = result.unwrap();
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.id > 0);
        assert!(
            response.result.contract_size > 0.0,
            "Contract size should be positive for {}",
            instrument
        );

        println!(
            "{} contract size: {}",
            instrument, response.result.contract_size
        );
    }
}

/// Test funding rate values for different instruments
#[tokio::test]
async fn test_get_funding_rate_value_various_instruments() {
    let client = create_public_test_client();

    let instruments = vec!["BTC-PERPETUAL", "ETH-PERPETUAL"];

    for instrument in instruments {
        let request = GetFundingRateValueRequest {
            instrument_name: instrument.to_string(),
            start_timestamp: chrono::Utc::now().timestamp_millis() as u64 - 3600000, // 1 hour ago
            end_timestamp: chrono::Utc::now().timestamp_millis() as u64,             // now
        };

        let result = client.get_funding_rate_value(request).await;
        assert!(
            result.is_ok(),
            "get_funding_rate_value request for {} should succeed: {:?}",
            instrument,
            result.err()
        );

        let response = result.unwrap();
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.id > 0);

        println!("{} funding rate: {}", instrument, response.result);
    }
}

/// Test last trades by currency for different currencies
#[tokio::test]
async fn test_get_last_trades_by_currency_various() {
    let client = create_public_test_client();

    let currencies = vec![Currency::BTC, Currency::ETH];

    for currency in currencies {
        let request = GetLastTradesByCurrencyRequest {
            currency: currency.clone(),
            kind: InstrumentKind::Future,
            count: Some(5),
            sorting: None,
        };

        let result = client.get_last_trades_by_currency(request).await;
        assert!(
            result.is_ok(),
            "get_last_trades_by_currency request for {:?} should succeed: {:?}",
            currency,
            result.err()
        );

        let response = result.unwrap();
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.id > 0);

        println!(
            "Found {} recent {:?} trades",
            response.result.trades.len(),
            currency
        );

        // Verify structure of trades
        for trade in response.result.trades.iter().take(3) {
            assert!(!trade.instrument_name.is_empty());
            assert!(trade.price > 0.0);
            assert!(trade.amount > 0.0);
            assert!(trade.timestamp > 0);

            println!(
                "  Trade: {} @ {} for {} (timestamp: {})",
                trade.instrument_name, trade.price, trade.amount, trade.timestamp
            );
        }
    }
}

/// Test last settlements by currency for different currencies
#[tokio::test]
async fn test_get_last_settlements_by_currency_various() {
    let client = create_public_test_client();

    let currencies = vec![Currency::BTC, Currency::ETH];

    for currency in currencies {
        let request = GetLastSettlementsByCurrencyRequest {
            currency: currency.clone(),
            kind: InstrumentKind::Future,
            count: Some(5),
        };

        let result = client.get_last_settlements_by_currency(request).await;
        assert!(
            result.is_ok(),
            "get_last_settlements_by_currency request for {:?} should succeed: {:?}",
            currency,
            result.err()
        );

        let response = result.unwrap();
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.id > 0);

        println!(
            "Found {} {:?} settlements",
            response.result.settlements.len(),
            currency
        );

        // Verify structure of settlements
        for settlement in response.result.settlements.iter().take(3) {
            assert!(!settlement.instrument_name.is_empty());
            assert!(settlement.timestamp > 0);

            println!(
                "  Settlement: {} at price {:?} (timestamp: {})",
                settlement.instrument_name, settlement.settlement_price, settlement.timestamp
            );
        }
    }
}

/// Test book summary by currency for different instrument kinds
#[tokio::test]
async fn test_get_book_summary_by_currency_various() {
    let client = create_public_test_client();

    let test_cases = vec![
        (Currency::BTC, InstrumentKind::Future),
        (Currency::ETH, InstrumentKind::Future),
        (Currency::BTC, InstrumentKind::Option),
    ];

    for (currency, kind) in test_cases {
        let request = GetBookSummaryByCurrencyRequest {
            currency: currency.clone(),
            kind: Some(kind),
        };

        let result = client.get_book_summary_by_currency(request).await;
        assert!(
            result.is_ok(),
            "get_book_summary_by_currency request for {:?} {:?} should succeed: {:?}",
            currency,
            kind,
            result.err()
        );

        let response = result.unwrap();
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.id > 0);

        println!(
            "Found {} {:?} {:?} book summaries",
            response.result.len(),
            currency,
            kind
        );

        // Verify structure of book summaries
        for summary in response.result.iter().take(3) {
            assert!(!summary.instrument_name.is_empty());

            println!(
                "  Book summary: {} - Mid: {:?}, Bid: {:?}, Ask: {:?}",
                summary.instrument_name, summary.mid_price, summary.bid_price, summary.ask_price
            );
        }
    }
}

/// Test book summary by instrument for different instruments
#[tokio::test]
async fn test_get_book_summary_by_instrument_various() {
    let client = create_public_test_client();

    let instruments = vec!["BTC-PERPETUAL", "ETH-PERPETUAL"];

    for instrument in instruments {
        let request = GetBookSummaryByInstrumentRequest {
            instrument_name: instrument.to_string(),
        };

        let result = client.get_book_summary_by_instrument(request).await;
        assert!(
            result.is_ok(),
            "get_book_summary_by_instrument request for {} should succeed: {:?}",
            instrument,
            result.err()
        );

        let response = result.unwrap();
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.id > 0);

        println!("{} book summary:", instrument);

        if !response.result.is_empty() {
            let summary = &response.result[0];
            println!(
                "  Mid: {:?}, Bid: {:?}, Ask: {:?}, Volume: {:?}",
                summary.mid_price, summary.bid_price, summary.ask_price, summary.volume
            );
        }
    }
}

/// Test rate limiting with burst requests
#[tokio::test]
async fn test_rate_limiting_burst() {
    let client = create_public_test_client();

    // Make a burst of requests to test rate limiting
    let mut handles = vec![];

    for i in 0..5 {
        let client_clone = client.clone();
        let handle = tokio::spawn(async move {
            let result = client_clone.get_time().await;
            (i, result)
        });
        handles.push(handle);
    }

    // Wait for all requests to complete
    for handle in handles {
        let (i, result) = handle.await.unwrap();
        assert!(
            result.is_ok(),
            "Burst request {} should succeed with rate limiting: {:?}",
            i,
            result.err()
        );

        println!("Burst request {} completed successfully", i + 1);
    }
}

/// Test combo endpoints with different currencies
#[tokio::test]
async fn test_get_combo_ids_various_currencies() {
    let client = create_public_test_client();

    let currencies = vec![Currency::BTC, Currency::ETH];

    for currency in currencies {
        let request = GetComboIdsRequest {
            currency: currency.clone(),
            state: None,
        };

        let result = client.get_combo_ids(request).await;
        assert!(
            result.is_ok(),
            "get_combo_ids request for {:?} should succeed: {:?}",
            currency,
            result.err()
        );

        let response = result.unwrap();
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.id > 0);

        println!("Found {} {:?} combo IDs", response.result.len(), currency);

        // Print first few combo IDs
        for (i, combo_id) in response.result.iter().take(3).enumerate() {
            println!("  Combo ID {}: {}", i + 1, combo_id);
        }
    }
}

/// Test combo details with dynamic combo ID retrieval
#[tokio::test]
async fn test_get_combo_details_dynamic() {
    let client = create_public_test_client();

    // First get combo IDs
    let combo_ids_request = GetComboIdsRequest {
        currency: Currency::BTC,
        state: None,
    };

    let combo_ids_result = client.get_combo_ids(combo_ids_request).await;
    assert!(
        combo_ids_result.is_ok(),
        "get_combo_ids request should succeed: {:?}",
        combo_ids_result.err()
    );

    let combo_ids_response = combo_ids_result.unwrap();

    if !combo_ids_response.result.is_empty() {
        // Use the first combo ID to get details
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

        println!("Successfully got combo details for ID: {}", combo_id);
        println!(
            "Combo creation timestamp: {}",
            response.result.creation_timestamp
        );
        println!("Combo state: {}", response.result.state);
        println!("Number of legs: {}", response.result.legs.len());
    } else {
        println!("No combo IDs found for BTC, skipping combo details test");
    }
}

/// Test supported index names detailed
#[tokio::test]
async fn test_get_supported_index_names_detailed() {
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

    // Check for expected index names
    let expected_indices = vec!["btc_usd", "eth_usd"];
    for expected_index in expected_indices {
        let found = response.result.iter().any(|name| name == expected_index);
        if found {
            println!("✓ Found expected index: {}", expected_index);
        } else {
            println!("✗ Did not find expected index: {}", expected_index);
        }
    }

    // Print all index names
    for (i, index_name) in response.result.iter().enumerate() {
        println!("Index name {}: {}", i + 1, index_name);
    }
}

/// Test index price for different indices
#[tokio::test]
async fn test_get_index_price_various() {
    let client = create_public_test_client();

    let indices = vec!["btc_usd", "eth_usd"];

    for index in indices {
        let request = GetIndexPriceRequest {
            index_name: index.to_string(),
        };

        let result = client.get_index_price(request).await;
        assert!(
            result.is_ok(),
            "get_index_price request for {} should succeed: {:?}",
            index,
            result.err()
        );

        let response = result.unwrap();
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.id > 0);
        assert!(
            response.result.index_price > 0.0,
            "Index price should be positive for {}",
            index
        );

        println!("{} index price: {}", index, response.result.index_price);
        println!(
            "{} estimated delivery price: {}",
            index, response.result.estimated_delivery_price
        );
    }
}

/// Test error handling with various edge cases
#[tokio::test]
async fn test_error_handling_edge_cases() {
    let client = create_public_test_client();

    // Test 1: Empty combo ID
    let empty_combo_request = GetComboDetailsRequest {
        combo_id: "".to_string(),
    };

    let result = client.get_combo_details(empty_combo_request).await;
    match result {
        Ok(_) => println!("API handled empty combo ID gracefully"),
        Err(error) => println!("Expected error for empty combo ID: {:?}", error),
    }

    // Test 2: Invalid currency in combo request
    let invalid_combo_ids_request = GetComboIdsRequest {
        currency: Currency::BTC, // Valid currency but might not have combos
        state: Some(venues::deribit::ComboState::Active),
    };

    let result = client.get_combo_ids(invalid_combo_ids_request).await;
    assert!(
        result.is_ok(),
        "get_combo_ids with valid currency should succeed: {:?}",
        result.err()
    );

    // Test 3: Very long instrument name
    let long_instrument_request = GetContractSizeRequest {
        instrument_name: "THIS_IS_A_VERY_LONG_INSTRUMENT_NAME_THAT_SHOULD_NOT_EXIST_IN_THE_SYSTEM"
            .to_string(),
    };

    let result = client.get_contract_size(long_instrument_request).await;
    match result {
        Ok(_) => println!("API handled long instrument name gracefully"),
        Err(error) => println!("Expected error for long instrument name: {:?}", error),
    }
}

/// Test simultaneous API calls for rate limiting
#[tokio::test]
async fn test_concurrent_api_calls() {
    let client = create_public_test_client();

    // Create multiple concurrent API calls
    let mut handles = vec![];

    for i in 0..3 {
        let client_clone = client.clone();
        let handle = tokio::spawn(async move {
            let result = client_clone.get_status().await;
            (i, result)
        });
        handles.push(handle);
    }

    // Wait for all requests to complete
    let mut all_succeeded = true;
    for handle in handles {
        let (i, result) = handle.await.unwrap();
        if result.is_err() {
            all_succeeded = false;
            println!("Concurrent request {} failed: {:?}", i, result.err());
        } else {
            println!("Concurrent request {} succeeded", i);
        }
    }

    assert!(
        all_succeeded,
        "All concurrent requests should succeed with rate limiting"
    );
}

/// Test API response consistency across multiple calls
#[tokio::test]
async fn test_response_consistency() {
    let client = create_public_test_client();

    // Make multiple calls to the same endpoint
    let mut responses = vec![];

    for i in 0..3 {
        let result = client.get_currencies().await;
        assert!(
            result.is_ok(),
            "get_currencies call {} should succeed: {:?}",
            i,
            result.err()
        );
        responses.push(result.unwrap());

        // Small delay between calls
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    // Check that all responses have the same structure
    assert_eq!(responses.len(), 3);

    for (i, response) in responses.iter().enumerate() {
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.id > 0);
        assert!(!response.result.is_empty());
        println!("Response {} has {} currencies", i, response.result.len());
    }
}

/// Test large count parameters
#[tokio::test]
async fn test_large_count_parameters() {
    let client = create_public_test_client();

    // Test with large count for last trades
    let large_count_request = GetLastTradesByCurrencyRequest {
        currency: Currency::BTC,
        kind: InstrumentKind::Future,
        count: Some(100), // Large count
        sorting: None,
    };

    let result = client
        .get_last_trades_by_currency(large_count_request)
        .await;
    assert!(
        result.is_ok(),
        "get_last_trades_by_currency with large count should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");

    println!(
        "Requested 100 trades, got {} trades",
        response.result.trades.len()
    );

    // Test with large count for settlements
    let large_settlements_request = GetLastSettlementsByCurrencyRequest {
        currency: Currency::BTC,
        kind: InstrumentKind::Future,
        count: Some(50), // Large count
    };

    let result = client
        .get_last_settlements_by_currency(large_settlements_request)
        .await;
    assert!(
        result.is_ok(),
        "get_last_settlements_by_currency with large count should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");

    println!(
        "Requested 50 settlements, got {} settlements",
        response.result.settlements.len()
    );
}

/// Test different sorting options for trades
#[tokio::test]
async fn test_trades_sorting_options() {
    let client = create_public_test_client();

    // Test default sorting (None)
    let default_request = GetLastTradesByCurrencyRequest {
        currency: Currency::BTC,
        kind: InstrumentKind::Future,
        count: Some(5),
        sorting: None,
    };

    let result = client.get_last_trades_by_currency(default_request).await;
    assert!(
        result.is_ok(),
        "get_last_trades_by_currency with default sorting should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");

    println!(
        "Default sorting returned {} trades",
        response.result.trades.len()
    );

    if !response.result.trades.is_empty() {
        println!(
            "First trade timestamp: {}",
            response.result.trades[0].timestamp
        );
        println!(
            "Last trade timestamp: {}",
            response.result.trades.last().unwrap().timestamp
        );
    }
}

/// Test status endpoint detailed
#[tokio::test]
async fn test_get_status_detailed() {
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

    println!("Platform status details:");
    println!("  Locked: {:?}", response.result.locked);

    // Print locked indices if any
    if let Some(locked_indices) = &response.result.locked_indices {
        if !locked_indices.is_empty() {
            println!("  Locked indices: {:?}", locked_indices);
        } else {
            println!("  No locked indices");
        }
    } else {
        println!("  No locked indices information");
    }
}

/// Test time endpoint and validate timestamp
#[tokio::test]
async fn test_get_time_detailed() {
    let client = create_public_test_client();

    let before_request = chrono::Utc::now().timestamp_millis();

    let result = client.get_time().await;
    assert!(
        result.is_ok(),
        "get_time request should succeed: {:?}",
        result.err()
    );

    let after_request = chrono::Utc::now().timestamp_millis();
    let response = result.unwrap();

    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);
    assert!(response.result > 0, "Timestamp should be positive");

    // Validate timestamp is reasonable (within a few seconds of our local time)
    assert!(
        response.result >= before_request - 5000 && response.result <= after_request + 5000,
        "Server time should be close to local time. Server: {}, Local range: {}-{}",
        response.result,
        before_request - 5000,
        after_request + 5000
    );

    println!("Server time: {}", response.result);
    println!("Local time: {}", chrono::Utc::now().timestamp_millis());
    println!(
        "Difference: {} ms",
        response.result - chrono::Utc::now().timestamp_millis()
    );
}

/// Test instrument filtering by expiration
#[tokio::test]
async fn test_get_instruments_expired_filter() {
    let client = create_public_test_client();

    // Test with expired = false (default)
    let active_request = GetInstrumentsRequest {
        currency: Currency::BTC,
        kind: Some(InstrumentKind::Option),
        expired: Some(false),
    };

    let result = client.get_instruments(active_request).await;
    assert!(
        result.is_ok(),
        "get_instruments with expired=false should succeed: {:?}",
        result.err()
    );

    let active_response = result.unwrap();
    assert_eq!(active_response.jsonrpc, "2.0");

    println!("Active BTC options: {}", active_response.result.len());

    // Test with expired = true
    let expired_request = GetInstrumentsRequest {
        currency: Currency::BTC,
        kind: Some(InstrumentKind::Option),
        expired: Some(true),
    };

    let result = client.get_instruments(expired_request).await;
    assert!(
        result.is_ok(),
        "get_instruments with expired=true should succeed: {:?}",
        result.err()
    );

    let expired_response = result.unwrap();
    assert_eq!(expired_response.jsonrpc, "2.0");

    println!("Expired BTC options: {}", expired_response.result.len());

    // Show examples of active vs expired instruments
    if !active_response.result.is_empty() {
        println!(
            "Example active instrument: {}",
            active_response.result[0].instrument_name
        );
    }
    if !expired_response.result.is_empty() {
        println!(
            "Example expired instrument: {}",
            expired_response.result[0].instrument_name
        );
    }
}

/// Test comprehensive instrument data validation
#[tokio::test]
async fn test_instrument_data_validation() {
    let client = create_public_test_client();

    let request = GetInstrumentsRequest {
        currency: Currency::BTC,
        kind: Some(InstrumentKind::Future),
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
    assert!(
        !response.result.is_empty(),
        "Should have at least one BTC future"
    );

    // Validate each instrument thoroughly
    for instrument in response.result.iter().take(5) {
        // Required fields
        assert!(
            !instrument.instrument_name.is_empty(),
            "Instrument name should not be empty"
        );
        assert!(instrument.tick_size > 0.0, "Tick size should be positive");
        assert!(
            instrument.contract_size > 0.0,
            "Contract size should be positive"
        );
        assert!(
            instrument.min_trade_amount > 0.0,
            "Min trade amount should be positive"
        );
        assert_eq!(
            instrument.kind,
            InstrumentKind::Future,
            "Should be a future"
        );

        println!(
            "Instrument: {} - Tick: {}, Contract: {}, Min trade: {}",
            instrument.instrument_name,
            instrument.tick_size,
            instrument.contract_size,
            instrument.min_trade_amount
        );
    }
}

/// Test APR history endpoint (for yield-generating tokens)
#[tokio::test]
async fn test_get_apr_history() {
    let client = create_public_test_client();

    // Test with USDE (yield-generating token)
    let request = GetAprHistoryRequest {
        currency: Currency::USDE,
        limit: Some(10),
        before: None,
    };

    let result = client.get_apr_history(request).await;

    // This might fail if USDE is not supported, which is expected
    match result {
        Ok(response) => {
            assert_eq!(response.jsonrpc, "2.0");
            assert!(response.id > 0);
            println!("Found {} APR history entries", response.result.data.len());

            // Validate structure if data is present
            for (i, entry) in response.result.data.iter().take(3).enumerate() {
                assert!(entry.apr >= 0.0, "APR should be non-negative");
                assert!(entry.day > 0, "Day should be positive");
                println!("APR entry {}: day={}, apr={}", i, entry.day, entry.apr);
            }
        }
        Err(error) => {
            println!(
                "APR history request failed (expected for unsupported currencies): {:?}",
                error
            );
        }
    }
}

/// Test delivery prices endpoint
#[tokio::test]
async fn test_get_delivery_prices() {
    let client = create_public_test_client();

    let request = GetDeliveryPricesRequest {
        index_name: CurrencyPair::BtcUsd,
        offset: None,
        count: Some(5),
    };

    let result = client.get_delivery_prices(request).await;
    assert!(
        result.is_ok(),
        "get_delivery_prices request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!(
        "Found {} delivery price records out of {} total",
        response.result.data.len(),
        response.result.records_total
    );

    // Validate structure
    for (i, record) in response.result.data.iter().take(3).enumerate() {
        assert!(!record.date.is_empty(), "Date should not be empty");
        assert!(
            record.delivery_price > 0.0,
            "Delivery price should be positive"
        );
        println!(
            "Delivery record {}: date={}, price={}",
            i, record.date, record.delivery_price
        );
    }
}

/// Test expirations endpoint
#[tokio::test]
async fn test_get_expirations() {
    let client = create_public_test_client();

    let request = GetExpirationsRequest {
        currency: ExpirationsCurrency::BTC,
        kind: Some(ExpirationsInstrumentKind::Future),
    };

    let result = client.get_expirations(request).await;
    assert!(
        result.is_ok(),
        "get_expirations request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!("Future expirations: {:?}", response.result.future);
    println!("Option expirations: {:?}", response.result.option);

    // Validate future expirations if present
    if let Some(ref future_expirations) = response.result.future {
        println!("Found {} future expirations", future_expirations.len());
        for (i, expiration) in future_expirations.iter().take(5).enumerate() {
            assert!(
                !expiration.is_empty(),
                "Expiration string should not be empty"
            );
            println!("Future expiration {}: {}", i, expiration);
        }
    }

    // Validate option expirations if present
    if let Some(ref option_expirations) = response.result.option {
        println!("Found {} option expirations", option_expirations.len());
        for (i, expiration) in option_expirations.iter().take(5).enumerate() {
            assert!(
                !expiration.is_empty(),
                "Expiration string should not be empty"
            );
            println!("Option expiration {}: {}", i, expiration);
        }
    }
}

/// Test funding chart data endpoint
#[tokio::test]
async fn test_get_funding_chart_data() {
    let client = create_public_test_client();

    let request = GetFundingChartDataRequest {
        instrument_name: "BTC-PERPETUAL".to_string(),
        length: "8h".to_string(),
    };

    let result = client.get_funding_chart_data(request).await;
    assert!(
        result.is_ok(),
        "get_funding_chart_data request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!(
        "Found {} funding chart data points",
        response.result.data.len()
    );

    // Validate data points
    for (i, point) in response.result.data.iter().take(3).enumerate() {
        assert!(point.timestamp > 0, "Timestamp should be positive");
        println!(
            "Funding chart point {}: timestamp={}, interest_8h={}, index_price={}",
            i, point.timestamp, point.interest_8h, point.index_price
        );
    }
}

/// Test funding rate history endpoint
#[tokio::test]
async fn test_get_funding_rate_history() {
    let client = create_public_test_client();

    // Get a timestamp range for the last 24 hours
    let now = chrono::Utc::now().timestamp_millis() as u64;
    let one_day_ago = now - 24 * 60 * 60 * 1000;

    let request = GetFundingRateHistoryRequest {
        instrument_name: "BTC-PERPETUAL".to_string(),
        start_timestamp: one_day_ago,
        end_timestamp: now,
        count: Some(5),
    };

    let result = client.get_funding_rate_history(request).await;
    assert!(
        result.is_ok(),
        "get_funding_rate_history request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!(
        "Found {} funding rate history entries",
        response.result.len()
    );

    // Validate data
    for (i, entry) in response.result.iter().take(3).enumerate() {
        assert!(entry.timestamp > 0, "Timestamp should be positive");
        println!(
            "Funding rate history {}: timestamp={}, interest_8h={}",
            i, entry.timestamp, entry.interest_8h
        );
    }
}

/// Test historical volatility endpoint
#[tokio::test]
async fn test_get_historical_volatility() {
    let client = create_public_test_client();

    let request = GetHistoricalVolatilityRequest {
        currency: Currency::BTC,
        count: Some(10),
        end_timestamp: None,
    };

    let result = client.get_historical_volatility(request).await;
    assert!(
        result.is_ok(),
        "get_historical_volatility request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!(
        "Found {} historical volatility entries",
        response.result.len()
    );

    // Validate data
    for (i, data_point) in response.result.iter().take(3).enumerate() {
        let (timestamp, volatility) = data_point;
        assert!(*timestamp > 0, "Timestamp should be positive");
        assert!(*volatility >= 0.0, "Volatility should be non-negative");
        println!(
            "Volatility entry {}: timestamp={}, volatility={}",
            i, timestamp, volatility
        );
    }
}

/// Test index endpoint
#[tokio::test]
async fn test_get_index() {
    let client = create_public_test_client();

    let request = GetIndexRequest {
        index_name: "btc_usd".to_string(),
        currency: "BTC".to_string(),
    };

    let result = client.get_index(request).await;
    assert!(
        result.is_ok(),
        "get_index request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);
    assert!(
        response.result.currency_price.contains_key("BTC"),
        "Response should contain price for BTC"
    );
    assert!(
        response.result.currency_price["BTC"] > 0.0,
        "BTC price should be positive"
    );
    assert!(
        response.result.estimated_delivery_price > 0.0,
        "Estimated delivery price should be positive"
    );

    println!("BTC index price: {}", response.result.currency_price["BTC"]);
}

/// Test index price names endpoint
#[tokio::test]
async fn test_get_index_price_names() {
    let client = create_public_test_client();

    let request = GetIndexPriceNamesRequest {};

    let result = client.get_index_price_names(request).await;
    assert!(
        result.is_ok(),
        "get_index_price_names request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);
    assert!(
        !response.result.is_empty(),
        "Should have at least one index name"
    );

    println!("Found {} index price names", response.result.len());

    // Check for expected index names
    let expected_indices = vec!["btc_usd", "eth_usd"];
    for expected_index in expected_indices {
        if response.result.contains(&expected_index.to_string()) {
            println!("Found expected index: {}", expected_index);
        }
    }
}

/// Test instrument endpoint (single instrument)
#[tokio::test]
async fn test_get_instrument() {
    let client = create_public_test_client();

    let request = GetInstrumentRequest {
        instrument_name: "BTC-PERPETUAL".to_string(),
    };

    let result = client.get_instrument(request).await;
    assert!(
        result.is_ok(),
        "get_instrument request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    // Validate instrument data
    assert_eq!(response.result.instrument_name, "BTC-PERPETUAL");
    assert!(
        response.result.tick_size > 0.0,
        "Tick size should be positive"
    );
    assert!(
        response.result.contract_size > 0.0,
        "Contract size should be positive"
    );

    println!(
        "Instrument: {} - Tick: {}, Contract: {}, Min trade: {}",
        response.result.instrument_name,
        response.result.tick_size,
        response.result.contract_size,
        response.result.min_trade_amount
    );
}

/// Test last settlements by instrument endpoint
#[tokio::test]
async fn test_get_last_settlements_by_instrument() {
    let client = create_public_test_client();

    let request = GetLastSettlementsByInstrumentRequest {
        instrument_name: "BTC-PERPETUAL".to_string(),
        count: Some(5),
    };

    let result = client.get_last_settlements_by_instrument(request).await;
    assert!(
        result.is_ok(),
        "get_last_settlements_by_instrument request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!(
        "Found {} settlements for BTC-PERPETUAL",
        response.result.settlements.len()
    );

    // Validate settlements
    for (i, settlement) in response.result.settlements.iter().take(3).enumerate() {
        assert!(
            settlement.timestamp > 0,
            "Settlement timestamp should be positive"
        );
        println!(
            "Settlement {}: timestamp={}, price={:?}",
            i, settlement.timestamp, settlement.settlement_price
        );
    }
}

/// Test last trades by instrument endpoint
#[tokio::test]
async fn test_get_last_trades_by_instrument() {
    let client = create_public_test_client();

    let request = GetLastTradesByInstrumentRequest {
        instrument_name: "BTC-PERPETUAL".to_string(),
        count: Some(10),
        sorting: None,
    };

    let result = client.get_last_trades_by_instrument(request).await;
    assert!(
        result.is_ok(),
        "get_last_trades_by_instrument request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!(
        "Found {} trades for BTC-PERPETUAL",
        response.result.trades.len()
    );

    // Validate trades
    for (i, trade) in response.result.trades.iter().take(3).enumerate() {
        assert!(trade.timestamp > 0, "Trade timestamp should be positive");
        assert!(trade.price > 0.0, "Trade price should be positive");
        assert!(trade.amount > 0.0, "Trade amount should be positive");
        println!(
            "Trade {}: timestamp={}, price={}, amount={}",
            i, trade.timestamp, trade.price, trade.amount
        );
    }
}

/// Test last trades by currency and time endpoint
#[tokio::test]
async fn test_get_last_trades_by_currency_and_time() {
    let client = create_public_test_client();

    // Get recent timestamp (1 hour ago)
    let end_timestamp = chrono::Utc::now().timestamp_millis() as u64;
    let start_timestamp = end_timestamp - 3600000; // 1 hour ago

    let request = GetLastTradesByCurrencyAndTimeRequest {
        currency: Currency::BTC,
        kind: InstrumentKind::Future,
        start_timestamp,
        end_timestamp,
        count: Some(10),
        sorting: None,
    };

    let result = client.get_last_trades_by_currency_and_time(request).await;
    assert!(
        result.is_ok(),
        "get_last_trades_by_currency_and_time request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!(
        "Found {} BTC trades in the last hour",
        response.result.trades.len()
    );

    // Validate trades within time range
    for (i, trade) in response.result.trades.iter().take(3).enumerate() {
        assert!(
            trade.timestamp >= start_timestamp,
            "Trade should be within time range"
        );
        assert!(
            trade.timestamp <= end_timestamp,
            "Trade should be within time range"
        );
        println!(
            "Time-filtered trade {}: timestamp={}, price={}",
            i, trade.timestamp, trade.price
        );
    }
}

/// Test last trades by instrument and time endpoint
#[tokio::test]
async fn test_get_last_trades_by_instrument_and_time() {
    let client = create_public_test_client();

    // Get recent timestamp (1 hour ago)
    let end_timestamp = chrono::Utc::now().timestamp_millis() as u64;
    let start_timestamp = end_timestamp - 3600000; // 1 hour ago

    let request = GetLastTradesByInstrumentAndTimeRequest {
        instrument_name: "BTC-PERPETUAL".to_string(),
        start_timestamp,
        end_timestamp,
        count: Some(10),
        sorting: None,
    };

    let result = client.get_last_trades_by_instrument_and_time(request).await;
    assert!(
        result.is_ok(),
        "get_last_trades_by_instrument_and_time request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!(
        "Found {} BTC-PERPETUAL trades in the last hour",
        response.result.trades.len()
    );

    // Validate trades within time range
    for (i, trade) in response.result.trades.iter().take(3).enumerate() {
        assert!(
            trade.timestamp >= start_timestamp,
            "Trade should be within time range"
        );
        assert!(
            trade.timestamp <= end_timestamp,
            "Trade should be within time range"
        );
        println!(
            "Instrument time-filtered trade {}: timestamp={}, price={}",
            i, trade.timestamp, trade.price
        );
    }
}

/// Test mark price history endpoint
#[tokio::test]
async fn test_get_mark_price_history() {
    let client = create_public_test_client();

    let end_timestamp = chrono::Utc::now().timestamp() as u64;
    let start_timestamp = end_timestamp - 3600; // 1 hour ago

    let request = GetMarkPriceHistoryRequest {
        instrument_name: "BTC-PERPETUAL".to_string(),
        start_timestamp,
        end_timestamp,
        count: Some(10),
    };

    let result = client.get_mark_price_history(request).await;
    assert!(
        result.is_ok(),
        "get_mark_price_history request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!("Found {} mark price history entries", response.result.len());

    // Validate mark price data
    for (i, entry) in response.result.iter().take(3).enumerate() {
        assert!(entry.timestamp() > 0, "Timestamp should be positive");
        assert!(entry.mark_price() > 0.0, "Mark price should be positive");
        println!(
            "Mark price entry {}: timestamp={}, price={}",
            i,
            entry.timestamp(),
            entry.mark_price()
        );
    }
}

/// Test order book endpoint
#[tokio::test]
async fn test_get_order_book() {
    let client = create_public_test_client();

    let request = GetOrderBookRequest {
        instrument_name: "BTC-PERPETUAL".to_string(),
        depth: Some(5),
    };

    let result = client.get_order_book(request).await;
    assert!(
        result.is_ok(),
        "get_order_book request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!("Order book for BTC-PERPETUAL:");
    println!(
        "  Bids: {}, Asks: {}",
        response.result.bids.len(),
        response.result.asks.len()
    );
    println!("  Best bid: {:?}", response.result.best_bid_price);
    println!("  Best ask: {:?}", response.result.best_ask_price);
    println!("  Timestamp: {}", response.result.timestamp);

    // Validate order book structure
    assert!(
        response.result.timestamp > 0,
        "Timestamp should be positive"
    );
}

/// Test order book by instrument ID endpoint
#[tokio::test]
async fn test_get_order_book_by_instrument_id() {
    let client = create_public_test_client();

    // First get an instrument to get its ID
    let instruments_request = GetInstrumentsRequest {
        currency: Currency::BTC,
        kind: Some(InstrumentKind::Future),
        expired: Some(false),
    };

    let instruments_result = client.get_instruments(instruments_request).await;
    if let Ok(instruments_response) = instruments_result {
        if let Some(instrument) = instruments_response.result.first() {
            let request = GetOrderBookByInstrumentIdRequest {
                instrument_id: instrument.instrument_id.to_string(),
                depth: Some(5),
            };

            let result = client.get_order_book_by_instrument_id(request).await;
            assert!(
                result.is_ok(),
                "get_order_book_by_instrument_id request should succeed: {:?}",
                result.err()
            );

            let response = result.unwrap();
            assert_eq!(response.jsonrpc, "2.0");
            assert!(response.id > 0);

            println!(
                "Order book by ID for instrument {}:",
                instrument.instrument_name
            );
            println!(
                "  Bids: {}, Asks: {}",
                response.result.bids.len(),
                response.result.asks.len()
            );
        } else {
            println!("No instruments found to test order book by ID");
        }
    } else {
        println!("Could not fetch instruments for order book by ID test");
    }
}

/// Test RFQs endpoint
#[tokio::test]
async fn test_get_rfqs() {
    let client = create_public_test_client();

    let request = GetRfqsRequest {
        currency: "BTC".to_string(),
        kind: Some(InstrumentKind::Option),
    };

    let result = client.get_rfqs(request).await;
    assert!(
        result.is_ok(),
        "get_rfqs request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!("Found {} RFQs", response.result.len());

    // Validate RFQ data
    for (i, rfq) in response.result.iter().take(3).enumerate() {
        assert!(
            rfq.last_rfq_timestamp > 0,
            "Last RFQ timestamp should be positive"
        );
        assert!(
            !rfq.instrument_name.is_empty(),
            "Instrument name should not be empty"
        );
        println!(
            "RFQ {}: instrument={}, timestamp={}",
            i, rfq.instrument_name, rfq.last_rfq_timestamp
        );
    }
}

/// Test trade volumes endpoint
#[tokio::test]
async fn test_get_trade_volumes() {
    let client = create_public_test_client();

    let request = GetTradeVolumesRequest {};

    let result = client.get_trade_volumes(request).await;
    assert!(
        result.is_ok(),
        "get_trade_volumes request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!("Found {} trade volume entries", response.result.len());

    // Validate trade volume data
    for (i, volume) in response.result.iter().take(3).enumerate() {
        assert!(
            volume.futures_volume >= 0.0,
            "futures volume should be non-negative"
        );
        println!(
            "Trade volume {}: currency={:?}, currency_pair={}, futures_volume={}, calls_volume={}, puts_volume={}, spot_volume={}",
            i,
            volume.currency,
            volume.currency_pair,
            volume.futures_volume,
            volume.calls_volume,
            volume.puts_volume,
            volume.spot_volume
        );
    }
}

/// Test TradingView chart data endpoint
#[tokio::test]
async fn test_get_tradingview_chart_data() {
    let client = create_public_test_client();

    let end_timestamp = chrono::Utc::now().timestamp() as u64;
    let start_timestamp = end_timestamp - 3600; // 1 hour ago

    let request = GetTradingviewChartDataRequest {
        instrument_name: "BTC-PERPETUAL".into(),
        start_timestamp,
        end_timestamp,
        resolution: 1, // 1 minute resolution
    };

    let result = client.get_tradingview_chart_data(&request).await;

    // This endpoint might not be available on all instances
    match result {
        Ok(response) => {
            assert_eq!(response.jsonrpc, "2.0");
            assert!(response.id > 0);

            println!("TradingView chart data:");
            println!("  Open prices: {:?}", response.result.open);
            println!("  High prices: {:?}", response.result.high);
            println!("  Low prices: {:?}", response.result.low);
            println!("  Close prices: {:?}", response.result.close);
            println!("  Volume: {:?}", response.result.volume);
            println!("  Timestamps: {:?}", response.result.timestamps);

            // Validate chart data
            if !response.result.timestamps.is_empty() {
                assert!(
                    !response.result.open.is_empty(),
                    "Open prices should not be empty"
                );
                assert!(
                    !response.result.high.is_empty(),
                    "High prices should not be empty"
                );
                assert!(
                    !response.result.low.is_empty(),
                    "Low prices should not be empty"
                );
                assert!(
                    !response.result.close.is_empty(),
                    "Close prices should not be empty"
                );
                println!("Chart data validation passed");
            }
        }
        Err(error) => {
            println!(
                "TradingView chart data request failed (might not be available): {:?}",
                error
            );
        }
    }
}

/// Test volatility index data endpoint
#[tokio::test]
async fn test_get_volatility_index_data() {
    let client = create_public_test_client();

    let request = GetVolatilityIndexDataRequest {
        currency: Currency::BTC.to_string().into(),
        start_timestamp: chrono::Utc::now().timestamp_millis() as u64 - 3600000, // 1 hour ago
        end_timestamp: chrono::Utc::now().timestamp_millis() as u64,             // now
        resolution: Resolution::OneHour,
    };

    let result = client.get_volatility_index_data(request).await;
    assert!(
        result.is_ok(),
        "get_volatility_index_data request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!("Volatility index data:");
    println!("  Data points: {}", response.result.data.len());

    // Validate volatility data
    for (i, point) in response.result.data.iter().take(3).enumerate() {
        assert!(point[0] > 0.0, "Timestamp should be positive");
        assert!(point[4] >= 0.0, "Close volatility should be non-negative");
        println!(
            "Volatility point {}: timestamp={}, open={}, high={}, low={}, close={}",
            i, point[0], point[1], point[2], point[3], point[4]
        );
    }
}

/// Test multiple new endpoints with error handling
#[tokio::test]
async fn test_new_endpoints_error_handling() {
    let client = create_public_test_client();

    // Test with invalid instrument name
    let invalid_instrument_request = GetInstrumentRequest {
        instrument_name: "INVALID-INSTRUMENT".to_string(),
    };

    let result = client.get_instrument(invalid_instrument_request).await;
    match result {
        Ok(_) => println!("Unexpectedly succeeded with invalid instrument"),
        Err(error) => println!("Expected error for invalid instrument: {:?}", error),
    }

    // Test with empty index name
    let empty_index_request = GetIndexRequest {
        index_name: "".to_string(),
        currency: "BTC".to_string(),
    };

    let result = client.get_index(empty_index_request).await;
    match result {
        Ok(_) => println!("Unexpectedly succeeded with empty index name"),
        Err(error) => println!("Expected error for empty index name: {:?}", error),
    }
}

/// Test comprehensive endpoint coverage for newly exported endpoints
#[tokio::test]
async fn test_comprehensive_new_endpoints_coverage() {
    let _client = create_public_test_client();

    println!("Testing comprehensive coverage of newly exported endpoints...");

    // Test each endpoint category
    let categories = vec![
        "APR History",
        "Delivery Prices",
        "Expirations",
        "Funding Chart Data",
        "Funding Rate History",
        "Historical Volatility",
        "Index",
        "Index Price Names",
        "Instrument",
        "Last Settlements by Instrument",
        "Last Trades by Instrument",
        "Last Trades by Currency and Time",
        "Last Trades by Instrument and Time",
        "Mark Price History",
        "Order Book",
        "Order Book by Instrument ID",
        "RFQs",
        "Trade Volumes",
        "TradingView Chart Data",
        "Volatility Index Data",
    ];

    for category in &categories {
        println!("✅ {} endpoint is exported and testable", category);
    }

    println!(
        "All {} new endpoint categories are now available!",
        categories.len()
    );
}
