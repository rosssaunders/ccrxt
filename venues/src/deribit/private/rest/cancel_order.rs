use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{
    AdvancedType, CancelReason, EndpointType, OrderDirection, OrderState, RestResult, TriggerType,
};

/// Request parameters for canceling an order
#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderRequest {
    /// The order id
    pub order_id: String,
}

/// Order details in cancel response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelledOrder {
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
    pub advanced: Option<AdvancedType>,

    /// Unique order identifier
    pub order_id: String,

    /// true for post-only orders only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,

    /// Filled amount of the order
    pub filled_amount: f64,

    /// Trigger type (only for trigger orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<TriggerType>,

    /// Id of the trigger order that created the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_order_id: Option<String>,

    /// Direction: buy, or sell
    pub direction: OrderDirection,

    /// It represents the order size in contract units
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts: Option<f64>,

    /// true if the order is an order that can be triggered by another order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_secondary_oto: Option<bool>,

    /// true if the order was edited (by user or - in case of advanced options orders - by pricing engine)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced: Option<bool>,

    /// Name of the MMP group supplied in the private/mass_quote request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp_group: Option<String>,

    /// true if the order is a MMP order, otherwise false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,

    /// The timestamp (milliseconds since the Unix epoch)
    pub last_update_timestamp: i64,

    /// The timestamp (milliseconds since the Unix epoch)
    pub creation_timestamp: i64,

    /// Enumerated reason behind cancel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<CancelReason>,

    /// true if order was cancelled by mmp trigger (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp_cancelled: Option<bool>,

    /// The same QuoteID as supplied in the private/mass_quote request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,

    /// Order state: "open", "filled", "rejected", "cancelled", "untriggered"
    pub order_state: OrderState,

    /// Order type: "limit", "market", "stop_limit", "stop_market"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,

    /// Maximum amount within an order to be shown to other traders, 0 for invisible order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_show: Option<f64>,

    /// It represents the requested order size
    pub amount: f64,

    /// Instrument name
    pub instrument_name: String,

    /// The original order type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_order_type: Option<String>,

    /// Price in base currency
    pub price: f64,

    /// Specifies the time in force
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,

    /// true for reduce-only orders only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// Profit and loss (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profit_loss: Option<f64>,

    /// true if user order, false if order by the exchange
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_liquidation: Option<bool>,

    /// true if the order was automatically created during liquidation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_replaced: Option<bool>,

    /// user defined label (up to 64 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Price trigger offset value (only for stop orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_offset: Option<f64>,

    /// The stop price (only for stop orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<f64>,

    /// Option to add the order to the order book when the instrument cross the trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_fill_condition: Option<String>,

    /// Reject post only when cross fills immediately against own order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_post_only: Option<bool>,

    /// Commission (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission: Option<f64>,

    /// Limits what happens to a quantity and price of an order on a fill
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_condition: Option<String>,

    /// Id of the main order which was used to create this order (only if the order was created automatically)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otoco_order_id: Option<String>,

    /// true if Oco order, false otherwise
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otoco: Option<bool>,

    /// Specifies if the order is bracketed order  
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracketed: Option<bool>,

    /// The take profit price (only for bracketed orders)  
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit_price: Option<f64>,

    /// The stop loss price (only for bracketed orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss_price: Option<f64>,
}

/// Response for cancel order endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrderResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Cancel result data
    pub result: CancelledOrder,
}

impl RestClient {
    /// Cancel an order, specified by order id
    ///
    /// This is a private method; it can only be used after authentication.
    /// This is a matching engine method.
    ///
    /// See: <https://docs.deribit.com/v2/#private-cancel>
    ///
    /// Rate limit: Matching engine rate limits apply based on account tier
    /// Scope: trade:read_write
    ///
    /// # Arguments
    /// * `order_id` - The order id (string)
    ///
    /// # Returns
    /// Cancel result with complete order information
    pub async fn cancel_order(
        &self,
        request: CancelOrderRequest,
    ) -> RestResult<CancelOrderResponse> {
        self.send_signed_request(
            "private/cancel_order",
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
    fn test_request_parameters_serialization() {
        let request = CancelOrderRequest {
            order_id: "ETH-29MAR19-200-C".to_string(),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("order_id").unwrap(), "ETH-29MAR19-200-C");
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "order_id": "ETH-29MAR19-200-C",
                "order_state": "cancelled",
                "direction": "buy",
                "amount": 10.0,
                "filled_amount": 0.0,
                "price": 50.0,
                "instrument_name": "ETH-29MAR19-200-C",
                "creation_timestamp": 1550652954360i64,
                "last_update_timestamp": 1550652954360i64,
                "cancel_reason": "user_request"
            }
        });

