//! Integration tests for Bitmart contract public REST API endpoints.
//!
//! These tests verify that the Bitmart contract public REST API client can successfully
//! communicate with the live API and receive valid responses.

use venues::bitmart::contract::{
    public::rest::GetContractDetailsRequest, public_client::RestClient,
};

/// Helper function to create a test client
fn create_contract_test_client() -> RestClient {
    use std::sync::Arc;
    let http_client = Arc::new(rest::native::NativeHttpClient::default());
    RestClient::new(http_client)
}

/// Test get contract details endpoint with no symbol (get all contracts)
#[tokio::test]
async fn test_get_all_contract_details() {
    let client = create_contract_test_client();
    let request = GetContractDetailsRequest { symbol: None };

    let result = client.get_contract_details(&request).await;
    assert!(
        result.is_ok(),
        "get_contract_details (all) should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, 1000, "Response should have success code");

    if let Some(data) = response.data {
        // Try to parse the JSON data - it could be an array or an object
        if let Ok(contracts) = serde_json::from_value::<
            Vec<venues::bitmart::contract::public::rest::ContractDetails>,
        >(data.clone())
        {
            println!("Found {} contracts", contracts.len());
            if let Some(first_contract) = contracts.first() {
                println!(
                    "First contract: {} (status: {:?})",
                    first_contract.symbol, first_contract.status
                );
            }
        } else {
            println!("Could not parse contract data - unexpected format");
        }
    } else {
        println!("No contract data returned (may be valid if no active contracts)");
    }
}

/// Test get contract details endpoint for specific symbol
#[tokio::test]
async fn test_get_specific_contract_details() {
    let client = create_contract_test_client();
    let request = GetContractDetailsRequest {
        symbol: Some("BTCUSDT".into()),
    };

    let result = client.get_contract_details(&request).await;
    assert!(
        result.is_ok(),
        "get_contract_details (specific) should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, 1000, "Response should have success code");

    if let Some(data) = response.data {
        // Try to parse the JSON data
        if let Ok(contracts) = serde_json::from_value::<
            Vec<venues::bitmart::contract::public::rest::ContractDetails>,
        >(data.clone())
        {
            if !contracts.is_empty() {
                let contract = &contracts[0];
                assert_eq!(contract.symbol, "BTCUSDT", "Should have correct symbol");
                println!(
                    "Contract details for {}: status={:?}",
                    contract.symbol, contract.status
                );
            } else {
                println!("No BTCUSDT contract found (may not be available)");
            }
        } else {
            println!("Could not parse contract data - unexpected format");
        }
    }
}

/// Test client creation and basic functionality
#[tokio::test]
async fn test_contract_client_creation() {
    let _client = create_contract_test_client();
    println!("✓ Bitmart contract client creation successful");
}

/// Test error handling with invalid symbol
#[tokio::test]
async fn test_error_handling_invalid_symbol() {
    let client = create_contract_test_client();
    let request = GetContractDetailsRequest {
        symbol: Some("INVALID_CONTRACT_XYZ".into()),
    };

    let result = client.get_contract_details(&request).await;

    // This should either fail or return empty results
    if result.is_err() {
        println!("Expected error for invalid symbol: {:?}", result.err());
    } else {
        let response = result.unwrap();
        if let Some(data) = response.data {
            if let Ok(contracts) = serde_json::from_value::<
                Vec<venues::bitmart::contract::public::rest::ContractDetails>,
            >(data)
            {
                if contracts.is_empty() {
                    println!("Empty response for invalid symbol (expected behavior)");
                } else {
                    println!("API returned contracts for invalid symbol (may be valid behavior)");
                }
            } else {
                println!("Could not parse response data");
            }
        }
    }
}

/// Test rate limiting behavior
#[tokio::test]
async fn test_rate_limiting() {
    let client = create_contract_test_client();

    // Make multiple rapid requests to test rate limiting
    let mut results = Vec::new();

    for i in 0..3 {
        let request = GetContractDetailsRequest { symbol: None };
        let result = client.get_contract_details(&request).await;
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
        successful_count >= 2,
        "At least 2 out of 3 requests should succeed"
    );

    println!(
        "Rate limiting test: {}/3 requests succeeded",
        successful_count
    );
}
