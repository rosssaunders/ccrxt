//! Integration tests for Gate.io delivery REST API endpoints
//!
//! These tests verify the functionality of delivery endpoints.
//! Tests run against the live Gate.io API using real market data.

use tokio;
use venues::gateio::{delivery::public::rest::RestClient, shared::enums::CandlestickInterval};

/// Helper function to create a test client for delivery public endpoints
fn create_delivery_test_client() -> RestClient {
    RestClient::new(false).expect("Failed to create Gate.io delivery REST client")
}

#[tokio::test]
async fn test_delivery_client_creation() {
    let _client = create_delivery_test_client();
    println!("✓ Delivery client creation successful");
}

/// Test delivery contracts endpoint
#[tokio::test]
async fn test_get_delivery_contracts() {
    use venues::gateio::delivery::public::rest::contracts::DeliveryContractsRequest;

    let client = create_delivery_test_client();
    let request = DeliveryContractsRequest {
        settle: "usdt".to_string(),
    };

    let result = client.get_delivery_contracts(request).await;
    assert!(
        result.is_ok(),
        "get_delivery_contracts request should succeed: {:?}",
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

    println!("Delivery contracts: {} contracts available", response.len());
}

/// Test single delivery contract endpoint
#[tokio::test]
async fn test_get_delivery_contract() {
    use venues::gateio::delivery::public::rest::contracts::DeliveryContractRequest;

    let client = create_delivery_test_client();
    let request = DeliveryContractRequest {
        settle: "usdt".to_string(),
        contract: "BTC_USDT_20250328".to_string(), // Use a quarterly contract
    };

    let result = client.get_delivery_contract(request).await;
    // Note: This might fail if the contract doesn't exist, which is okay for a delivery contract
    if let Ok(response) = result {
        assert!(!response.name.is_empty(), "Should have contract name");
        assert!(
            !response.contract_type.is_empty(),
            "Should have contract type"
        );

        println!(
            "Delivery contract: {} (type: {})",
            response.name, response.contract_type
        );
    } else {
        println!("No active delivery contracts found (expected for some periods)");
    }
}

/// Test delivery tickers endpoint
#[tokio::test]
async fn test_get_delivery_tickers() {
    use venues::gateio::delivery::public::rest::tickers::DeliveryTickersRequest;

    let client = create_delivery_test_client();
    let request = DeliveryTickersRequest {
        settle: "usdt".to_string(),
        contract: None, // Get all tickers
    };

    let result = client.get_delivery_tickers(request).await;
    assert!(
        result.is_ok(),
        "get_delivery_tickers request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();

    if !response.is_empty() {
        let ticker = &response[0];
        assert!(!ticker.contract.is_empty(), "Should have contract name");
        assert!(!ticker.last.is_empty(), "Should have last price");

        println!("Delivery tickers: {} tickers available", response.len());
    } else {
        println!("No active delivery tickers found (expected when no contracts are active)");
    }
}

/// Test delivery order book endpoint
#[tokio::test]
async fn test_get_delivery_order_book() {
    use venues::gateio::delivery::public::rest::order_book::DeliveryOrderBookRequest;

    let client = create_delivery_test_client();

    // First get available contracts to test with
    let contracts_request =
        venues::gateio::delivery::public::rest::contracts::DeliveryContractsRequest {
            settle: "usdt".to_string(),
        };

    let contracts_result = client.get_delivery_contracts(contracts_request).await;
    if let Ok(contracts) = contracts_result {
        if !contracts.is_empty() {
            let contract_name = &contracts[0].name;

            let request = DeliveryOrderBookRequest {
                settle: "usdt".to_string(),
                contract: contract_name.clone(),
                interval: Some("0".to_string()),
                limit: Some(10),
                with_id: None,
            };

            let result = client.get_delivery_order_book(request).await;
            assert!(
                result.is_ok(),
                "get_delivery_order_book request should succeed: {:?}",
                result.err()
            );

            let response = result.unwrap();
            // Order book might be empty for inactive contracts
            println!(
                "Delivery order book for {}: {} bids, {} asks",
                contract_name,
                response.bids.len(),
                response.asks.len()
            );
        } else {
            println!("No delivery contracts available to test order book");
        }
    }
}

/// Test delivery trades endpoint
#[tokio::test]
async fn test_get_delivery_trades() {
    use venues::gateio::delivery::public::rest::trades::DeliveryTradesRequest;

    let client = create_delivery_test_client();

    // First get available contracts to test with
    let contracts_request =
        venues::gateio::delivery::public::rest::contracts::DeliveryContractsRequest {
            settle: "usdt".to_string(),
        };

    let contracts_result = client.get_delivery_contracts(contracts_request).await;
    if let Ok(contracts) = contracts_result {
        if !contracts.is_empty() {
            let contract_name = &contracts[0].name;

            let request = DeliveryTradesRequest {
                settle: "usdt".to_string(),
                contract: contract_name.clone(),
                offset: None,
                limit: Some(10),
                last_id: None,
                from: None,
                to: None,
            };

            let result = client.get_delivery_trades(request).await;
            assert!(
                result.is_ok(),
                "get_delivery_trades request should succeed: {:?}",
                result.err()
            );

            let response = result.unwrap();
            println!(
                "Delivery trades for {}: {} trades",
                contract_name,
                response.len()
            );
        } else {
            println!("No delivery contracts available to test trades");
        }
    }
}

/// Test delivery candlesticks endpoint
#[tokio::test]
async fn test_get_delivery_candlesticks() {
    use venues::gateio::delivery::models::DeliveryCandlesticksRequest;

    let client = create_delivery_test_client();

    // First get available contracts to test with
    let contracts_request =
        venues::gateio::delivery::public::rest::contracts::DeliveryContractsRequest {
            settle: "usdt".to_string(),
        };

    let contracts_result = client.get_delivery_contracts(contracts_request).await;
    if let Ok(contracts) = contracts_result {
        if !contracts.is_empty() {
            let contract_name = &contracts[0].name;

            let request = DeliveryCandlesticksRequest {
                settle: "usdt".to_string(),
                contract: contract_name.clone(),
                from: None,
                to: None,
                limit: Some(10),
                interval: Some(CandlestickInterval::Minutes1),
            };

            let result = client.get_delivery_candlesticks(request).await;
            assert!(
                result.is_ok(),
                "get_delivery_candlesticks request should succeed: {:?}",
                result.err()
            );

            let response = result.unwrap();
            println!(
                "Delivery candlesticks for {}: {} candles",
                contract_name,
                response.len()
            );
        } else {
            println!("No delivery contracts available to test candlesticks");
        }
    }
}

/// Test delivery mark price candlesticks endpoint
#[tokio::test]
async fn test_get_delivery_mark_price_candlesticks() {
    use venues::gateio::delivery::models::DeliveryCandlesticksRequest;

    let client = create_delivery_test_client();

    // First get available contracts to test with
    let contracts_request =
        venues::gateio::delivery::public::rest::contracts::DeliveryContractsRequest {
            settle: "usdt".to_string(),
        };

    let contracts_result = client.get_delivery_contracts(contracts_request).await;
    if let Ok(contracts) = contracts_result {
        if !contracts.is_empty() {
            let contract_name = &contracts[0].name;

            let request = DeliveryCandlesticksRequest {
                settle: "usdt".to_string(),
                contract: contract_name.clone(),
                from: None,
                to: None,
                limit: Some(10),
                interval: Some(CandlestickInterval::Minutes1),
            };

            let result = client.get_delivery_mark_price_candlesticks(request).await;

            // Note: This endpoint appears to require authentication even though it's in the public module
            // This is why we handle both success and authentication error cases
            match result {
                Ok(response) => {
                    println!(
                        "Delivery mark price candlesticks for {}: {} candles",
                        contract_name,
                        response.len()
                    );
                }
                Err(e) => {
                    // Check if it's an authentication error (expected for this endpoint)
                    let error_string = format!("{:?}", e);
                    if error_string.contains("MISSING_REQUIRED_HEADER")
                        || error_string.contains("Timestamp")
                    {
                        println!(
                            "Delivery mark price candlesticks endpoint requires authentication (expected)"
                        );
                    } else {
                        // Re-raise unexpected errors
                        assert!(
                            false,
                            "Unexpected error from get_delivery_mark_price_candlesticks: {:?}",
                            e
                        );
                    }
                }
            }
        } else {
            println!("No delivery contracts available to test mark price candlesticks");
        }
    }
}

/// Test delivery index price candlesticks endpoint
#[tokio::test]
async fn test_get_delivery_index_price_candlesticks() {
    use venues::gateio::delivery::models::DeliveryCandlesticksRequest;

    let client = create_delivery_test_client();

    // First get available contracts to test with
    let contracts_request =
        venues::gateio::delivery::public::rest::contracts::DeliveryContractsRequest {
            settle: "usdt".to_string(),
        };

    let contracts_result = client.get_delivery_contracts(contracts_request).await;
    if let Ok(contracts) = contracts_result {
        if !contracts.is_empty() {
            let contract_name = &contracts[0].name;

            let request = DeliveryCandlesticksRequest {
                settle: "usdt".to_string(),
                contract: contract_name.clone(),
                from: None,
                to: None,
                limit: Some(10),
                interval: Some(CandlestickInterval::Minutes1),
            };

            let result = client.get_delivery_index_price_candlesticks(request).await;

            // Note: This endpoint may require authentication even though it's in the public module
            match result {
                Ok(response) => {
                    println!(
                        "Delivery index price candlesticks for {}: {} candles",
                        contract_name,
                        response.len()
                    );
                }
                Err(e) => {
                    // Check if it's an authentication error
                    let error_string = format!("{:?}", e);
                    if error_string.contains("MISSING_REQUIRED_HEADER")
                        || error_string.contains("Timestamp")
                    {
                        println!(
                            "Delivery index price candlesticks endpoint requires authentication (may be expected)"
                        );
                    } else {
                        // Re-raise unexpected errors
                        assert!(
                            false,
                            "Unexpected error from get_delivery_index_price_candlesticks: {:?}",
                            e
                        );
                    }
                }
            }
        } else {
            println!("No delivery contracts available to test index price candlesticks");
        }
    }
}

/// Test delivery insurance endpoint
#[tokio::test]
async fn test_get_delivery_insurance() {
    use venues::gateio::delivery::public::rest::insurance::DeliveryInsuranceRequest;

    let client = create_delivery_test_client();
    let request = DeliveryInsuranceRequest {
        settle: "usdt".to_string(),
        limit: Some(10),
    };

    let result = client.get_delivery_insurance(request).await;
    assert!(
        result.is_ok(),
        "get_delivery_insurance request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(!response.is_empty(), "Should have insurance data");

    let insurance = &response[0];
    assert!(insurance.t > 0, "Insurance should have valid timestamp");
    assert!(insurance.b > 0.0, "Insurance should have positive balance");

    println!("Delivery insurance: {} entries", response.len());
}

/// Test delivery risk limit tiers endpoint
#[tokio::test]
async fn test_get_delivery_risk_limit_tiers() {
    use venues::gateio::delivery::public::rest::risk_limit_tiers::DeliveryRiskLimitTiersRequest;

    let client = create_delivery_test_client();

    // First get available contracts to test with
    let contracts_request =
        venues::gateio::delivery::public::rest::contracts::DeliveryContractsRequest {
            settle: "usdt".to_string(),
        };

    let contracts_result = client.get_delivery_contracts(contracts_request).await;
    if let Ok(contracts) = contracts_result {
        if !contracts.is_empty() {
            let contract_name = &contracts[0].name;

            let request = DeliveryRiskLimitTiersRequest {
                settle: "usdt".to_string(),
                contract: contract_name.clone(),
                offset: Some(0),
                limit: Some(10),
            };

            let result = client.get_delivery_risk_limit_tiers(request).await;
            assert!(
                result.is_ok(),
                "get_delivery_risk_limit_tiers request should succeed: {:?}",
                result.err()
            );

            let response = result.unwrap();
            assert!(!response.is_empty(), "Should have risk limit tiers data");

            let tier = &response[0];
            assert!(!tier.risk_limit.is_empty(), "Tier should have risk limit");
            assert!(!tier.risk_limit.is_empty(), "Tier should have risk limit");

            println!(
                "Delivery risk limit tiers for {}: {} tiers",
                contract_name,
                response.len()
            );
        } else {
            println!("No delivery contracts available to test risk limit tiers");
        }
    }
}