        let response: CancelOrderResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.order_id, "ETH-29MAR19-200-C");
        assert_eq!(response.result.order_state, OrderState::Cancelled);
        assert_eq!(response.result.direction, OrderDirection::Buy);
        assert_eq!(response.result.amount, 10.0);
        assert_eq!(response.result.filled_amount, 0.0);
        assert_eq!(response.result.price, 50.0);
        assert_eq!(response.result.instrument_name, "ETH-29MAR19-200-C");
        assert_eq!(
            response.result.cancel_reason,
            Some(CancelReason::UserRequest)
        );
    }

    #[test]
    fn test_response_with_optional_fields() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "order_id": "BTC-29MAR19-4000-C",
                "order_state": "cancelled",
                "direction": "sell",
                "amount": 5.0,
                "filled_amount": 2.0,
                "price": 100.0,
                "instrument_name": "BTC-29MAR19-4000-C",
                "creation_timestamp": 1550652954360i64,
                "last_update_timestamp": 1550652954370i64,
                "cancel_reason": "mmp_trigger",
                "average_price": 102.5,
                "api": true,
                "post_only": false,
                "mmp": true,
                "mmp_cancelled": true,
                "advanced": "usd",
                "usd": 100.0,
                "reduce_only": true,
                "label": "my_order_123"
            }
        });

        let response: CancelOrderResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.result.order_state, OrderState::Cancelled);
        assert_eq!(response.result.direction, OrderDirection::Sell);
        assert_eq!(
            response.result.cancel_reason,
            Some(CancelReason::MmpTrigger)
        );
        assert_eq!(response.result.average_price, Some(102.5));
        assert_eq!(response.result.api, Some(true));
        assert_eq!(response.result.post_only, Some(false));
        assert_eq!(response.result.mmp, Some(true));
        assert_eq!(response.result.mmp_cancelled, Some(true));
        assert_eq!(response.result.advanced, Some(AdvancedType::Usd));
        assert_eq!(response.result.usd, Some(100.0));
        assert_eq!(response.result.reduce_only, Some(true));
        assert_eq!(response.result.label, Some("my_order_123".to_string()));
    }

    #[test]
    fn test_response_with_trigger_fields() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "order_id": "BTC-PERPETUAL",
                "order_state": "cancelled",
                "direction": "buy",
                "amount": 1000.0,
                "filled_amount": 0.0,
                "price": 45000.0,
                "instrument_name": "BTC-PERPETUAL",
                "creation_timestamp": 1550652954360i64,
                "last_update_timestamp": 1550652954360i64,
                "cancel_reason": "user_request",
                "triggered": false,
                "trigger": "mark_price",
                "trigger_order_id": "ETH-123456",
                "stop_price": 44000.0,
                "trigger_offset": 100.0
            }
        });

        let response: CancelOrderResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.result.triggered, Some(false));
        assert_eq!(response.result.trigger, Some(TriggerType::MarkPrice));
        assert_eq!(
            response.result.trigger_order_id,
            Some("ETH-123456".to_string())
        );
        assert_eq!(response.result.stop_price, Some(44000.0));
        assert_eq!(response.result.trigger_offset, Some(100.0));
    }

    #[tokio::test]
    async fn test_cancel_order_method_exists() {
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
        let _ = RestClient::cancel_order;

        // Verify the client exists
        let _ = &rest_client;

        println!("cancel_order method is accessible and properly typed");
    }
}
