use serde::{Deserialize, Serialize};

use crate::cryptocom::{PrivateRestClient as RestClient, RestResult};

const ORDER_HISTORY_BY_INSTRUMENT_ENDPOINT: &str =
    "exchange/v1/private/get_order_history_by_instrument";

/// Parameters for get order history by instrument request
#[derive(Debug, Clone, Serialize)]
pub struct GetOrderHistoryByInstrumentRequest {
    /// Instrument name (required)
    pub instrument_name: String,

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

/// Order history by instrument entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderHistoryByInstrumentEntry {
    /// If order is a quote. Present only if true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<bool>,

    /// Whether the trigger order has been triggered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered: Option<bool>,

    /// Optional field with value true added only when created with Mobile Application
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<bool>,

    /// The name of the application that placed the order on behalf of the user (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,

    /// Implied volatility in percent. (Only if advanced="implv")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implv: Option<f64>,

    /// The initial display amount of iceberg order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_amount: Option<f64>,

    /// Option price in USD (Only if advanced="usd")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<f64>,

    /// The Ids of the orders that will be triggered if the order is filled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oto_order_ids: Option<Vec<String>>,

    /// true if created with API
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<bool>,

    /// Average fill price of the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_price: Option<f64>,

    /// Advanced type: "usd" or "implv" (Only for options; field is omitted if not applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced: Option<String>,

    /// Unique order identifier
    pub order_id: String,

    /// true for post-only orders only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,

    /// Filled amount of the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filled_amount: Option<f64>,

    /// Trigger type (only for trigger orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,

    /// Id of the trigger order that created the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_order_id: Option<String>,

    /// Direction: buy, or sell
    pub direction: String,

    /// Order size in contract units
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts: Option<f64>,

    /// true if the order is an order that can be triggered by another order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_secondary_oto: Option<bool>,

    /// true if the order was edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced: Option<bool>,

    /// Name of the MMP group supplied in the private/mass_quote request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp_group: Option<String>,

    /// true if the order is a MMP order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,

    /// The timestamp (milliseconds since the Unix epoch)
    pub last_update_timestamp: i64,

    /// The timestamp when order was created (milliseconds since the Unix epoch)
    pub creation_timestamp: i64,

    /// Order state
    pub order_state: String,

    /// Order type
    pub order_type: String,

    /// Price level of the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    /// Time in force
    pub time_in_force: String,

    /// Amount of the order
    pub amount: f64,

    /// Instrument name
    pub instrument_name: String,

    /// Cumulative amount filled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cumulative_amount: Option<f64>,

    /// Total fees paid by this order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cumulative_fee: Option<f64>,

    /// Client order ID if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
}

/// Response for get order history by instrument endpoint (simple format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderHistoryByInstrumentResponse {
    /// Array of order history data
    pub result: Vec<OrderHistoryByInstrumentEntry>,
}

/// Response for get order history by instrument endpoint (with continuation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderHistoryByInstrumentWithContinuationResponse {
    /// Array of order history data  
    pub result: Vec<OrderHistoryByInstrumentEntry>,

    /// Continuation token for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
}

impl RestClient {
    /// Get order history by instrument
    ///
    /// Retrieves history of orders that have been partially or fully filled.
    /// This is a private method; it can only be used after authentication.
    ///
    /// Scope: trade:read
    ///
    /// [docs](https://exchange-docs.crypto.com/derivatives/index.html)
    ///
    /// Rate limit: Not specified in documentation
    ///
    /// # Arguments
    /// * `params` - Request parameters including required instrument_name and optional parameters
    ///
    /// # Returns
    /// Order history information for the specified instrument
    pub async fn get_order_history_by_instrument(
        &self,
        params: GetOrderHistoryByInstrumentRequest,
    ) -> RestResult<GetOrderHistoryByInstrumentResponse> {
        self.send_signed_request(ORDER_HISTORY_BY_INSTRUMENT_ENDPOINT, params)
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
        let request = GetOrderHistoryByInstrumentRequest {
            instrument_name: "BTCUSD-PERP".to_string(),
            count: None,
            offset: None,
            include_old: None,
            include_unfilled: None,
            with_continuation: None,
            continuation: None,
            historical: None,
        };

        assert_eq!(request.instrument_name, "BTCUSD-PERP");
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
        let request = GetOrderHistoryByInstrumentRequest {
            instrument_name: "BTCUSD-PERP".to_string(),
            count: Some(50),
            offset: Some(10),
            include_old: Some(true),
            include_unfilled: Some(false),
            with_continuation: Some(true),
            continuation: None,
            historical: Some(false),
        };

        assert_eq!(request.instrument_name, "BTCUSD-PERP");
        assert_eq!(request.count, Some(50));
        assert_eq!(request.offset, Some(10));
        assert_eq!(request.include_old, Some(true));
        assert_eq!(request.include_unfilled, Some(false));
        assert_eq!(request.with_continuation, Some(true));
        assert_eq!(request.historical, Some(false));
    }

