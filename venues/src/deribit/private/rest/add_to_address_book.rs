use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{
    AddressBookType, AddressStatus, Currency, EndpointType, JsonRpcResult, RestResult,
};

/// REST API endpoint constant
const ADD_TO_ADDRESS_BOOK_ENDPOINT: &str = "private/add_to_address_book";

/// Request parameters for adding to address book
#[derive(Debug, Clone, Serialize)]
pub struct AddToAddressBookRequest {
    /// The currency symbol
    pub currency: Currency,

    /// Address book type
    #[serde(rename = "type")]
    pub address_type: AddressBookType,

    /// Address in currency format
    pub address: String,

    /// Label of the address book entry
    pub label: String,

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

    /// The user can pass a list of currencies to add the address for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_currencies: Option<Vec<Currency>>,
}

/// Address book entry data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressBookEntry {
    /// Address in proper format for currency
    pub address: String,

    /// Indicates that the user agreed to share provided information with 3rd parties
    pub agreed: bool,

    /// Geographical address of the beneficiary
    pub beneficiary_address: String,

    /// Beneficiary company name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_company_name: Option<String>,

    /// First name of the beneficiary (if beneficiary is a person)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_first_name: Option<String>,

    /// Last name of the beneficiary (if beneficiary is a person)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_last_name: Option<String>,

    /// DID of beneficiary VASP
    pub beneficiary_vasp_did: String,

    /// Name of beneficiary VASP
    pub beneficiary_vasp_name: String,

    /// Website of the beneficiary VASP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_vasp_website: Option<String>,

    /// The timestamp (milliseconds since the Unix epoch)
    pub creation_timestamp: i64,

    /// Currency, i.e "BTC", "ETH", "USDC"
    pub currency: String,

    /// Signalises that addition information regarding the beneficiary of the address is required
    pub info_required: bool,

    /// Label of the address book entry
    pub label: String,

    /// The user confirms that he provided address belongs to him and he has access to it via an un-hosted wallet software
    pub personal: bool,

    /// If address requires email confirmation for withdrawals
    pub requires_confirmation: bool,

    /// If email confirmation change is in progress
    pub requires_confirmation_change: bool,

    /// Wallet address status
    pub status: AddressStatus,

    /// Address book type
    #[serde(rename = "type")]
    pub address_type: AddressBookType,

    /// Timestamp when the address will be ready
    pub waiting_timestamp: Option<i64>,
}

/// Response for add to address book endpoint
pub type AddToAddressBookResponse = JsonRpcResult<AddressBookEntry>;

