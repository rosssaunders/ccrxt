use serde::Serialize;

use crate::deribit::{
    AddressBookType, Currency, EndpointType, JsonRpcResult, PrivateRestClient, RestResult,
};

/// REST API endpoint constant
const UPDATE_IN_ADDRESS_BOOK_ENDPOINT: &str = "private/update_in_address_book";

/// Request parameters for update in address book
#[derive(Debug, Clone, Serialize)]
pub struct UpdateInAddressBookRequest {
    /// The currency symbol
    pub currency: Currency,

    /// Address book type
    #[serde(rename = "type")]
    pub address_type: AddressBookType,

    /// Address in currency format, it must be in address book
    pub address: String,

    /// Name of beneficiary VASP
    pub beneficiary_vasp_name: String,

    /// DID of beneficiary VASP
    pub beneficiary_vasp_did: String,

    /// Website of the beneficiary VASP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_vasp_website: Option<String>,

    /// First name of beneficiary (if beneficiary is a person)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_first_name: Option<String>,

    /// Last name of beneficiary (if beneficiary is a person)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_last_name: Option<String>,

    /// Beneficiary company name (if beneficiary is a company)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_company_name: Option<String>,

    /// Geographical address of the beneficiary
    pub beneficiary_address: String,

    /// Indicates that the user agreed to share provided information with 3rd parties
    pub agreed: bool,

    /// The user confirms that he provided address belongs to him and he has access to it via an un-hosted wallet software
    pub personal: bool,

    /// Label of the address book entry
    pub label: String,
}

/// Response for update in address book endpoint
pub type UpdateInAddressBookResponse = JsonRpcResult<String>;

