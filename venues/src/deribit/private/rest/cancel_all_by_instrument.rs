use serde::Serialize;

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, OrderType, RestResult};

/// REST API endpoint constant
const CANCEL_ALL_BY_INSTRUMENT_ENDPOINT: &str = "private/cancel_all_by_instrument";

/// Request parameters for cancel all by instrument endpoint
#[derive(Debug, Clone, Serialize)]
pub struct CancelAllByInstrumentRequest {
    /// Instrument name (required)
    pub instrument_name: String,
    /// Order type filter (optional)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<OrderType>,
    /// When detailed is set to true output format is changed (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed: Option<bool>,
    /// When set to true orders in combo instruments affecting a given position will also be cancelled (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_combos: Option<bool>,
    /// Whether or not to reject incoming quotes for 1 second after cancelling (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freeze_quotes: Option<bool>,
}

/// Response for cancel all by instrument endpoint
pub type CancelAllByInstrumentResponse = JsonRpcResult<i64>;

impl RestClient {
    /// Cancel all orders by instrument, optionally filtered by order type
    ///
    /// This method cancels all orders by instrument, optionally filtered by order type.
    /// This endpoint requires trade:read_write scope.
    ///
    /// See: <https://docs.deribit.com/v2/#private-cancel_all_by_instrument>
    ///
    /// Rate limit: Matching engine endpoint (tier-based rate limiting)
    /// Scope: trade:read_write
    ///
    /// # Arguments
    /// * `instrument_name` - The instrument name
    /// * `order_type` - Optional order type filter
    /// * `detailed` - Optional flag for detailed output format
    /// * `include_combos` - Optional flag to include combo instruments
    /// * `freeze_quotes` - Optional flag to reject incoming quotes for 1 second after cancelling
    ///
    /// # Returns
    /// Result with total number of successfully cancelled orders
    pub async fn cancel_all_by_instrument(
        &self,
        request: CancelAllByInstrumentRequest,
    ) -> RestResult<CancelAllByInstrumentResponse> {
        self.send_signed_request(
            CANCEL_ALL_BY_INSTRUMENT_ENDPOINT,
            &request,
            EndpointType::MatchingEngine,
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
    fn test_request_parameters_serialization_minimal() {
        let request = CancelAllByInstrumentRequest {
            instrument_name: "BTC-28JUN24-65000-C".to_string(),
            order_type: None,
            detailed: None,
            include_combos: None,
            freeze_quotes: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        // Should contain required instrument_name field
        assert_eq!(
            json_value.get("instrument_name").unwrap(),
            "BTC-28JUN24-65000-C"
        );
        // Should not contain optional fields when None
        assert!(json_value.get("type").is_none());
        assert!(json_value.get("detailed").is_none());
        assert!(json_value.get("include_combos").is_none());
        assert!(json_value.get("freeze_quotes").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_with_order_type() {
        let request = CancelAllByInstrumentRequest {
            instrument_name: "ETH-PERPETUAL".to_string(),
            order_type: Some(OrderType::Limit),
            detailed: None,
            include_combos: None,
            freeze_quotes: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("instrument_name").unwrap(), "ETH-PERPETUAL");
        assert_eq!(json_value.get("type").unwrap(), "limit");
        assert!(json_value.get("detailed").is_none());
        assert!(json_value.get("include_combos").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_with_include_combos() {
        let request = CancelAllByInstrumentRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
            order_type: None,
            detailed: None,
            include_combos: Some(true),
            freeze_quotes: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("instrument_name").unwrap(), "BTC-PERPETUAL");
        assert_eq!(json_value.get("include_combos").unwrap(), true);
        assert!(json_value.get("type").is_none());
        assert!(json_value.get("detailed").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_full() {
        let request = CancelAllByInstrumentRequest {
            instrument_name: "SOL-29MAR24-180-P".to_string(),
            order_type: Some(OrderType::Stop),
            detailed: Some(true),
            include_combos: Some(false),
            freeze_quotes: Some(true),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(
            json_value.get("instrument_name").unwrap(),
            "SOL-29MAR24-180-P"
        );
        assert_eq!(json_value.get("type").unwrap(), "stop");
        assert_eq!(json_value.get("detailed").unwrap(), true);
        assert_eq!(json_value.get("include_combos").unwrap(), false);
        assert_eq!(json_value.get("freeze_quotes").unwrap(), true);
    }

    #[test]
    fn test_request_parameters_serialization_with_trigger_all() {
        let request = CancelAllByInstrumentRequest {
            instrument_name: "BTC-28JUN24-65000-P".to_string(),
            order_type: Some(OrderType::TriggerAll),
            detailed: None,
            include_combos: None,
            freeze_quotes: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(
            json_value.get("instrument_name").unwrap(),
            "BTC-28JUN24-65000-P"
        );
        assert_eq!(json_value.get("type").unwrap(), "trigger_all");
    }

    #[test]
    fn test_request_parameters_serialization_with_trailing_stop() {
        let request = CancelAllByInstrumentRequest {
            instrument_name: "ETH-28JUN24-3500-C".to_string(),
            order_type: Some(OrderType::TrailingStop),
            detailed: Some(false),
            include_combos: Some(true),
            freeze_quotes: Some(false),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(
            json_value.get("instrument_name").unwrap(),
            "ETH-28JUN24-3500-C"
        );
        assert_eq!(json_value.get("type").unwrap(), "trailing_stop");
        assert_eq!(json_value.get("detailed").unwrap(), false);
        assert_eq!(json_value.get("include_combos").unwrap(), true);
        assert_eq!(json_value.get("freeze_quotes").unwrap(), false);
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": 5
        });

        let response: CancelAllByInstrumentResponse =
            serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, 5);
    }

    #[test]
    fn test_response_structures_deserialization_zero_cancelled() {
        let response_json = json!({
            "id": 42,
            "jsonrpc": "2.0",
            "result": 0
        });

        let response: CancelAllByInstrumentResponse =
            serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 42);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, 0);
    }

    #[tokio::test]
    async fn test_cancel_all_by_instrument_method_exists() {
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
        let _ = RestClient::cancel_all_by_instrument;

        // Verify the client exists
        let _ = &rest_client;

        println!("cancel_all_by_instrument method is accessible and properly typed");
    }
}
