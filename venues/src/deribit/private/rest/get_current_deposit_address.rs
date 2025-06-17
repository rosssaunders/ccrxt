use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::deribit::{Currency, EndpointType, RestResult};

/// Request parameters for get current deposit address
#[derive(Debug, Clone, Serialize)]
pub struct GetCurrentDepositAddressRequest {
    /// The currency symbol
    pub currency: Currency,
}

/// Deposit address information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositAddress {
    /// Address in proper format for currency
    pub address: String,
    /// The timestamp (milliseconds since the Unix epoch)
    pub creation_timestamp: i64,
    /// Currency, i.e "BTC", "ETH", "USDC"
    pub currency: String,
    /// Address type/purpose
    #[serde(rename = "type")]
    pub address_type: String,
}

/// Response for get current deposit address endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCurrentDepositAddressResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Object if address is created, null otherwise
    pub result: Option<DepositAddress>,
}

impl RestClient {
    /// Retrieve deposit address for currency
    ///
    /// This endpoint requires wallet:read scope and returns the current deposit
    /// address for the specified currency, or null if none exists.
    ///
    /// See: <https://docs.deribit.com/v2/#private-get_current_deposit_address>
    ///
    /// Rate limit: 500 credits per request (non-matching engine)
    /// Scope: wallet:read
    ///
    /// # Arguments
    /// * `currency` - The currency symbol
    ///
    /// # Returns
    /// Result containing optional deposit address information
    pub async fn get_current_deposit_address(
        &self,
        currency: Currency,
    ) -> RestResult<GetCurrentDepositAddressResponse> {
        let request = GetCurrentDepositAddressRequest { currency };
        self.send_signed_request(
            "private/get_current_deposit_address",
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
    fn test_request_parameters_serialization_btc() {
        let request = GetCurrentDepositAddressRequest {
            currency: Currency::BTC,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "BTC");
    }

    #[test]
    fn test_request_parameters_serialization_eth() {
        let request = GetCurrentDepositAddressRequest {
            currency: Currency::ETH,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "ETH");
    }

    #[test]
    fn test_request_parameters_serialization_usdc() {
        let request = GetCurrentDepositAddressRequest {
            currency: Currency::USDC,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "USDC");
    }

    #[test]
    fn test_response_structures_deserialization_with_address() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "address": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
                "creation_timestamp": 1640995200000i64,
                "currency": "BTC",
                "type": "deposit"
            }
        });

        let response: GetCurrentDepositAddressResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.result.is_some());

        let deposit_address = response.result.unwrap();
        assert_eq!(deposit_address.address, "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh");
        assert_eq!(deposit_address.creation_timestamp, 1640995200000i64);
        assert_eq!(deposit_address.currency, "BTC");
        assert_eq!(deposit_address.address_type, "deposit");
    }

    #[test]
    fn test_response_structures_deserialization_null_result() {
        let response_json = json!({
            "id": 2,
            "jsonrpc": "2.0",
            "result": null
        });

        let response: GetCurrentDepositAddressResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 2);
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.result.is_none());
    }

    #[test]
    fn test_response_structures_deserialization_eth_address() {
        let response_json = json!({
            "id": 3,
            "jsonrpc": "2.0",
            "result": {
                "address": "0x742d35Cc6634C0532925a3b8D05c4ae5e34D7b1c",
                "creation_timestamp": 1640995300000i64,
                "currency": "ETH",
                "type": "deposit"
            }
        });

        let response: GetCurrentDepositAddressResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 3);
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.result.is_some());

        let deposit_address = response.result.unwrap();
        assert_eq!(deposit_address.address, "0x742d35Cc6634C0532925a3b8D05c4ae5e34D7b1c");
        assert_eq!(deposit_address.creation_timestamp, 1640995300000i64);
        assert_eq!(deposit_address.currency, "ETH");
        assert_eq!(deposit_address.address_type, "deposit");
    }

    #[test]
    fn test_deposit_address_serialization() {
        let deposit_address = DepositAddress {
            address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            creation_timestamp: 1640995200000i64,
            currency: "BTC".to_string(),
            address_type: "deposit".to_string(),
        };

        let json_str = serde_json::to_string(&deposit_address).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("address").unwrap(), "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh");
        assert_eq!(json_value.get("creation_timestamp").unwrap(), 1640995200000i64);
        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(json_value.get("type").unwrap(), "deposit");
    }

    #[tokio::test]
    async fn test_get_current_deposit_address_method_exists() {
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
        let _ = RestClient::get_current_deposit_address;

        // Verify the client exists
        let _ = &rest_client;

        println!("get_current_deposit_address method is accessible and properly typed");
    }

    #[test]
    fn test_all_supported_currencies() {
        let currencies = vec![Currency::BTC, Currency::ETH, Currency::USDC];

        for currency in currencies {
            let request = GetCurrentDepositAddressRequest {
                currency: currency.clone(),
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
        }

        println!("All supported currencies work correctly");
    }
}