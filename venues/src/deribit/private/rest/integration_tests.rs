//! Integration tests for Deribit private REST API functionality

use super::*;
use crate::deribit::{RateLimiter, AccountTier};
use rest::secrets::ExposableSecret;
use reqwest::Client;

/// A plain text implementation of ExposableSecret for testing purposes.
#[derive(Clone)]
struct TestSecret {
    secret: String,
}

impl ExposableSecret for TestSecret {
    fn expose_secret(&self) -> String {
        self.secret.clone()
    }
}

impl TestSecret {
    fn new(secret: String) -> Self {
        Self { secret }
    }
}

#[test]
fn test_deribit_client_creation_and_request_building() {
    // Create a test client
    let api_key = Box::new(TestSecret::new("test_api_key".to_string())) as Box<dyn ExposableSecret>;
    let api_secret = Box::new(TestSecret::new("test_api_secret".to_string())) as Box<dyn ExposableSecret>;
    let client = Client::new();
    let rate_limiter = RateLimiter::new(AccountTier::Tier1);

    let deribit_client = RestClient::new(
        api_key,
        api_secret,
        "https://test.deribit.com",
        client,
        rate_limiter,
    );

    // Create a sample request
    let request = UpdateInAddressBookRequest {
        currency: Currency::Btc,
        address_type: AddressType::Withdrawal,
        address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
        beneficiary_vasp_name: "Example Exchange".to_string(),
        beneficiary_vasp_did: "did:example:123456789".to_string(),
        beneficiary_vasp_website: Some("https://example-exchange.com".to_string()),
        beneficiary_first_name: Some("John".to_string()),
        beneficiary_last_name: Some("Doe".to_string()),
        beneficiary_company_name: None,
        beneficiary_address: "123 Main Street, Bitcoin City, BC 12345".to_string(),
        agreed: true,
        personal: false,
        label: "Primary Withdrawal Address".to_string(),
    };

    // Verify the request can be serialized
    let serialized = serde_json::to_value(&request).unwrap();
    assert_eq!(serialized["currency"], "BTC");
    assert_eq!(serialized["type"], "withdrawal");
    assert!(serialized["agreed"].as_bool().unwrap());

    // Verify client was created successfully
    assert_eq!(deribit_client.base_url, "https://test.deribit.com");
}

#[test]
fn test_all_currency_variants() {
    let currencies = vec![
        Currency::Btc,
        Currency::Eth,
        Currency::Steth,
        Currency::Ethw,
        Currency::Usdc,
        Currency::Usdt,
        Currency::Eurr,
        Currency::Matic,
        Currency::Sol,
        Currency::Xrp,
        Currency::Usyc,
        Currency::Paxg,
        Currency::Bnb,
        Currency::Usde,
    ];

    for currency in currencies {
        let request = UpdateInAddressBookRequest {
            currency,
            address_type: AddressType::Transfer,
            address: "test_address".to_string(),
            beneficiary_vasp_name: "Test VASP".to_string(),
            beneficiary_vasp_did: "did:test:123".to_string(),
            beneficiary_vasp_website: None,
            beneficiary_first_name: None,
            beneficiary_last_name: None,
            beneficiary_company_name: Some("Test Company".to_string()),
            beneficiary_address: "Test Address".to_string(),
            agreed: true,
            personal: false,
            label: "Test Label".to_string(),
        };

        // Should serialize without errors
        let _serialized = serde_json::to_value(&request).unwrap();
    }
}

#[test]
fn test_all_address_type_variants() {
    let address_types = vec![
        AddressType::Transfer,
        AddressType::Withdrawal,
        AddressType::DepositSource,
    ];

    for address_type in address_types {
        let request = UpdateInAddressBookRequest {
            currency: Currency::Usdc,
            address_type,
            address: "test_address".to_string(),
            beneficiary_vasp_name: "Test VASP".to_string(),
            beneficiary_vasp_did: "did:test:123".to_string(),
            beneficiary_vasp_website: None,
            beneficiary_first_name: Some("Test".to_string()),
            beneficiary_last_name: Some("User".to_string()),
            beneficiary_company_name: None,
            beneficiary_address: "Test Address".to_string(),
            agreed: true,
            personal: true,
            label: "Test Label".to_string(),
        };

        // Should serialize without errors
        let _serialized = serde_json::to_value(&request).unwrap();
    }
}