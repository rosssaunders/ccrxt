use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::deribit::{AddressBookType, Currency, EndpointType, RestResult};

/// Request parameters for remove from address book
#[derive(Debug, Clone, Serialize)]
pub struct RemoveFromAddressBookRequest {
    /// The currency symbol
    pub currency: Currency,
    /// Address book type
    #[serde(rename = "type")]
    pub address_type: AddressBookType,
    /// Address in currency format, it must be in address book
    pub address: String,
}

/// Response for remove from address book endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveFromAddressBookResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Result of method execution. "ok" in case of success
    pub result: String,
}

impl RestClient {
    /// Removes address book entry
    ///
    /// This endpoint requires wallet:read_write scope and removes the specified
    /// address from the address book of the given type and currency.
    ///
    /// See: <https://docs.deribit.com/v2/#private-remove_from_address_book>
    ///
    /// Rate limit: 500 credits per request (non-matching engine)
    /// Scope: wallet:read_write
    ///
    /// # Arguments
    /// * `request` - Request parameters containing currency, address type, and address
    ///
    /// # Returns
    /// Result containing "ok" string on success
    pub async fn remove_from_address_book(
        &self,
        request: RemoveFromAddressBookRequest,
    ) -> RestResult<RemoveFromAddressBookResponse> {
        self.send_signed_request(
            "private/remove_from_address_book",
            &request,
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
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
    fn test_request_parameters_serialization_btc_withdrawal() {
        let request = RemoveFromAddressBookRequest {
            currency: Currency::BTC,
            address_type: AddressBookType::Withdrawal,
            address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(json_value.get("type").unwrap(), "withdrawal");
        assert_eq!(
            json_value.get("address").unwrap(),
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"
        );
    }

    #[test]
    fn test_request_parameters_serialization_eth_transfer() {
        let request = RemoveFromAddressBookRequest {
            currency: Currency::ETH,
            address_type: AddressBookType::Transfer,
            address: "0x742d35Cc6634C0532925a3b8D05c4ae5e34D7b1c".to_string(),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "ETH");
        assert_eq!(json_value.get("type").unwrap(), "transfer");
        assert_eq!(
            json_value.get("address").unwrap(),
            "0x742d35Cc6634C0532925a3b8D05c4ae5e34D7b1c"
        );
    }

    #[test]
    fn test_request_parameters_serialization_usdc_deposit_source() {
        let request = RemoveFromAddressBookRequest {
            currency: Currency::USDC,
            address_type: AddressBookType::DepositSource,
            address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "USDC");
        assert_eq!(json_value.get("type").unwrap(), "deposit_source");
        assert_eq!(
            json_value.get("address").unwrap(),
            "0x1234567890abcdef1234567890abcdef12345678"
        );
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": "ok"
        });

        let response: RemoveFromAddressBookResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, "ok");
    }

    #[tokio::test]
    async fn test_remove_from_address_book_method_exists() {
        // Test that the method exists and compiles without needing to call it
        let api_key = Box::new(PlainTextSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret = Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
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
        let _ = RestClient::remove_from_address_book;

        // Verify the client exists
        let _ = &rest_client;

        println!("remove_from_address_book method is accessible and properly typed");
    }

    #[test]
    fn test_all_currency_and_type_combinations() {
        let test_cases = vec![
            (Currency::BTC, AddressBookType::Withdrawal, "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"),
            (Currency::ETH, AddressBookType::Transfer, "0x742d35Cc6634C0532925a3b8D05c4ae5e34D7b1c"),
            (Currency::USDC, AddressBookType::DepositSource, "0x1234567890abcdef1234567890abcdef12345678"),
        ];

        for (currency, address_type, address) in test_cases {
            let request = RemoveFromAddressBookRequest {
                currency: currency.clone(),
                address_type: address_type.clone(),
                address: address.to_string(),
            };

            let json_str = serde_json::to_string(&request).unwrap();
            let json_value: Value = serde_json::from_str(&json_str).unwrap();

            // Verify all fields are serialized correctly
            assert!(json_value.get("currency").is_some());
            assert!(json_value.get("type").is_some());
            assert_eq!(json_value.get("address").unwrap(), address);
        }

        println!("All currency and address type combinations work correctly");
    }
}