impl PrivateRestClient {
    /// Update an entry in the address book.
    ///
    /// [docs](https://docs.deribit.com/v2/#private-update_in_address_book)
    ///
    /// Rate limit: 500 credits per request (non-matching engine)
    /// Scope: wallet:read_write
    ///
    /// # Arguments
    /// * `params` - Parameters for the update in address book request
    ///
    /// # Returns
    /// Response for update in address book endpoint
    pub async fn update_in_address_book(
        &self,
        params: UpdateInAddressBookRequest,
    ) -> RestResult<UpdateInAddressBookResponse> {
        self.send_signed_request(
            UPDATE_IN_ADDRESS_BOOK_ENDPOINT,
            &params,
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rest::secrets::SecretString;
    use serde_json::{Value, json};

    use super::*;
    use crate::deribit::{AccountTier, credentials::Credentials};

    #[test]
    fn test_request_parameters_serialization_full() {
        let request = UpdateInAddressBookRequest {
            currency: Currency::BTC,
            address_type: AddressBookType::Withdrawal,
            address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            beneficiary_vasp_name: "Example VASP".to_string(),
            beneficiary_vasp_did: "did:example:123456".to_string(),
            beneficiary_vasp_website: Some("https://example.com".to_string()),
            beneficiary_first_name: Some("John".to_string()),
            beneficiary_last_name: Some("Doe".to_string()),
            beneficiary_company_name: None,
            beneficiary_address: "123 Main St, Anytown, USA".to_string(),
            agreed: true,
            personal: true,
            label: "Updated BTC Wallet".to_string(),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(json_value.get("type").unwrap(), "withdrawal");
        assert_eq!(
            json_value.get("address").unwrap(),
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"
        );
        assert_eq!(
            json_value.get("beneficiary_vasp_name").unwrap(),
            "Example VASP"
        );
        assert_eq!(
            json_value.get("beneficiary_vasp_did").unwrap(),
            "did:example:123456"
        );
        assert_eq!(
            json_value.get("beneficiary_vasp_website").unwrap(),
            "https://example.com"
        );
        assert_eq!(json_value.get("beneficiary_first_name").unwrap(), "John");
        assert_eq!(json_value.get("beneficiary_last_name").unwrap(), "Doe");
        assert_eq!(
            json_value.get("beneficiary_address").unwrap(),
            "123 Main St, Anytown, USA"
        );
        assert_eq!(json_value.get("agreed").unwrap(), true);
        assert_eq!(json_value.get("personal").unwrap(), true);
        assert_eq!(json_value.get("label").unwrap(), "Updated BTC Wallet");
        assert!(json_value.get("beneficiary_company_name").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_company_beneficiary() {
        let request = UpdateInAddressBookRequest {
            currency: Currency::ETH,
            address_type: AddressBookType::Transfer,
            address: "0x742d35Cc6634C0532925a3b8D05c4ae5e34D7b1c".to_string(),
            beneficiary_vasp_name: "Corporate VASP".to_string(),
            beneficiary_vasp_did: "did:example:789".to_string(),
            beneficiary_vasp_website: None,
            beneficiary_first_name: None,
            beneficiary_last_name: None,
            beneficiary_company_name: Some("ACME Corp".to_string()),
            beneficiary_address: "456 Business Ave, Corporate City".to_string(),
            agreed: false,
            personal: false,
            label: "Updated ETH Corporate Address".to_string(),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "ETH");
        assert_eq!(json_value.get("type").unwrap(), "transfer");
        assert_eq!(
            json_value.get("beneficiary_company_name").unwrap(),
            "ACME Corp"
        );
        assert_eq!(json_value.get("agreed").unwrap(), false);
        assert_eq!(json_value.get("personal").unwrap(), false);
        assert!(json_value.get("beneficiary_vasp_website").is_none());
        assert!(json_value.get("beneficiary_first_name").is_none());
        assert!(json_value.get("beneficiary_last_name").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_minimal() {
        let request = UpdateInAddressBookRequest {
            currency: Currency::USDC,
            address_type: AddressBookType::DepositSource,
            address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
            beneficiary_vasp_name: "Minimal VASP".to_string(),
            beneficiary_vasp_did: "did:example:minimal".to_string(),
            beneficiary_vasp_website: None,
            beneficiary_first_name: None,
            beneficiary_last_name: None,
            beneficiary_company_name: None,
            beneficiary_address: "Minimal Address".to_string(),
            agreed: true,
            personal: false,
            label: "Minimal Label".to_string(),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "USDC");
        assert_eq!(json_value.get("type").unwrap(), "deposit_source");
        assert_eq!(
            json_value.get("beneficiary_vasp_name").unwrap(),
            "Minimal VASP"
        );
        assert_eq!(
            json_value.get("beneficiary_vasp_did").unwrap(),
            "did:example:minimal"
        );
        assert_eq!(
            json_value.get("beneficiary_address").unwrap(),
            "Minimal Address"
        );
        assert_eq!(json_value.get("agreed").unwrap(), true);
        assert_eq!(json_value.get("personal").unwrap(), false);
        assert_eq!(json_value.get("label").unwrap(), "Minimal Label");
        assert!(json_value.get("beneficiary_vasp_website").is_none());
        assert!(json_value.get("beneficiary_first_name").is_none());
        assert!(json_value.get("beneficiary_last_name").is_none());
        assert!(json_value.get("beneficiary_company_name").is_none());
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": "ok"
        });

        let response: UpdateInAddressBookResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, "ok");
    }

    #[tokio::test]
    async fn test_update_in_address_book_method_exists() {
        // Test that the method exists and compiles without needing to call it
        let credentials = Credentials {
            api_key: SecretString::from("test_key".to_string()),
            api_secret: SecretString::from("test_secret".to_string()),
        };
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = crate::deribit::RateLimiter::new(AccountTier::Tier4);

        let rest_client = PrivateRestClient::new(
            credentials,
            "https://test.deribit.com",
            rate_limiter,
            http_client,
        );

        // Test that we can get a function reference to the method
        let _ = PrivateRestClient::update_in_address_book;

        // Verify the client exists
        let _ = &rest_client;

        println!("update_in_address_book method is accessible and properly typed");
    }
}
