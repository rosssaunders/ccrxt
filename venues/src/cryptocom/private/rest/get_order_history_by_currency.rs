use serde::{Deserialize, Serialize};

use super::client::RestClient;
use super::get_order_history_by_instrument::OrderHistoryByInstrumentEntry;
use crate::cryptocom::RestResult;

const ORDER_HISTORY_BY_CURRENCY_ENDPOINT: &str = "private/get_order_history_by_currency";
/// Parameters for get order history by currency request
#[derive(Debug, Clone, Serialize)]
pub struct GetOrderHistoryByCurrencyRequest {
    /// The currency symbol (required)
    pub currency: String,
    /// Instrument kind filter (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Number of requested items, default - 20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// The offset for pagination, default - 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Include in result orders older than 2 days, default - false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_old: Option<bool>,
    /// Include in result fully unfilled closed orders, default - false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_unfilled: Option<bool>,
    /// When set to true, the API response format changes from a simple list of orders to an object containing the orders and a continuation token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_continuation: Option<bool>,
    /// Continuation token for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
    /// Determines whether historical trade and order records should be retrieved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical: Option<bool>,
}

/// Response for get order history by currency endpoint (simple format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderHistoryByCurrencyResponse {
    /// Array of order history data
    pub result: Vec<OrderHistoryByInstrumentEntry>,
}

/// Response for get order history by currency endpoint (with continuation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderHistoryByCurrencyWithContinuationResponse {
    /// Array of order history data  
    pub result: Vec<OrderHistoryByInstrumentEntry>,
    /// Continuation token for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
}

