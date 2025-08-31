//! Create order endpoint for Coinbase Exchange REST API
//!
//! Place a new order on the Coinbase Exchange.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::coinbaseexchange::{
    EndpointType, RestResult,
    enums::{OrderSide, OrderStatus, OrderType, SelfTradePrevention, StopDirection, TimeInForce},
    private_client::RestClient,
};

const ORDERS_ENDPOINT: &str = "orders";

/// Request to create a new order
#[derive(Debug, Clone, Serialize)]
pub struct CreateOrderRequest {
    /// Create order on a specific profile_id. If none is passed, defaults to default profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,

    /// Order type: limit, market, or stop
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Order side: buy or sell
    pub side: OrderSide,

    /// Product ID (e.g., "BTC-USD")
    pub product_id: String,

    /// Self-trade prevention flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp: Option<SelfTradePrevention>,

    /// Stop order direction (loss or entry)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopDirection>,

    /// Price threshold at which a stop order will be placed on the book
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,

    /// Price per unit of cryptocurrency - required for limit/stop orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Amount of base currency to buy or sell - required for limit/stop orders and market sells
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,

    /// Amount of quote currency to buy - required for market buys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funds: Option<String>,

    /// Time in force policy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Cancel after duration (min, hour, day) - requires time_in_force to be GTT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_after: Option<String>,

    /// If true, order will only execute as a maker order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,

    /// Optional order ID selected by the user or frontend client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,

    /// For iceberg orders - specifies how much to show
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_floor: Option<String>,

    /// Required for take profit/stop loss orders - the updated limit price upon stop loss trigger activation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_limit_price: Option<String>,
}

/// Response from creating a new order
#[derive(Debug, Clone, Deserialize)]
pub struct CreateOrderResponse {
    /// Order ID
    pub id: String,

    /// Price per unit of base currency
    #[serde(default)]
    pub price: String,

    /// Amount of base currency to buy/sell
    #[serde(default)]
    pub size: String,

    /// Product ID the order was placed on
    pub product_id: String,

    /// Profile ID that placed the order
    #[serde(default)]
    pub profile_id: String,

    /// Order side
    pub side: OrderSide,

    /// Amount of quote currency to spend (for market orders)
    #[serde(default)]
    pub funds: String,

    /// Funds with fees
    #[serde(default)]
    pub specified_funds: String,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Time in force policy
    #[serde(default)]
    pub time_in_force: TimeInForce,

    /// Timestamp at which order expires
    #[serde(default)]
    pub expire_time: Option<DateTime<Utc>>,

    /// If true, forces order to be maker only
    #[serde(default)]
    pub post_only: bool,

    /// Timestamp at which order was placed
    pub created_at: DateTime<Utc>,

    /// Timestamp at which order was done
    pub done_at: Option<DateTime<Utc>>,

    /// Reason order was done (filled, rejected, or otherwise)
    #[serde(default)]
    pub done_reason: String,

    /// Reason order was rejected by engine
    #[serde(default)]
    pub reject_reason: String,

    /// Fees paid on current filled amount
    #[serde(default)]
    pub fill_fees: String,

    /// Amount (in base currency) of the order that has been filled
    #[serde(default)]
    pub filled_size: String,

    /// Executed value
    #[serde(default)]
    pub executed_value: String,

    /// Order status
    pub status: OrderStatus,

    /// True if funds have been exchanged and settled
    #[serde(default)]
    pub settled: bool,

    /// Stop order direction
    pub stop: Option<StopDirection>,

    /// Price (in quote currency) at which to execute the order
    #[serde(default)]
    pub stop_price: String,

    /// Funding amount
    #[serde(default)]
    pub funding_amount: String,

    /// Client order ID
    #[serde(default)]
    pub client_oid: String,

    /// Market type where order was traded
    #[serde(default)]
    pub market_type: String,

    /// Stop limit price for TPSL order
    #[serde(default)]
    pub stop_limit_price: String,
}

impl RestClient {
    /// Create a new order
    ///
    /// Place a new order on the Coinbase Exchange. Orders can be limit, market, or stop orders.
    /// Each profile can place a maximum of 500 open orders on a product.
    ///
    /// [docs](https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_postorders)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The order creation request parameters including order type, side, and product
    ///
    /// # Returns
    /// A result containing the created order details with ID, status, and fill information
    ///
    /// # API Key Permissions
    /// This endpoint requires the "trade" permission.
    pub async fn create_order(
        &self,
        request: CreateOrderRequest,
    ) -> RestResult<CreateOrderResponse> {
        self.send_post_request(ORDERS_ENDPOINT, request, EndpointType::Private)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_order_request_serialization() {
        let request = CreateOrderRequest {
            profile_id: None,
            order_type: OrderType::Limit,
            side: OrderSide::Buy,
            product_id: "BTC-USD".to_string(),
            stp: Some(SelfTradePrevention::DecrementAndCancel),
            stop: None,
            stop_price: None,
            price: Some("50000.00".to_string()),
            size: Some("0.01".to_string()),
            funds: None,
            time_in_force: Some(TimeInForce::GoodTillCanceled),
            cancel_after: None,
            post_only: Some(false),
            client_oid: Some("test-order-123".to_string()),
            max_floor: None,
            stop_limit_price: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"type\":\"limit\""));
        assert!(json.contains("\"side\":\"buy\""));
        assert!(json.contains("\"product_id\":\"BTC-USD\""));
        assert!(json.contains("\"price\":\"50000.00\""));
        assert!(json.contains("\"size\":\"0.01\""));
        assert!(!json.contains("\"profile_id\"")); // Should be omitted when None
    }

    #[test]
    fn test_create_order_response_deserialization() {
        let json = r#"{
            "id": "d0c5340b-6d6c-49d9-b567-48c4bfca13d2",
            "price": "50000.00",
            "size": "0.01",
            "product_id": "BTC-USD",
            "profile_id": "default",
            "side": "buy",
            "funds": "",
            "specified_funds": "",
            "type": "limit",
            "time_in_force": "GTC",
            "post_only": false,
            "created_at": "2021-01-01T00:00:00.000Z",
            "done_at": null,
            "done_reason": "",
            "reject_reason": "",
            "fill_fees": "0.00",
            "filled_size": "0.00",
            "executed_value": "0.00",
            "status": "pending",
            "settled": false,
            "stop": null,
            "stop_price": "",
            "funding_amount": "",
            "client_oid": "",
            "market_type": "",
            "stop_limit_price": ""
        }"#;

        let response: CreateOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "d0c5340b-6d6c-49d9-b567-48c4bfca13d2");
        assert_eq!(response.price, "50000.00");
        assert_eq!(response.size, "0.01");
        assert_eq!(response.product_id, "BTC-USD");
        assert_eq!(response.side, OrderSide::Buy);
        assert_eq!(response.order_type, OrderType::Limit);
        assert_eq!(response.status, OrderStatus::Pending);
    }
}
