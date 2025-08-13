use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{
    Currency, EndpointType, JsonRpcResult, OrderDirection, RestResult, TradeOrderType, TriggerType,
};

/// REST API endpoint constant
const GET_TRIGGER_ORDER_HISTORY_ENDPOINT: &str = "private/get_trigger_order_history";

/// Request parameters for getting trigger order history
#[derive(Debug, Clone, Serialize)]
pub struct GetTriggerOrderHistoryRequest {
    /// The currency symbol
    pub currency: Currency,

    /// Instrument name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,

    /// Number of requested items, default - 20 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,

    /// Continuation token for pagination (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
}

/// Individual trigger order entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerOrderEntry {
    /// It represents the requested order size. For perpetual and inverse futures the amount is in USD units.
    /// For options and linear futures and it is the underlying base currency coin.
    pub amount: f64,

    /// Direction: buy, or sell
    pub direction: OrderDirection,

    /// Unique instrument identifier
    pub instrument_name: String,

    /// true if the order is an order that can be triggered by another order, otherwise not present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_secondary_oto: Option<bool>,

    /// User defined label (presented only when previously set for order by user)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// The timestamp (milliseconds since the Unix epoch)
    pub last_update_timestamp: i64,

    /// Unique reference that identifies a one_cancels_others (OCO) pair
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oco_ref: Option<String>,

    /// Unique order identifier
    pub order_id: String,

    /// Order state: "triggered", "cancelled", or "rejected" with rejection reason
    pub order_state: String,

    /// Requested order type: "limit" or "market"
    pub order_type: TradeOrderType,

    /// true for post-only orders only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,

    /// Price in base currency
    pub price: f64,

    /// Optional (not added for spot). 'true for reduce-only orders only'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// Type of last request performed on the trigger order by user or system.
    /// "cancel" - when order was cancelled, "trigger:order" - when trigger order spawned market or limit order after being triggered
    pub request: String,

    /// Source of the order that is linked to the trigger order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// The timestamp (milliseconds since the Unix epoch)
    pub timestamp: i64,

    /// Trigger type (only for trigger orders). Allowed values: "index_price", "mark_price", "last_price"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<TriggerType>,

    /// The maximum deviation from the price peak beyond which the order will be triggered (Only for trailing trigger orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_offset: Option<f64>,

    /// Id of the user order used for the trigger-order reference before triggering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_order_id: Option<String>,

    /// Trigger price (Only for future trigger orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<f64>,
}

/// Result data for get trigger order history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTriggerOrderHistoryResult {
    /// Continuation token for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,

    /// Array of trigger order entries
    pub entries: Vec<TriggerOrderEntry>,
}

/// Response for get trigger order history endpoint
pub type GetTriggerOrderHistoryResponse = JsonRpcResult<GetTriggerOrderHistoryResult>;

impl RestClient {
    /// Retrieves detailed log of the user's trigger orders
    ///
    /// This is a private method; it can only be used after authentication.
    /// Scope: trade:read
    ///
    /// [docs](https://docs.deribit.com/v2/#private-get_trigger_order_history)
    ///
    /// Rate limit: Non-matching engine rate limits apply (500 credits)
    ///
    /// # Arguments
    /// * `currency` - The currency symbol (required)
    /// * `instrument_name` - Instrument name (optional)
    /// * `count` - Number of requested items, default - 20 (optional)
    /// * `continuation` - Continuation token for pagination (optional)
    ///
    /// # Returns
    /// Trigger order history information for the specified currency
    pub async fn get_trigger_order_history(
        &self,
        request: GetTriggerOrderHistoryRequest,
    ) -> RestResult<GetTriggerOrderHistoryResponse> {
        self.send_signed_request(
            GET_TRIGGER_ORDER_HISTORY_ENDPOINT,
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
    /// REST API endpoint constant
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
        let request = GetTriggerOrderHistoryRequest {
            currency: Currency::BTC,
            instrument_name: Some("BTC-PERPETUAL".to_string()),
            count: Some(50),
            continuation: Some("continuation_token_123".to_string()),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(json_value.get("instrument_name").unwrap(), "BTC-PERPETUAL");
        assert_eq!(json_value.get("count").unwrap(), 50);
        assert_eq!(
            json_value.get("continuation").unwrap(),
            "continuation_token_123"
        );
    }

    #[test]
    fn test_request_with_minimal_parameters() {
        let request = GetTriggerOrderHistoryRequest {
            currency: Currency::ETH,
            instrument_name: None,
            count: None,
            continuation: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "ETH");

        // Optional fields should not be present when None
        assert!(
            !json_value
                .as_object()
                .unwrap()
                .contains_key("instrument_name")
        );
        assert!(!json_value.as_object().unwrap().contains_key("count"));
        assert!(!json_value.as_object().unwrap().contains_key("continuation"));
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "continuation": "next_token_456",
                "entries": [
                    {
                        "amount": 1000.0,
                        "direction": "buy",
                        "instrument_name": "BTC-PERPETUAL",
                        "is_secondary_oto": false,
                        "label": "my_trigger_order",
                        "last_update_timestamp": 1640995200000i64,
                        "oco_ref": "oco_123",
                        "order_id": "TRG-123456",
                        "order_state": "triggered",
                        "order_type": "limit",
                        "post_only": true,
                        "price": 45000.0,
                        "reduce_only": false,
                        "request": "trigger:order",
                        "source": "web",
                        "timestamp": 1640995200000i64,
                        "trigger": "index_price",
                        "trigger_offset": 100.0,
                        "trigger_order_id": "TRG-REF-123",
                        "trigger_price": 44000.0
                    }
                ]
            }
        });

