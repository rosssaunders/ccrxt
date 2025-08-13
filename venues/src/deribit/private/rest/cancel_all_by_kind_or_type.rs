use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{
    Currency, EndpointType, InstrumentKind, JsonRpcResult, OrderType, RestResult,
};

/// REST API endpoint constant
const CANCEL_ALL_BY_KIND_OR_TYPE_ENDPOINT: &str = "private/cancel_all_by_kind_or_type";

/// Currency selection for cancel all by kind or type endpoint
/// Can be a single currency, array of currencies, or "any" for all currencies
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CurrencySelection {
    /// Array of currencies  
    Multiple(Vec<Currency>),

    /// Single currency or the special "any" value for all currencies
    Single(Currency),
}

impl CurrencySelection {
    /// Create a CurrencySelection for a single currency
    pub fn single(currency: Currency) -> Self {
        Self::Single(currency)
    }

    /// Create a CurrencySelection for multiple currencies
    pub fn multiple(currencies: Vec<Currency>) -> Self {
        Self::Multiple(currencies)
    }

    /// Create a CurrencySelection for all currencies ("any")
    pub fn any() -> Self {
        Self::Single(Currency::Any)
    }
}

impl<'de> Deserialize<'de> for CurrencySelection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde_json::Value;
        let value = Value::deserialize(deserializer)?;

        match value {
            Value::Array(arr) => {
                let currencies: Result<Vec<Currency>, _> = arr
                    .into_iter()
                    .map(|v| Currency::deserialize(v).map_err(serde::de::Error::custom))
                    .collect();
                Ok(CurrencySelection::Multiple(currencies?))
            }
            Value::String(_) => {
                let currency = Currency::deserialize(value).map_err(serde::de::Error::custom)?;
                Ok(CurrencySelection::Single(currency))
            }
            _ => Err(serde::de::Error::custom("Expected string or array")),
        }
    }
}

