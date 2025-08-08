use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{Currency, EndpointType, JsonRpcResult, RestResult};

/// REST API endpoint constant
const SUBMIT_TRANSFER_TO_SUBACCOUNT_ENDPOINT: &str = "private/submit_transfer_to_subaccount";

/// Request parameters for submit transfer to subaccount
#[derive(Debug, Clone, Serialize)]
pub struct SubmitTransferToSubaccountRequest {
    /// The currency symbol
    pub currency: Currency,

    /// Amount of funds to be transferred
    pub amount: f64,

    /// Id of destination subaccount
    pub destination: i64,
}

/// Transfer data for submit transfer to subaccount result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubaccountTransferData {
    /// Amount of funds in given currency
    pub amount: f64,

    /// The timestamp (milliseconds since the Unix epoch)
    pub created_timestamp: i64,

    /// Currency, i.e "BTC", "ETH", "USDC"
    pub currency: String,

    /// Transfer direction
    pub direction: String,

    /// Id of transfer
    pub id: i64,

    /// For transfer to subaccount returns this subaccount name
    pub other_side: String,

    /// Transfer state
    pub state: String,

    /// Type of transfer: "user" - sent to user, "subaccount" - sent to subaccount
    #[serde(rename = "type")]
    pub transfer_type: String,

    /// The timestamp (milliseconds since the Unix epoch)
    pub updated_timestamp: i64,
}

/// Response for submit transfer to subaccount endpoint
pub type SubmitTransferToSubaccountResponse = JsonRpcResult<SubaccountTransferData>;

impl RestClient {
    /// Transfer funds to subaccount
    ///
    /// This endpoint requires wallets:read_write scope and transfers funds
    /// to the specified subaccount.
    ///
    /// [docs]: https://docs.deribit.com/v2/#private-submit_transfer_to_subaccount
    ///
    /// Rate limit: 500 credits per request (non-matching engine)
    /// Scope: wallets:read_write
    ///
    /// # Arguments
    /// * `request` - Request parameters containing currency, amount, and destination subaccount
    ///
    /// # Returns
    /// Result containing transfer data
    pub async fn submit_transfer_to_subaccount(
        &self,
        request: SubmitTransferToSubaccountRequest,
    ) -> RestResult<SubmitTransferToSubaccountResponse> {
        self.send_signed_request(
            SUBMIT_TRANSFER_TO_SUBACCOUNT_ENDPOINT,
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
    fn test_request_parameters_serialization() {
        let request = SubmitTransferToSubaccountRequest {
            currency: Currency::BTC,
            amount: 0.1,
            destination: 12345,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(json_value.get("amount").unwrap(), 0.1);
        assert_eq!(json_value.get("destination").unwrap(), 12345);
    }

    #[test]
    fn test_request_parameters_serialization_eth() {
        let request = SubmitTransferToSubaccountRequest {
            currency: Currency::ETH,
            amount: 1.5,
            destination: 67890,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "ETH");
        assert_eq!(json_value.get("amount").unwrap(), 1.5);
        assert_eq!(json_value.get("destination").unwrap(), 67890);
    }

    #[test]
    fn test_request_parameters_serialization_usdc() {
        let request = SubmitTransferToSubaccountRequest {
            currency: Currency::USDC,
            amount: 100.0,
            destination: 54321,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "USDC");
        assert_eq!(json_value.get("amount").unwrap(), 100.0);
        assert_eq!(json_value.get("destination").unwrap(), 54321);
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "amount": 0.1,
                "created_timestamp": 1640995200000i64,
                "currency": "BTC",
                "direction": "payment",
                "id": 9876543,
                "other_side": "subaccount_01",
                "state": "confirmed",
                "type": "subaccount",
                "updated_timestamp": 1640995300000i64
            }
        });

        let response: SubmitTransferToSubaccountResponse =
            serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.amount, 0.1);
        assert_eq!(response.result.created_timestamp, 1640995200000i64);
        assert_eq!(response.result.currency, "BTC");
        assert_eq!(response.result.direction, "payment");
        assert_eq!(response.result.id, 9876543);
        assert_eq!(response.result.other_side, "subaccount_01");
        assert_eq!(response.result.state, "confirmed");
        assert_eq!(response.result.transfer_type, "subaccount");
        assert_eq!(response.result.updated_timestamp, 1640995300000i64);
    }

    #[test]
    fn test_response_structures_deserialization_prepared_state() {
        let response_json = json!({
            "id": 2,
            "jsonrpc": "2.0",
            "result": {
                "amount": 1.5,
                "created_timestamp": 1640995400000i64,
                "currency": "ETH",
                "direction": "payment",
                "id": 9876544,
                "other_side": "trading_subaccount",
                "state": "prepared",
                "type": "subaccount",
                "updated_timestamp": 1640995400000i64
            }
        });

        let response: SubmitTransferToSubaccountResponse =
            serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 2);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.amount, 1.5);
        assert_eq!(response.result.currency, "ETH");
        assert_eq!(response.result.other_side, "trading_subaccount");
        assert_eq!(response.result.state, "prepared");
        assert_eq!(response.result.transfer_type, "subaccount");
    }

    #[test]
    fn test_subaccount_transfer_data_serialization() {
        let transfer_data = SubaccountTransferData {
            amount: 0.1,
            created_timestamp: 1640995200000i64,
            currency: "BTC".to_string(),
            direction: "payment".to_string(),
            id: 9876543,
            other_side: "subaccount_01".to_string(),
            state: "confirmed".to_string(),
            transfer_type: "subaccount".to_string(),
            updated_timestamp: 1640995300000i64,
        };

        let json_str = serde_json::to_string(&transfer_data).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("amount").unwrap(), 0.1);
        assert_eq!(
            json_value.get("created_timestamp").unwrap(),
            1640995200000i64
        );
        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(json_value.get("direction").unwrap(), "payment");
        assert_eq!(json_value.get("id").unwrap(), 9876543);
        assert_eq!(json_value.get("other_side").unwrap(), "subaccount_01");
        assert_eq!(json_value.get("state").unwrap(), "confirmed");
        assert_eq!(json_value.get("type").unwrap(), "subaccount");
        assert_eq!(
            json_value.get("updated_timestamp").unwrap(),
            1640995300000i64
        );
    }

    #[tokio::test]
    async fn test_submit_transfer_to_subaccount_method_exists() {
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
        let _ = RestClient::submit_transfer_to_subaccount;

        // Verify the client exists
        let _ = &rest_client;

        println!("submit_transfer_to_subaccount method is accessible and properly typed");
    }

    #[test]
    fn test_all_supported_currencies() {
        let test_cases = vec![
            (Currency::BTC, 0.1, 12345),
            (Currency::ETH, 1.5, 67890),
            (Currency::USDC, 100.0, 54321),
        ];

        for (currency, amount, destination) in test_cases {
            let request = SubmitTransferToSubaccountRequest {
                currency: currency.clone(),
                amount,
                destination,
            };

            let json_str = serde_json::to_string(&request).unwrap();
            let json_value: Value = serde_json::from_str(&json_str).unwrap();

            // Verify all fields are serialized correctly
            assert!(json_value.get("currency").is_some());
            assert_eq!(json_value.get("amount").unwrap(), amount);
            assert_eq!(json_value.get("destination").unwrap(), destination);
        }

        println!("All supported currencies work correctly");
    }
}