    #[test]
    fn test_request_serialization_minimal() {
        let request = GetOrderHistoryByInstrumentRequest {
            instrument_name: "BTCUSD-PERP".to_string(),
            count: None,
            offset: None,
            include_old: None,
            include_unfilled: None,
            with_continuation: None,
            continuation: None,
            historical: None,
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("instrument_name").unwrap(), "BTCUSD-PERP");

        // Check that optional fields are not present when None
        assert!(!json_value.as_object().unwrap().contains_key("count"));
        assert!(!json_value.as_object().unwrap().contains_key("offset"));
        assert!(!json_value.as_object().unwrap().contains_key("include_old"));
    }

    #[test]
    fn test_request_serialization_full() {
        let request = GetOrderHistoryByInstrumentRequest {
            instrument_name: "BTCUSD-PERP".to_string(),
            count: Some(20),
            offset: Some(5),
            include_old: Some(true),
            include_unfilled: Some(false),
            with_continuation: Some(true),
            continuation: Some("some_token".to_string()),
            historical: Some(false),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("instrument_name").unwrap(), "BTCUSD-PERP");
        assert_eq!(json_value.get("count").unwrap(), 20);
        assert_eq!(json_value.get("offset").unwrap(), 5);
        assert_eq!(json_value.get("include_old").unwrap(), true);
        assert_eq!(json_value.get("include_unfilled").unwrap(), false);
        assert_eq!(json_value.get("with_continuation").unwrap(), true);
        assert_eq!(json_value.get("continuation").unwrap(), "some_token");
        assert_eq!(json_value.get("historical").unwrap(), false);
    }

    #[test]
    fn test_order_history_entry_deserialization() {
        let entry_json = json!({
            "order_id": "12345",
            "direction": "buy",
            "last_update_timestamp": 1597026383085_i64,
            "creation_timestamp": 1597026383085_i64,
            "order_state": "filled",
            "order_type": "limit",
            "time_in_force": "good_till_cancel",
            "amount": 0.01,
            "instrument_name": "BTCUSD-PERP",
            "price": 50000.0,
            "filled_amount": 0.01,
            "average_price": 50000.0,
            "cumulative_amount": 0.01,
            "cumulative_fee": 1.25,
            "client_oid": "my_order_123",
            "api": true,
            "post_only": false
        });

        let entry: OrderHistoryByInstrumentEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.order_id, "12345");
        assert_eq!(entry.direction, "buy");
        assert_eq!(entry.order_state, "filled");
        assert_eq!(entry.order_type, "limit");
        assert_eq!(entry.instrument_name, "BTCUSD-PERP");
        assert_eq!(entry.amount, 0.01);
        assert_eq!(entry.price, Some(50000.0));
        assert_eq!(entry.filled_amount, Some(0.01));
        assert_eq!(entry.average_price, Some(50000.0));
        assert_eq!(entry.client_oid, Some("my_order_123".to_string()));
        assert_eq!(entry.api, Some(true));
        assert_eq!(entry.post_only, Some(false));
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

        let response: GetOrderHistoryByInstrumentResponse =
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
            ],
            "continuation": "next_page_token"
        });

        let response: GetOrderHistoryByInstrumentWithContinuationResponse =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result.len(), 1);
        assert_eq!(response.result[0].order_id, "12345");
        assert_eq!(response.continuation, Some("next_page_token".to_string()));
    }
}