/// Request parameters for cancel all by kind or type endpoint
#[derive(Debug, Clone, Serialize)]
pub struct CancelAllByKindOrTypeRequest {
    /// The currency symbol, list of currency symbols or "any" for all (required)
    pub currency: CurrencySelection,

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

/// Response for cancel all by kind or type endpoint
pub type CancelAllByKindOrTypeResponse = JsonRpcResult<i64>;

impl RestClient {
    /// Cancel all orders in currency(currencies), optionally filtered by instrument kind and/or order type
    ///
    /// This method cancels all orders in currency(currencies), optionally filtered by instrument
    /// kind and/or order type. This endpoint requires trade:read_write scope.
    ///
    /// [docs](https://docs.deribit.com/v2/#private-cancel_all_by_kind_or_type)
    ///
    /// Rate limit: Matching engine endpoint (tier-based rate limiting)
    /// Scope: trade:read_write
    ///
    /// # Arguments
    /// * `currency` - The currency symbol, list of currency symbols or "any" for all
    /// * `kind` - Optional instrument kind filter
    /// * `order_type` - Optional order type filter
    /// * `detailed` - Optional flag for detailed output format
    /// * `freeze_quotes` - Optional flag to reject incoming quotes for 1 second after cancelling
    ///
    /// # Returns
    /// Result with total number of successfully cancelled orders
    pub async fn cancel_all_by_kind_or_type(
        &self,
        request: CancelAllByKindOrTypeRequest,
    ) -> RestResult<CancelAllByKindOrTypeResponse> {
        self.send_signed_request(
            CANCEL_ALL_BY_KIND_OR_TYPE_ENDPOINT,
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
    fn test_currency_selection_single() {
        let selection = CurrencySelection::single(Currency::BTC);
        let json_str = serde_json::to_string(&selection).unwrap();
        assert_eq!(json_str, "\"BTC\"");

        let deserialized: CurrencySelection = serde_json::from_str(&json_str).unwrap();
        match deserialized {
            CurrencySelection::Single(curr) => assert_eq!(curr, Currency::BTC),
            _ => assert_eq!(true, false, "Expected Single variant"),
        }
    }

    #[test]
    fn test_currency_selection_multiple() {
        let selection = CurrencySelection::multiple(vec![Currency::BTC, Currency::ETH]);
        let json_str = serde_json::to_string(&selection).unwrap();
        assert_eq!(json_str, "[\"BTC\",\"ETH\"]");

        let deserialized: CurrencySelection = serde_json::from_str(&json_str).unwrap();
        match deserialized {
            CurrencySelection::Multiple(currencies) => {
                assert_eq!(currencies.len(), 2);
                assert_eq!(currencies[0], Currency::BTC);
                assert_eq!(currencies[1], Currency::ETH);
            }
            _ => assert_eq!(true, false, "Expected Multiple variant"),
        }
    }

    #[test]
    fn test_currency_selection_any() {
        let selection = CurrencySelection::any();
        let json_str = serde_json::to_string(&selection).unwrap();
        assert_eq!(json_str, "\"any\"");

        let deserialized: CurrencySelection = serde_json::from_str(&json_str).unwrap();
        match deserialized {
            CurrencySelection::Single(Currency::Any) => (), // This is what we expect
            _ => assert_eq!(true, false, "Expected Single(Currency::Any) variant"),
        }
    }

    #[test]
    fn test_request_parameters_serialization_single_currency() {
        let request = CancelAllByKindOrTypeRequest {
            currency: CurrencySelection::single(Currency::BTC),
            kind: None,
            order_type: None,
            detailed: None,
            freeze_quotes: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        // Should contain required currency field as a string
        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        // Should not contain optional fields when None
        assert!(json_value.get("kind").is_none());
        assert!(json_value.get("type").is_none());
        assert!(json_value.get("detailed").is_none());
        assert!(json_value.get("freeze_quotes").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_multiple_currencies() {
        let request = CancelAllByKindOrTypeRequest {
            currency: CurrencySelection::multiple(vec![
                Currency::BTC,
                Currency::ETH,
                Currency::USDC,
            ]),
            kind: Some(InstrumentKind::Future),
            order_type: None,
            detailed: None,
            freeze_quotes: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        // Should contain currency field as an array
        let currency_array = json_value.get("currency").unwrap().as_array().unwrap();
        assert_eq!(currency_array.len(), 3);
        assert_eq!(currency_array[0], "BTC");
        assert_eq!(currency_array[1], "ETH");
        assert_eq!(currency_array[2], "USDC");

        assert_eq!(json_value.get("kind").unwrap(), "future");
        assert!(json_value.get("type").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_any_currency() {
        let request = CancelAllByKindOrTypeRequest {
            currency: CurrencySelection::any(),
            kind: Some(InstrumentKind::Option),
            order_type: Some(OrderType::Limit),
            detailed: Some(true),
            freeze_quotes: Some(false),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "any");
        assert_eq!(json_value.get("kind").unwrap(), "option");
        assert_eq!(json_value.get("type").unwrap(), "limit");
        assert_eq!(json_value.get("detailed").unwrap(), true);
        assert_eq!(json_value.get("freeze_quotes").unwrap(), false);
    }

    #[test]
    fn test_request_parameters_serialization_with_order_type() {
        let request = CancelAllByKindOrTypeRequest {
            currency: CurrencySelection::single(Currency::USDC),
            kind: None,
            order_type: Some(OrderType::Stop),
            detailed: None,
            freeze_quotes: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "USDC");
        assert_eq!(json_value.get("type").unwrap(), "stop");
        assert!(json_value.get("kind").is_none());
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": 5
        });

        let response: CancelAllByKindOrTypeResponse =
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

        let response: CancelAllByKindOrTypeResponse =
            serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 42);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, 0);
    }

    #[tokio::test]
    async fn test_cancel_all_by_kind_or_type_method_exists() {
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
        let _ = RestClient::cancel_all_by_kind_or_type;

        // Verify the client exists
        let _ = &rest_client;

        println!("cancel_all_by_kind_or_type method is accessible and properly typed");
    }
}
