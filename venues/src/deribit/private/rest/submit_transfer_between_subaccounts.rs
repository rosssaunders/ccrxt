use serde::{Deserialize, Serialize};

use super::RestClient;
// Reuse the TransferData struct from submit_transfer_to_user
use super::submit_transfer_to_user::TransferData;
use crate::deribit::{EndpointType, RestResult};

/// Request parameters for submit transfer between subaccounts
#[derive(Debug, Clone, Serialize)]
pub struct SubmitTransferBetweenSubaccountsRequest {
    /// The currency symbol (BTC, ETH, USDC, USDT, EURR)
    pub currency: String,
    /// Amount of funds to be transferred
    pub amount: f64,
    /// Id of destination subaccount
    pub destination: i32,
    /// Id of the source (sub)account (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<i32>,
}

/// Response for submit transfer between subaccounts endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitTransferBetweenSubaccountsResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Transfer result data
    pub result: TransferData,
}

impl RestClient {
    /// Transfer funds between two (sub)accounts
    ///
    /// Transfer funds between two (sub)accounts.
    ///
    /// See: <https://docs.deribit.com/v2/#private-submit_transfer_between_subaccounts>
    ///
    /// Rate limit: 500 credits per request (non-matching engine)
    /// Scope: wallets:read_write
    ///
    /// # Arguments
    /// * `params` - Parameters for the transfer (currency, amount, destination, optional source)
    ///
    /// # Returns
    /// Transfer result with complete transfer information
    pub async fn submit_transfer_between_subaccounts(
        &self,
        params: SubmitTransferBetweenSubaccountsRequest,
    ) -> RestResult<SubmitTransferBetweenSubaccountsResponse> {
        self.send_signed_request(
            "private/submit_transfer_between_subaccounts",
            &params,
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
    fn test_request_parameters_serialization() {
        let request = SubmitTransferBetweenSubaccountsRequest {
            currency: "BTC".to_string(),
            amount: 0.001,
            destination: 12345,
            source: Some(67890),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(json_value.get("amount").unwrap(), 0.001);
        assert_eq!(json_value.get("destination").unwrap(), 12345);
        assert_eq!(json_value.get("source").unwrap(), 67890);
    }

    #[test]
    fn test_request_parameters_serialization_without_source() {
        let request = SubmitTransferBetweenSubaccountsRequest {
            currency: "ETH".to_string(),
            amount: 0.5,
            destination: 54321,
            source: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "ETH");
        assert_eq!(json_value.get("amount").unwrap(), 0.5);
        assert_eq!(json_value.get("destination").unwrap(), 54321);
        assert!(json_value.get("source").is_none()); // Should be omitted when None
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "amount": 0.001,
                "created_timestamp": 1640995200000i64,
                "currency": "BTC",
                "direction": "payment",
                "id": 12345,
                "other_side": "subaccount_name_123",
                "state": "confirmed",
                "type": "subaccount",
                "updated_timestamp": 1640995210000i64
            }
        });

        let response: SubmitTransferBetweenSubaccountsResponse =
            serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.amount, 0.001);
        assert_eq!(response.result.currency, "BTC");
        assert_eq!(response.result.direction, "payment");
        assert_eq!(response.result.state, "confirmed");
        assert_eq!(response.result.transfer_type, "subaccount");
        assert_eq!(response.result.other_side, "subaccount_name_123");
    }

    #[tokio::test]
    async fn test_submit_transfer_between_subaccounts_method_exists() {
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
        let _ = RestClient::submit_transfer_between_subaccounts;

        // Verify the client exists
        let _ = &rest_client;

        println!("submit_transfer_between_subaccounts method is accessible and properly typed");
    }

    #[test]
    fn test_all_supported_currencies() {
        // Test that all supported currencies work in serialization
        let currencies = ["BTC", "ETH", "USDC", "USDT", "EURR"];

        for currency in currencies {
            let request = SubmitTransferBetweenSubaccountsRequest {
                currency: currency.to_string(),
                amount: 0.001,
                destination: 12345,
                source: None,
            };

            let json_str = serde_json::to_string(&request).unwrap();
            let json_value: Value = serde_json::from_str(&json_str).unwrap();

            assert_eq!(json_value.get("currency").unwrap(), currency);
            println!("Currency {} serializes correctly", currency);
        }
    }
}