        let response: GetTriggerOrderHistoryResponse =
            serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(
            response.result.continuation,
            Some("next_token_456".to_string())
        );
        assert_eq!(response.result.entries.len(), 1);

        let entry = &response.result.entries[0];
        assert_eq!(entry.amount, 1000.0);
        assert_eq!(entry.direction, OrderDirection::Buy);
        assert_eq!(entry.instrument_name, "BTC-PERPETUAL");
        assert_eq!(entry.is_secondary_oto, Some(false));
        assert_eq!(entry.label, Some("my_trigger_order".to_string()));
        assert_eq!(entry.last_update_timestamp, 1640995200000);
        assert_eq!(entry.oco_ref, Some("oco_123".to_string()));
        assert_eq!(entry.order_id, "TRG-123456");
        assert_eq!(entry.order_state, "triggered");
        assert_eq!(entry.order_type, TradeOrderType::Limit);
        assert_eq!(entry.post_only, Some(true));
        assert_eq!(entry.price, 45000.0);
        assert_eq!(entry.reduce_only, Some(false));
        assert_eq!(entry.request, "trigger:order");
        assert_eq!(entry.source, Some("web".to_string()));
        assert_eq!(entry.timestamp, 1640995200000);
        assert_eq!(entry.trigger, Some(TriggerType::IndexPrice));
        assert_eq!(entry.trigger_offset, Some(100.0));
        assert_eq!(entry.trigger_order_id, Some("TRG-REF-123".to_string()));
        assert_eq!(entry.trigger_price, Some(44000.0));
    }

    #[test]
    fn test_trigger_order_entry_with_minimal_fields() {
        let entry_json = json!({
            "amount": 500.0,
            "direction": "sell",
            "instrument_name": "ETH-PERPETUAL",
            "last_update_timestamp": 1640995300000i64,
            "order_id": "TRG-789123",
            "order_state": "cancelled",
            "order_type": "market",
            "price": 3000.0,
            "request": "cancel",
            "timestamp": 1640995300000i64
        });

        let entry: TriggerOrderEntry = serde_json::from_value(entry_json).unwrap();

        assert_eq!(entry.amount, 500.0);
        assert_eq!(entry.direction, OrderDirection::Sell);
        assert_eq!(entry.instrument_name, "ETH-PERPETUAL");
        assert_eq!(entry.is_secondary_oto, None);
        assert_eq!(entry.label, None);
        assert_eq!(entry.last_update_timestamp, 1640995300000);
        assert_eq!(entry.oco_ref, None);
        assert_eq!(entry.order_id, "TRG-789123");
        assert_eq!(entry.order_state, "cancelled");
        assert_eq!(entry.order_type, TradeOrderType::Market);
        assert_eq!(entry.post_only, None);
        assert_eq!(entry.price, 3000.0);
        assert_eq!(entry.reduce_only, None);
        assert_eq!(entry.request, "cancel");
        assert_eq!(entry.source, None);
        assert_eq!(entry.timestamp, 1640995300000);
        assert_eq!(entry.trigger, None);
        assert_eq!(entry.trigger_offset, None);
        assert_eq!(entry.trigger_order_id, None);
        assert_eq!(entry.trigger_price, None);
    }

    #[test]
    fn test_response_with_empty_entries() {
        let response_json = json!({
            "id": 2,
            "jsonrpc": "2.0",
            "result": {
                "entries": []
            }
        });

        let response: GetTriggerOrderHistoryResponse =
            serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 2);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.continuation, None);
        assert_eq!(response.result.entries.len(), 0);
    }

    #[test]
    fn test_rejected_order_state() {
        let entry_json = json!({
            "amount": 100.0,
            "direction": "buy",
            "instrument_name": "BTC-25DEC24-40000-C",
            "last_update_timestamp": 1640995400000i64,
            "order_id": "TRG-REJECTED-001",
            "order_state": "rejected:reduce_direction",
            "order_type": "limit",
            "price": 41000.0,
            "request": "cancel",
            "timestamp": 1640995400000i64
        });

        let entry: TriggerOrderEntry = serde_json::from_value(entry_json).unwrap();

        assert_eq!(entry.order_state, "rejected:reduce_direction");
        assert_eq!(entry.order_id, "TRG-REJECTED-001");
        assert_eq!(entry.amount, 100.0);
    }

    #[tokio::test]
    async fn test_get_trigger_order_history_method_exists() {
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
        let _ = RestClient::get_trigger_order_history;

        // Verify the client exists
        let _ = &rest_client;

        println!("get_trigger_order_history method is accessible and properly typed");
    }
}
