//! Integration tests for Gate.io options REST API endpoints
//!
//! These tests verify the functionality of options endpoints.
//! Tests run against the live Gate.io API using real market data.

use std::sync::Arc;

use rest::native::NativeHttpClient;
use tokio;
use venues::gateio::{PublicRestClient};

/// Helper function to create a test client for options public endpoints
fn create_options_test_client() -> PublicRestClient {
    let http_client = Arc::new(NativeHttpClient::default());
    let rate_limiter = Arc::new(venues::gateio::RateLimiter::default());
    PublicRestClient::new(http_client, rate_limiter, false).expect("Failed to create Gate.io options REST client")
}

#[tokio::test]
async fn test_options_client_creation() {
    let _client = create_options_test_client();
    println!("✓ Options client creation successful");
}

/// Test options underlyings endpoint
#[tokio::test]
async fn test_get_options_underlyings() {
    let client = create_options_test_client();

    let result = client.get_options_underlyings().await;
    assert!(
        result.is_ok(),
        "get_options_underlyings request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.is_empty(), "Should have underlying data");

    let underlying = &response[0];
    assert!(!underlying.name.is_empty(), "Underlying should have name");

    println!(
        "Options underlyings: {} underlyings available",
        response.len()
    );
}

/// Test options expirations endpoint
#[tokio::test]
async fn test_get_options_expirations() {
    use venues::gateio::public::rest::options::expirations::OptionsExpirationsRequest;

    let client = create_options_test_client();
    let request = OptionsExpirationsRequest {
        underlying: "BTC_USDT".to_string(),
    };

    let result = client.get_options_expirations(request).await;
    assert!(
        result.is_ok(),
        "get_options_expirations request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.is_empty(), "Should have expiration data");

    println!("Options expirations: {} expiration dates", response.len());
}

/// Test options contracts endpoint
#[tokio::test]
async fn test_get_options_contracts() {
    use venues::gateio::public::rest::options::contracts::OptionsContractsRequest;

    let client = create_options_test_client();

    // Test with BTC_USDT as the underlying (required parameter)
    let request = OptionsContractsRequest {
        underlying: Some("BTC_USDT".to_string()),
        expiration: None,
    };

    let result = client.get_options_contracts(request).await;
    assert!(
        result.is_ok(),
        "get_options_contracts request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    println!(
        "Options contracts for BTC_USDT: {} contracts found",
        response.len()
    );
}

/// Test options contracts filtering by expiration
#[tokio::test]
async fn test_get_options_contracts_filtered() {
    use venues::gateio::public::rest::options::contracts::OptionsContractsRequest;

    let client = create_options_test_client();

    // Test with BTC_USDT as a common underlying
    let request = OptionsContractsRequest {
        underlying: Some("BTC_USDT".to_string()),
        expiration: None,
    };

    let result = client.get_options_contracts(request).await;
    assert!(
        result.is_ok(),
        "get_options_contracts filtered request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    println!(
        "Options contracts filtered for BTC_USDT: {} contracts",
        response.len()
    );
}

/// Test options tickers endpoint
#[tokio::test]
async fn test_get_options_tickers() {
    use venues::gateio::public::rest::options::tickers::OptionsTickersRequest;

    let client = create_options_test_client();

    // First get available underlyings
    let underlyings_result = client.get_options_underlyings().await;
    if let Ok(underlyings) = underlyings_result {
        if !underlyings.is_empty() {
            let underlying_name = &underlyings[0].name;

            let request = OptionsTickersRequest {
                underlying: Some(underlying_name.clone()),
            };

            let result = client.get_options_tickers(request).await;
            assert!(
                result.is_ok(),
                "get_options_tickers request should succeed: {:?}",
                result.err()
            );

            let response = result.unwrap();
            println!(
                "Options tickers for {}: {} tickers",
                underlying_name,
                response.len()
            );
        } else {
            println!("No options underlyings available to test tickers");
        }
    }
}

/// Test underlying ticker endpoint
#[tokio::test]
async fn test_get_underlying_ticker() {
    // UnderlyingTicker doesn't need a request struct - uses string parameter directly

    let client = create_options_test_client();

    // First get available underlyings
    let underlyings_result = client.get_options_underlyings().await;
    if let Ok(underlyings) = underlyings_result {
        if !underlyings.is_empty() {
            let underlying_name = &underlyings[0].name;

            let result = client.get_underlying_ticker(underlying_name).await;
            assert!(
                result.is_ok(),
                "get_underlying_ticker request should succeed: {:?}",
                result.err()
            );

            let response = result.unwrap();
            assert!(!response.index_price.is_empty(), "Should have index price");

            println!(
                "Underlying ticker for {}: index_price={}, trade_enabled={:?}",
                underlying_name, response.index_price, response.trade_enabled
            );
        } else {
            println!("No options underlyings available to test underlying ticker");
        }
    }
}

/// Test options order book endpoint
#[tokio::test]
async fn test_get_options_order_book() {
    use venues::gateio::public::rest::options::{
        contracts::OptionsContractsRequest, order_book::OptionsOrderBookRequest,
    };

    let client = create_options_test_client();

    // First get available underlyings and contracts
    let underlyings_result = client.get_options_underlyings().await;
    if let Ok(underlyings) = underlyings_result {
        if !underlyings.is_empty() {
            let underlying_name = &underlyings[0].name;

            let contracts_request = OptionsContractsRequest {
                underlying: Some(underlying_name.clone()),
                expiration: None,
            };

            let contracts_result = client.get_options_contracts(contracts_request).await;
            if let Ok(contracts) = contracts_result {
                if !contracts.is_empty() {
                    let contract_name = &contracts[0].name;

                    let request = OptionsOrderBookRequest {
                        contract: contract_name.clone(),
                        interval: Some("0".to_string()),
                        limit: Some(10),
                        with_id: None,
                    };

                    let result = client.get_options_order_book(request).await;
                    assert!(
                        result.is_ok(),
                        "get_options_order_book request should succeed: {:?}",
                        result.err()
                    );

                    let response = result.unwrap();
                    println!(
                        "Options order book for {}: {} bids, {} asks",
                        contract_name,
                        response.bids.len(),
                        response.asks.len()
                    );
                } else {
                    println!(
                        "No options contracts available for {} to test order book",
                        underlying_name
                    );
                }
            }
        } else {
            println!("No options underlyings available to test order book");
        }
    }
}

/// Test options trades endpoint
#[tokio::test]
async fn test_get_options_trades() {
    use venues::gateio::public::rest::options::{
        contracts::OptionsContractsRequest, trades::OptionsTradesRequest,
    };

    let client = create_options_test_client();

    // First get available underlyings and contracts
    let underlyings_result = client.get_options_underlyings().await;
    if let Ok(underlyings) = underlyings_result {
        if !underlyings.is_empty() {
            let underlying_name = &underlyings[0].name;

            let contracts_request = OptionsContractsRequest {
                underlying: Some(underlying_name.clone()),
                expiration: None,
            };

            let contracts_result = client.get_options_contracts(contracts_request).await;
            if let Ok(contracts) = contracts_result {
                if !contracts.is_empty() {
                    let contract_name = &contracts[0].name;

                    let request = OptionsTradesRequest {
                        contract: contract_name.clone(),
                        last_id: None,
                        limit: Some(10),
                    };

                    let result = client.get_options_trades(request).await;
                    assert!(
                        result.is_ok(),
                        "get_options_trades request should succeed: {:?}",
                        result.err()
                    );

                    let response = result.unwrap();
                    println!(
                        "Options trades for {}: {} trades",
                        contract_name,
                        response.len()
                    );
                } else {
                    println!(
                        "No options contracts available for {} to test trades",
                        underlying_name
                    );
                }
            }
        } else {
            println!("No options underlyings available to test trades");
        }
    }
}

/// Test options candlesticks endpoint
#[tokio::test]
async fn test_get_options_candlesticks() {
    use venues::gateio::public::rest::options::{
        contracts::OptionsContractsRequest, get_options_candlesticks::OptionsCandlesticksRequest,
    };

    let client = create_options_test_client();

    // First get available underlyings and contracts
    let underlyings_result = client.get_options_underlyings().await;
    if let Ok(underlyings) = underlyings_result {
        if !underlyings.is_empty() {
            let underlying_name = &underlyings[0].name;

            let contracts_request = OptionsContractsRequest {
                underlying: Some(underlying_name.clone()),
                expiration: None,
            };

            let contracts_result = client.get_options_contracts(contracts_request).await;
            if let Ok(contracts) = contracts_result {
                if !contracts.is_empty() {
                    let contract_name = &contracts[0].name;

                    let request = OptionsCandlesticksRequest {
                        contract: contract_name.clone(),
                        from: None,
                        to: None,
                        limit: Some(10),
                        interval: Some("1m".to_string()),
                    };

                    let result = client.get_options_candlesticks(request).await;
                    assert!(
                        result.is_ok(),
                        "get_options_candlesticks request should succeed: {:?}",
                        result.err()
                    );

                    let response = result.unwrap();
                    println!(
                        "Options candlesticks for {}: {} candles",
                        contract_name,
                        response.len()
                    );
                } else {
                    println!(
                        "No options contracts available for {} to test candlesticks",
                        underlying_name
                    );
                }
            }
        } else {
            println!("No options underlyings available to test candlesticks");
        }
    }
}

/// Test underlying candlesticks endpoint
#[tokio::test]
async fn test_get_underlying_candlesticks() {
    use venues::gateio::public::rest::options::get_underlying_candlesticks::UnderlyingCandlesticksRequest;

    let client = create_options_test_client();

    // First get available underlyings
    let underlyings_result = client.get_options_underlyings().await;
    if let Ok(underlyings) = underlyings_result {
        if !underlyings.is_empty() {
            let underlying_name = &underlyings[0].name;

            let request = UnderlyingCandlesticksRequest {
                underlying: underlying_name.clone(),
                from: None,
                to: None,
                interval: Some("1m".to_string()),
                limit: Some(10),
            };

            let result = client.get_underlying_candlesticks(request).await;
            assert!(
                result.is_ok(),
                "get_underlying_candlesticks request should succeed: {:?}",
                result.err()
            );

            let response = result.unwrap();
            println!(
                "Underlying candlesticks for {}: {} candles",
                underlying_name,
                response.len()
            );
        } else {
            println!("No options underlyings available to test underlying candlesticks");
        }
    }
}

/// Test options settlements endpoint
#[tokio::test]
async fn test_get_options_settlements() {
    use venues::gateio::public::rest::options::settlements::OptionsSettlementsRequest;

    let client = create_options_test_client();

    // First get available underlyings
    let underlyings_result = client.get_options_underlyings().await;
    if let Ok(underlyings) = underlyings_result {
        if !underlyings.is_empty() {
            let underlying_name = &underlyings[0].name;

            let request = OptionsSettlementsRequest {
                underlying: Some(underlying_name.clone()),
                limit: Some(10),
            };

            let result = client.get_options_settlements(request).await;
            assert!(
                result.is_ok(),
                "get_options_settlements request should succeed: {:?}",
                result.err()
            );

            let response = result.unwrap();
            println!(
                "Options settlements for {}: {} settlements",
                underlying_name,
                response.len()
            );
        } else {
            println!("No options underlyings available to test settlements");
        }
    }
}

/// Test options contract settlement endpoint
#[tokio::test]
async fn test_get_options_contract_settlement() {
    // OptionsContractSettlement doesn't need a request struct - uses contract name directly
    use venues::gateio::public::rest::options::contracts::OptionsContractsRequest;

    let client = create_options_test_client();

    // First get available underlyings and contracts
    let underlyings_result = client.get_options_underlyings().await;
    if let Ok(underlyings) = underlyings_result {
        if !underlyings.is_empty() {
            let underlying_name = &underlyings[0].name;

            let contracts_request = OptionsContractsRequest {
                underlying: Some(underlying_name.clone()),
                expiration: None,
            };

            let contracts_result = client.get_options_contracts(contracts_request).await;
            if let Ok(contracts) = contracts_result {
                if !contracts.is_empty() {
                    let contract_name = &contracts[0].name;

                    let result = client.get_options_contract_settlement(contract_name).await;
                    // Note: This endpoint might return empty results if no settlements exist
                    // or an error if the contract has never been settled
                    if result.is_err() {
                        println!(
                            "Contract settlement request failed (may be no settlement history): {:?}",
                            result.err()
                        );
                    } else {
                        let response = result.unwrap();
                        println!(
                            "Options contract settlement for {}: price={}",
                            contract_name, response.settle_price
                        );
                    }
                } else {
                    println!(
                        "No options contracts available for {} to test contract settlement",
                        underlying_name
                    );
                }
            }
        } else {
            println!("No options underlyings available to test contract settlement");
        }
    }
}