impl RestClient {
    /// Adds new entry to address book of given type
    ///
    /// Adds a new entry to the address book for the specified type and currency.
    /// The request requires wallet:read_write scope.
    ///
    /// [docs]: https://docs.deribit.com/v2/#private-add_to_address_book
    ///
    /// Rate limit: 500 credits per request (non-matching engine)
    /// Scope: wallet:read_write
    ///
    /// # Arguments
    /// * `currency` - The currency symbol
    /// * `address_type` - Address book type (transfer, withdrawal, deposit_source)
    /// * `address` - Address in currency format
    /// * `label` - Label of the address book entry
    /// * `beneficiary_vasp_name` - Name of beneficiary VASP
    /// * `beneficiary_vasp_did` - DID of beneficiary VASP
    /// * `beneficiary_vasp_website` - Website of the beneficiary VASP (optional)
    /// * `beneficiary_first_name` - First name of beneficiary (optional)
    /// * `beneficiary_last_name` - Last name of beneficiary (optional)
    /// * `beneficiary_company_name` - Beneficiary company name (optional)
    /// * `beneficiary_address` - Geographical address of the beneficiary
    /// * `agreed` - User agreed to share information with 3rd parties
    /// * `personal` - User confirms address belongs to them
    /// * `extra_currencies` - List of additional currencies (optional)
    ///
    /// # Returns
    /// Address book entry result with complete entry information
    pub async fn add_to_address_book(
        &self,
        request: AddToAddressBookRequest,
    ) -> RestResult<AddToAddressBookResponse> {
        self.send_signed_request(
            ADD_TO_ADDRESS_BOOK_ENDPOINT,
            &request,
            EndpointType::MatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use rest::secrets::ExposableSecret;
    use serde_json::{Value, json};

    use super::*;
    use crate::deribit::AccountTier;

    // Test secret implementation
    #[derive(Clone)]
    struct PlainTextSecret {
        secret: String,
    }

    impl PlainTextSecret {
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    #[test]
    fn test_request_parameters_serialization() {
        let request = AddToAddressBookRequest {
            currency: Currency::BTC,
            address_type: AddressBookType::Withdrawal,
            address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            label: "My BTC Wallet".to_string(),
            beneficiary_vasp_name: "Example VASP".to_string(),
            beneficiary_vasp_did: "did:example:123456".to_string(),
            beneficiary_vasp_website: Some("https://example.com".to_string()),
            beneficiary_first_name: Some("John".to_string()),
            beneficiary_last_name: Some("Doe".to_string()),
            beneficiary_company_name: None,
            beneficiary_address: "123 Main St, Anytown, USA".to_string(),
            agreed: true,
            personal: true,
            extra_currencies: Some(vec![Currency::ETH, Currency::USDC]),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(json_value.get("type").unwrap(), "withdrawal");
        assert_eq!(
            json_value.get("address").unwrap(),
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"
        );
        assert_eq!(json_value.get("label").unwrap(), "My BTC Wallet");
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
        assert!(json_value.get("beneficiary_company_name").is_none());

        let extra_currencies = json_value
            .get("extra_currencies")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(extra_currencies.len(), 2);
        assert_eq!(extra_currencies[0], "ETH");
        assert_eq!(extra_currencies[1], "USDC");
    }

    #[test]
    fn test_request_parameters_serialization_minimal() {
        let request = AddToAddressBookRequest {
            currency: Currency::ETH,
            address_type: AddressBookType::Transfer,
            address: "0x742d35Cc6634C0532925a3b8D05c4ae5e34D7b1c".to_string(),
            label: "ETH Transfer Address".to_string(),
            beneficiary_vasp_name: "Another VASP".to_string(),
            beneficiary_vasp_did: "did:example:789".to_string(),
            beneficiary_vasp_website: None,
            beneficiary_first_name: None,
            beneficiary_last_name: None,
            beneficiary_company_name: Some("ACME Corp".to_string()),
            beneficiary_address: "456 Business Ave, Corporate City".to_string(),
            agreed: true,
            personal: false,
            extra_currencies: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "ETH");
        assert_eq!(json_value.get("type").unwrap(), "transfer");
        assert_eq!(
            json_value.get("beneficiary_company_name").unwrap(),
            "ACME Corp"
        );
        assert_eq!(json_value.get("personal").unwrap(), false);
        assert!(json_value.get("beneficiary_vasp_website").is_none());
        assert!(json_value.get("beneficiary_first_name").is_none());
        assert!(json_value.get("beneficiary_last_name").is_none());
        assert!(json_value.get("extra_currencies").is_none());
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "address": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
                "agreed": true,
                "beneficiary_address": "123 Main St, Anytown, USA",
                "beneficiary_company_name": null,
                "beneficiary_first_name": "John",
                "beneficiary_last_name": "Doe",
                "beneficiary_vasp_did": "did:example:123456",
                "beneficiary_vasp_name": "Example VASP",
                "beneficiary_vasp_website": "https://example.com",
                "creation_timestamp": 1640995200000i64,
                "currency": "BTC",
                "info_required": false,
                "label": "My BTC Wallet",
                "personal": true,
                "requires_confirmation": false,
                "requires_confirmation_change": false,
                "status": "confirmed",
                "type": "withdrawal",
                "waiting_timestamp": null
            }
        });

        let response: AddToAddressBookResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(
            response.result.address,
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"
        );
        assert!(response.result.agreed);
        assert_eq!(
            response.result.beneficiary_address,
            "123 Main St, Anytown, USA"
        );
        assert_eq!(
            response.result.beneficiary_first_name,
            Some("John".to_string())
        );
        assert_eq!(
            response.result.beneficiary_last_name,
            Some("Doe".to_string())
        );
        assert_eq!(response.result.beneficiary_company_name, None);
        assert_eq!(response.result.beneficiary_vasp_did, "did:example:123456");
        assert_eq!(response.result.beneficiary_vasp_name, "Example VASP");
        assert_eq!(
            response.result.beneficiary_vasp_website,
            Some("https://example.com".to_string())
        );
        assert_eq!(response.result.currency, "BTC");
        assert_eq!(response.result.label, "My BTC Wallet");
        assert!(response.result.personal);
        assert_eq!(response.result.status, AddressStatus::Confirmed);
        assert_eq!(response.result.address_type, AddressBookType::Withdrawal);
        assert_eq!(response.result.waiting_timestamp, None);
    }

    #[test]
    fn test_response_with_company_beneficiary() {
        let response_json = json!({
            "id": 2,
            "jsonrpc": "2.0",
            "result": {
                "address": "0x742d35Cc6634C0532925a3b8D05c4ae5e34D7b1c",
                "agreed": true,
                "beneficiary_address": "456 Business Ave, Corporate City",
                "beneficiary_company_name": "ACME Corp",
                "beneficiary_first_name": null,
                "beneficiary_last_name": null,
                "beneficiary_vasp_did": "did:example:789",
                "beneficiary_vasp_name": "Another VASP",
                "beneficiary_vasp_website": null,
                "creation_timestamp": 1640995300000i64,
                "currency": "ETH",
                "info_required": true,
                "label": "ETH Transfer Address",
                "personal": false,
                "requires_confirmation": true,
                "requires_confirmation_change": false,
                "status": "waiting",
                "type": "transfer",
                "waiting_timestamp": 1640995400000i64
            }
        });

        let response: AddToAddressBookResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.result.currency, "ETH");
        assert_eq!(
            response.result.beneficiary_company_name,
            Some("ACME Corp".to_string())
        );
        assert_eq!(response.result.beneficiary_first_name, None);
        assert_eq!(response.result.beneficiary_last_name, None);
        assert_eq!(response.result.beneficiary_vasp_website, None);
        assert!(!response.result.personal);
        assert!(response.result.info_required);
        assert!(response.result.requires_confirmation);
        assert_eq!(response.result.status, AddressStatus::Waiting);
        assert_eq!(response.result.address_type, AddressBookType::Transfer);
        assert_eq!(response.result.waiting_timestamp, Some(1640995400000i64));
    }

    #[tokio::test]
    async fn test_add_to_address_book_method_exists() {
        // Test that the method exists and compiles without needing to call it
        let api_key =
            Box::new(PlainTextSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret =
            Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = crate::deribit::RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://test.deribit.com",
            rate_limiter,
            http_client,
        );

        // Test that we can get a function reference to the method
        let _ = RestClient::add_to_address_book;

        // Verify the client exists
        let _ = &rest_client;

        println!("add_to_address_book method is accessible and properly typed");
    }
}