impl RestClient {
    /// Get order history by currency
    ///
    /// Retrieves history of orders that have been partially or fully filled.
    /// This is a private method; it can only be used after authentication.
    ///
    /// Scope: trade:read
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html>
    ///
    /// Rate limit: Not specified in documentation
    ///
    /// # Arguments
    /// * `params` - Request parameters including required currency and optional parameters
    ///
    /// # Returns
    /// Order history information for the specified currency
    pub async fn get_order_history_by_currency(
        &self,
        params: GetOrderHistoryByCurrencyRequest,
    ) -> RestResult<GetOrderHistoryByCurrencyResponse> {
        self.send_signed_request(ORDER_HISTORY_BY_CURRENCY_ENDPOINT, params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rest::secrets::ExposableSecret;
    use serde_json::json;

    use super::*;

    /// A plain text implementation of ExposableSecret for testing purposes.
    #[derive(Clone)]
    #[allow(dead_code)]
    struct PlainTextSecret {
        secret: String,
    }

    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    impl PlainTextSecret {
        #[allow(dead_code)]
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    #[test]
    fn test_request_creation_minimal() {
        let request = GetOrderHistoryByCurrencyRequest {
            currency: "BTC".to_string(),
            kind: None,
            count: None,
            offset: None,
            include_old: None,
            include_unfilled: None,
            with_continuation: None,
            continuation: None,
            historical: None,
        };

        assert_eq!(request.currency, "BTC");
        assert!(request.kind.is_none());
        assert!(request.count.is_none());
        assert!(request.offset.is_none());
        assert!(request.include_old.is_none());
        assert!(request.include_unfilled.is_none());
        assert!(request.with_continuation.is_none());
        assert!(request.continuation.is_none());
        assert!(request.historical.is_none());
    }

    #[test]
    fn test_request_creation_with_builder() {
        let request = GetOrderHistoryByCurrencyRequest {
            currency: "ETH".to_string(),
            kind: Some("future".to_string()),
            count: Some(50),
            offset: Some(10),
            include_old: Some(true),
            include_unfilled: Some(false),
            with_continuation: Some(true),
            continuation: None,
            historical: Some(false),
        };

        assert_eq!(request.currency, "ETH");
        assert_eq!(request.kind, Some("future".to_string()));
        assert_eq!(request.count, Some(50));
        assert_eq!(request.offset, Some(10));
        assert_eq!(request.include_old, Some(true));
        assert_eq!(request.include_unfilled, Some(false));
        assert_eq!(request.with_continuation, Some(true));
        assert_eq!(request.historical, Some(false));
    }

    #[test]
    fn test_request_serialization_minimal() {
        let request = GetOrderHistoryByCurrencyRequest {
            currency: "USDC".to_string(),
            kind: None,
            count: None,
            offset: None,
            include_old: None,
            include_unfilled: None,
            with_continuation: None,
            continuation: None,
            historical: None,
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("currency").unwrap(), "USDC");

        // Check that optional fields are not present when None
        assert!(!json_value.as_object().unwrap().contains_key("kind"));
        assert!(!json_value.as_object().unwrap().contains_key("count"));
        assert!(!json_value.as_object().unwrap().contains_key("offset"));
        assert!(!json_value.as_object().unwrap().contains_key("include_old"));
    }

    #[test]
    fn test_request_serialization_full() {
        let request = GetOrderHistoryByCurrencyRequest {
            currency: "USDT".to_string(),
            kind: Some("option".to_string()),
            count: Some(20),
            offset: Some(5),
            include_old: Some(true),
            include_unfilled: Some(false),
            with_continuation: Some(true),
            continuation: Some("some_token".to_string()),
            historical: Some(false),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("currency").unwrap(), "USDT");
        assert_eq!(json_value.get("kind").unwrap(), "option");
        assert_eq!(json_value.get("count").unwrap(), 20);
        assert_eq!(json_value.get("offset").unwrap(), 5);
        assert_eq!(json_value.get("include_old").unwrap(), true);
        assert_eq!(json_value.get("include_unfilled").unwrap(), false);
        assert_eq!(json_value.get("with_continuation").unwrap(), true);
        assert_eq!(json_value.get("continuation").unwrap(), "some_token");
        assert_eq!(json_value.get("historical").unwrap(), false);
    }

    #[test]
    fn test_response_deserialization() {
        let response_json = json!({
            "result": [
                {
                    "order_id": "12345",
                    "direction": "buy",
                    "last_update_timestamp": 1597026383085_i64,
                    "creation_timestamp": 1597026383085_i64,
                    "order_state": "filled",
                    "order_type": "limit",
                    "time_in_force": "good_till_cancel",
                    "amount": 0.01,
                    "instrument_name": "BTCUSD-PERP"
                }
            ]
        });

        let response: GetOrderHistoryByCurrencyResponse =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result.len(), 1);
        assert_eq!(response.result[0].order_id, "12345");
        assert_eq!(response.result[0].instrument_name, "BTCUSD-PERP");
    }

    #[test]
    fn test_response_with_continuation_deserialization() {
        let response_json = json!({
            "result": [
                {
                    "order_id": "67890",
                    "direction": "sell",
                    "last_update_timestamp": 1597026383085_i64,
                    "creation_timestamp": 1597026383085_i64,
                    "order_state": "partially_filled",
                    "order_type": "market",
                    "time_in_force": "immediate_or_cancel",
                    "amount": 0.05,
                    "instrument_name": "ETHUSD-PERP"
                }
            ],
            "continuation": "next_page_token"
        });

        let response: GetOrderHistoryByCurrencyWithContinuationResponse =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result.len(), 1);
        assert_eq!(response.result[0].order_id, "67890");
        assert_eq!(response.continuation, Some("next_page_token".to_string()));
    }

    #[test]
    fn test_supported_currencies() {
        // Test that all documented currencies work
        let currencies = vec!["BTC", "ETH", "USDC", "USDT", "EURR"];

        for currency in currencies {
            let request = GetOrderHistoryByCurrencyRequest {
                currency: currency.to_string(),
                kind: None,
                count: None,
                offset: None,
                include_old: None,
                include_unfilled: None,
                with_continuation: None,
                continuation: None,
                historical: None,
            };
            assert_eq!(request.currency, currency);

            let json_value = serde_json::to_value(request).unwrap();
            assert_eq!(json_value.get("currency").unwrap(), currency);
        }
    }

    #[test]
    fn test_supported_kinds() {
        // Test that all documented kinds work
        let kinds = vec![
            "future",
            "option",
            "spot",
            "future_combo",
            "option_combo",
            "combo",
            "any",
        ];

        for kind in kinds {
            let request = GetOrderHistoryByCurrencyRequest {
                currency: "BTC".to_string(),
                kind: Some(kind.to_string()),
                count: None,
                offset: None,
                include_old: None,
                include_unfilled: None,
                with_continuation: None,
                continuation: None,
                historical: None,
            };
            assert_eq!(request.kind, Some(kind.to_string()));

            let json_value = serde_json::to_value(request).unwrap();
            assert_eq!(json_value.get("kind").unwrap(), kind);
        }
    }
}
