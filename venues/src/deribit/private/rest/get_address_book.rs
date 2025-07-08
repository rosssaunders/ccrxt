use serde::Serialize;

use super::RestClient;
pub use super::add_to_address_book::AddressBookEntry;
use crate::deribit::{AddressBookType, Currency, EndpointType, JsonRpcResult, RestResult};

/// REST API endpoint constant
const GET_ADDRESS_BOOK_ENDPOINT: &str = "private/get_address_book";

/// Request parameters for get address book
#[derive(Debug, Clone, Serialize)]
pub struct GetAddressBookRequest {
    /// The currency symbol
    pub currency: Currency,
    /// Address book type
    #[serde(rename = "type")]
    pub address_type: AddressBookType,
}

/// Response for get address book endpoint
pub type GetAddressBookResponse = JsonRpcResult<Vec<AddressBookEntry>>;

impl RestClient {
    /// Retrieves address book of given type
    ///
    /// This endpoint requires wallet:read scope and returns all address book entries
    /// for the specified currency and address type.
    ///
    /// See: <https://docs.deribit.com/v2/#private-get_address_book>
    ///
    /// Rate limit: 500 credits per request (non-matching engine)
    /// Scope: wallet:read
    ///
    /// # Arguments
    /// * `currency` - The currency symbol
    /// * `address_type` - Address book type (transfer, withdrawal, or deposit_source)
    ///
    /// # Returns
    /// Result containing array of address book entries
    pub async fn get_address_book(
        &self,
        request: GetAddressBookRequest,
    ) -> RestResult<GetAddressBookResponse> {
        self.send_signed_request(
            GET_ADDRESS_BOOK_ENDPOINT,
            &request,
            EndpointType::MatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use rest::secrets::ExposableSecret;
    /// REST API endpoint constant
    use serde_json::{Value, json};

    use super::*;
    use crate::deribit::{AccountTier, AddressStatus};

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
        let request = GetAddressBookRequest {
            currency: Currency::BTC,
            address_type: AddressBookType::Withdrawal,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(json_value.get("type").unwrap(), "withdrawal");
    }

    #[test]
    fn test_request_parameters_serialization_transfer() {
        let request = GetAddressBookRequest {
            currency: Currency::ETH,
            address_type: AddressBookType::Transfer,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "ETH");
        assert_eq!(json_value.get("type").unwrap(), "transfer");
    }

    #[test]
    fn test_request_parameters_serialization_deposit_source() {
        let request = GetAddressBookRequest {
            currency: Currency::USDC,
            address_type: AddressBookType::DepositSource,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "USDC");
        assert_eq!(json_value.get("type").unwrap(), "deposit_source");
    }

    #[test]
    fn test_response_structures_deserialization_empty() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": []
        });

        let response: GetAddressBookResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.result.is_empty());
    }

    #[test]
    fn test_response_structures_deserialization_with_entries() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": [
                {
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
                },
                {
                    "address": "0x742d35Cc6634C0532925a3b8D05c4ae5e34D7b1c",
                    "agreed": false,
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
            ]
        });

        let response: GetAddressBookResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.len(), 2);

        // Check first entry
        let first_entry = &response.result[0];
        assert_eq!(
            first_entry.address,
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"
        );
        assert_eq!(first_entry.currency, "BTC");
        assert_eq!(first_entry.label, "My BTC Wallet");
        assert_eq!(first_entry.status, AddressStatus::Confirmed);
        assert_eq!(first_entry.address_type, AddressBookType::Withdrawal);
        assert!(first_entry.agreed);
        assert!(first_entry.personal);

        // Check second entry
        let second_entry = &response.result[1];
        assert_eq!(
            second_entry.address,
            "0x742d35Cc6634C0532925a3b8D05c4ae5e34D7b1c"
        );
        assert_eq!(second_entry.currency, "ETH");
        assert_eq!(second_entry.label, "ETH Transfer Address");
        assert_eq!(second_entry.status, AddressStatus::Waiting);
        assert_eq!(second_entry.address_type, AddressBookType::Transfer);
        assert!(!second_entry.agreed);
        assert!(!second_entry.personal);
        assert_eq!(
            second_entry.beneficiary_company_name,
            Some("ACME Corp".to_string())
        );
    }

    #[tokio::test]
    async fn test_get_address_book_method_exists() {
        // Test that the method exists and compiles without needing to call it
        let api_key =
            Box::new(PlainTextSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret =
            Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let client = reqwest::Client::new();
        let rate_limiter = crate::deribit::RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://test.deribit.com",
            rate_limiter,
            client,
        );

        // Test that we can get a function reference to the method
        let _ = RestClient::get_address_book;

        // Verify the client exists
        let _ = &rest_client;

        println!("get_address_book method is accessible and properly typed");
    }

    #[test]
    fn test_all_currency_and_type_combinations() {
        let currencies = vec![Currency::BTC, Currency::ETH, Currency::USDC];
        let types = vec![
            AddressBookType::Transfer,
            AddressBookType::Withdrawal,
            AddressBookType::DepositSource,
        ];

        for currency in currencies {
            for address_type in &types {
                let request = GetAddressBookRequest {
                    currency: currency.clone(),
                    address_type: address_type.clone(),
                };

                let json_str = serde_json::to_string(&request).unwrap();
                let json_value: Value = serde_json::from_str(&json_str).unwrap();

                // Verify currency serialization
                match currency {
                    Currency::BTC => assert_eq!(json_value.get("currency").unwrap(), "BTC"),
                    Currency::ETH => assert_eq!(json_value.get("currency").unwrap(), "ETH"),
                    Currency::USDC => assert_eq!(json_value.get("currency").unwrap(), "USDC"),
                    _ => {}
                }

                // Verify type serialization
                match address_type {
                    AddressBookType::Transfer => {
                        assert_eq!(json_value.get("type").unwrap(), "transfer")
                    }
                    AddressBookType::Withdrawal => {
                        assert_eq!(json_value.get("type").unwrap(), "withdrawal")
                    }
                    AddressBookType::DepositSource => {
                        assert_eq!(json_value.get("type").unwrap(), "deposit_source")
                    }
                }
            }
        }

        println!("All currency and address type combinations work correctly");
    }
}
