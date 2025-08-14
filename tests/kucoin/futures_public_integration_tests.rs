//! Integration tests for KuCoin futures public REST API endpoints.
//!
//! These tests verify that the KuCoin futures public REST API client can successfully
//! communicate with the live API and receive valid responses.

use std::sync::Arc;

use rest::native::NativeHttpClient;
use venues::kucoin::futures::public::rest::{
    GetAllContractsRequest, GetContractRequest, GetCurrentFundingRateRequest,
    RestClient as PublicRestClient,
};

/// Helper function to create a test client with shared rate limiter
fn create_futures_test_client() -> PublicRestClient {
    let http_client = Arc::new(NativeHttpClient::default());
    PublicRestClient::new_default(http_client)
}

/// Test get all contracts endpoint
#[tokio::test]
async fn test_get_all_contracts() {
    let client = create_futures_test_client();
    let request = GetAllContractsRequest {};

    let result = client.get_all_contracts(request).await;
    assert!(
        result.is_ok(),
        "get_all_contracts should succeed: {:?}",
        result.err()
    );

    let (response, _headers) = result.unwrap();
    let contracts = response.data;
    assert!(!contracts.is_empty(), "Should have at least one contract");

    println!("Found {} futures contracts", contracts.len());
    if let Some(first_contract) = contracts.first() {
        println!(
            "First contract: {} (type: {:?})",
            first_contract.symbol, first_contract.contract_type
        );
    }
}

/// Test get single contract endpoint
#[tokio::test]
async fn test_get_contract() {
    let client = create_futures_test_client();

    // First get available contracts to test with
    let all_contracts_request = GetAllContractsRequest {};
    let contracts_result = client.get_all_contracts(all_contracts_request).await;

    if let Ok((response, _)) = contracts_result {
        let contracts = response.data;
        if !contracts.is_empty() {
            let contract_symbol = &contracts[0].symbol;

            let request = GetContractRequest {
                symbol: contract_symbol.clone(),
            };

            let result = client.get_contract(request).await;
            assert!(
                result.is_ok(),
                "get_contract should succeed: {:?}",
                result.err()
            );

            let (response, _headers) = result.unwrap();
            let contract = response.data;
            assert_eq!(
                contract.symbol, *contract_symbol,
                "Should have correct symbol"
            );

            println!(
                "Contract details: {} - {} (status: {:?})",
                contract.symbol, contract.base_currency, contract.status
            );
        } else {
            println!("No futures contracts available to test single contract");
        }
    }
}

/// Test get current funding rate endpoint
#[tokio::test]
async fn test_get_current_funding_rate() {
    let client = create_futures_test_client();

    // First get available contracts to test with
    let all_contracts_request = GetAllContractsRequest {};
    let contracts_result = client.get_all_contracts(all_contracts_request).await;

    if let Ok((response, _)) = contracts_result {
        let contracts = response.data;
        if !contracts.is_empty() {
            let contract_symbol = &contracts[0].symbol;

            let request = GetCurrentFundingRateRequest {
                symbol: contract_symbol.clone(),
            };

            let result = client.get_current_funding_rate(request).await;
            assert!(
                result.is_ok(),
                "get_current_funding_rate should succeed: {:?}",
                result.err()
            );

            let (response, _headers) = result.unwrap();
            let funding_rate = response.data;
            // Note: funding rate symbol may differ from contract symbol
            assert!(!funding_rate.symbol.is_empty(), "Should have a symbol");

            println!(
                "Current funding rate for contract {} (funding symbol: {}): {} (granularity: {})",
                contract_symbol, funding_rate.symbol, funding_rate.value, funding_rate.granularity
            );
        } else {
            println!("No futures contracts available to test current funding rate");
        }
    }
}

/// Test client creation and basic functionality
#[tokio::test]
async fn test_futures_client_creation() {
    let _client = create_futures_test_client();
    println!("✓ KuCoin futures client creation successful");
}

/// Test comprehensive error handling
#[tokio::test]
async fn test_error_handling_invalid_symbol() {
    let client = create_futures_test_client();
    let request = GetContractRequest {
        symbol: "INVALID_SYMBOL_XYZ".to_string(),
    };

    let result = client.get_contract(request).await;
    // This should either fail or return empty results
    if result.is_err() {
        println!("Expected error for invalid symbol: {:?}", result.err());
    } else {
        println!("API returned response for invalid symbol (may be valid behavior)");
    }
}

/// Test rate limiting behavior
#[tokio::test]
async fn test_rate_limiting() {
    let client = create_futures_test_client();

    // Make multiple rapid requests to test rate limiting
    let mut results = Vec::new();

    for i in 0..5 {
        let request = GetAllContractsRequest {};
        let result = client.get_all_contracts(request).await;
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

    // At least some requests should succeed with reasonable rate limiting
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
