use serde::Serialize;

use super::RestClient;
use crate::deribit::{
    Currency, EndpointType, InstrumentKind, JsonRpcResult, OrderType, RestResult,
};

/// REST API endpoint constant
const CANCEL_ALL_BY_CURRENCY_ENDPOINT: &str = "private/cancel_all_by_currency";

/// Request parameters for cancel all by currency endpoint
#[derive(Debug, Clone, Serialize)]
pub struct CancelAllByCurrencyRequest {
    /// The currency symbol (required)
    pub currency: Currency,

    /// Instrument kind filter (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<InstrumentKind>,

    /// Order type filter (optional)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<OrderType>,

    /// When detailed is set to true output format is changed (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed: Option<bool>,

    /// Whether or not to reject incoming quotes for 1 second after cancelling (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freeze_quotes: Option<bool>,
}

/// Response for cancel all by currency endpoint
pub type CancelAllByCurrencyResponse = JsonRpcResult<i64>;

impl RestClient {
    /// Cancel all orders by currency, optionally filtered by instrument kind and/or order type
    ///
    /// This method cancels all orders by currency, optionally filtered by instrument kind and/or
    /// order type. This endpoint requires trade:read_write scope.
    ///
    /// See: <https://docs.deribit.com/v2/#private-cancel_all_by_currency>
    ///
    /// Rate limit: Matching engine endpoint (tier-based rate limiting)
    /// Scope: trade:read_write
    ///
    /// # Arguments
    /// * `currency` - The currency symbol
    /// * `kind` - Optional instrument kind filter
    /// * `order_type` - Optional order type filter
    /// * `detailed` - Optional flag for detailed output format
    /// * `freeze_quotes` - Optional flag to reject incoming quotes for 1 second after cancelling
    ///
    /// # Returns
    /// Result with total number of successfully cancelled orders
    pub async fn cancel_all_by_currency(
        &self,
        request: CancelAllByCurrencyRequest,
    ) -> RestResult<CancelAllByCurrencyResponse> {
        self.send_signed_request(
            CANCEL_ALL_BY_CURRENCY_ENDPOINT,
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
        let request = CancelAllByCurrencyRequest {
            currency: Currency::BTC,
            kind: None,
            order_type: None,
            detailed: None,
            freeze_quotes: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        // Should contain required currency field
        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        // Should not contain optional fields when None
        assert!(json_value.get("kind").is_none());
        assert!(json_value.get("type").is_none());
        assert!(json_value.get("detailed").is_none());
        assert!(json_value.get("freeze_quotes").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_with_kind() {
        let request = CancelAllByCurrencyRequest {
            currency: Currency::ETH,
            kind: Some(InstrumentKind::Future),
            order_type: None,
            detailed: None,
            freeze_quotes: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "ETH");
        assert_eq!(json_value.get("kind").unwrap(), "future");
        assert!(json_value.get("type").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_with_order_type() {
        let request = CancelAllByCurrencyRequest {
            currency: Currency::USDC,
            kind: None,
            order_type: Some(OrderType::Limit),
            detailed: None,
            freeze_quotes: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "USDC");
        assert_eq!(json_value.get("type").unwrap(), "limit");
        assert!(json_value.get("kind").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_full() {
        let request = CancelAllByCurrencyRequest {
            currency: Currency::USDT,
            kind: Some(InstrumentKind::Option),
            order_type: Some(OrderType::Stop),
            detailed: Some(true),
            freeze_quotes: Some(false),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "USDT");
        assert_eq!(json_value.get("kind").unwrap(), "option");
        assert_eq!(json_value.get("type").unwrap(), "stop");
        assert_eq!(json_value.get("detailed").unwrap(), true);
        assert_eq!(json_value.get("freeze_quotes").unwrap(), false);
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": 3
        });

        let response: CancelAllByCurrencyResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, 3);
    }

    #[test]
    fn test_response_structures_deserialization_zero_cancelled() {
        let response_json = json!({
            "id": 42,
            "jsonrpc": "2.0",
            "result": 0
        });

        let response: CancelAllByCurrencyResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 42);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, 0);
    }

    #[tokio::test]
    async fn test_cancel_all_by_currency_method_exists() {
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
        let _ = RestClient::cancel_all_by_currency;

        // Verify the client exists
        let _ = &rest_client;

        println!("cancel_all_by_currency method is accessible and properly typed");
    }
}
