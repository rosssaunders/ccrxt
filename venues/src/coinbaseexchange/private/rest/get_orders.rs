//! Get orders endpoint for Coinbase Exchange REST API
//!
//! List current open orders and get individual order details.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{RestClient, get_account_balances::PaginationInfo};
use crate::coinbaseexchange::{
    EndpointType, RestResult,
    enums::{OrderSide, OrderStatus, OrderType, StopDirection, TimeInForce},
};

const ORDERS_ENDPOINT: &str = "orders";

/// Request to get all orders
#[derive(Debug, Clone, Serialize)]
pub struct GetOrdersRequest {
    /// Filter results by a specific profile_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,

    /// Filter results by a specific product_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,

    /// Sort criteria for results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorted_by: Option<String>,

    /// Ascending or descending order, by sortedBy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorting: Option<String>,

    /// Filter results by minimum posted date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateTime<Utc>>,

    /// Filter results by maximum posted date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateTime<Utc>>,

    /// Used for pagination. Sets start cursor to before date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Used for pagination. Sets end cursor to after date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Limit on number of results to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Array with order statuses to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<OrderStatus>>,

    /// Market type which the order was traded in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_type: Option<String>,
}

/// Request to get a single order
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetOrderRequest {
    /// Market type which the order was traded in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_type: Option<String>,
}

/// Order information
#[derive(Debug, Clone, Deserialize)]
pub struct Order {
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

/// Response from getting orders
pub type GetOrdersResponse = Vec<Order>;

/// Response from getting a single order
pub type GetOrderResponse = Order;

impl RestClient {
    /// Get all orders
    ///
    /// List your current open orders. Only open or un-settled orders are returned by default.
    /// As soon as an order is no longer open and settled, it will no longer appear in the default request.
    ///
    /// [docs](https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_getorders)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The orders request parameters
    ///
    /// # Returns
    /// A result containing the list of orders and pagination info or an error
    ///
    /// # API Key Permissions
    /// This endpoint requires either the "view" or "trade" permission.
    pub async fn get_orders(
        &self,
        request: GetOrdersRequest,
    ) -> RestResult<(GetOrdersResponse, Option<PaginationInfo>)> {
        self.send_get_request_with_pagination(ORDERS_ENDPOINT, request, EndpointType::Private)
            .await
    }

    /// Get a single order by ID
    ///
    /// Get a single order by order ID. Orders can be queried using either the exchange assigned ID
    /// or the client assigned client_oid. When using client_oid it must be preceded by the "client:" namespace.
    ///
    /// [docs](https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_getorder)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `order_id` - The order ID (either exchange ID or "client:client_oid")
    /// * `request` - Optional request parameters
    ///
    /// # Returns
    /// A result containing the order details or an error
    ///
    /// # API Key Permissions
    /// This endpoint requires either the "view" or "trade" permission.
    pub async fn get_order(
        &self,
        order_id: &str,
        request: GetOrderRequest,
    ) -> RestResult<GetOrderResponse> {
        let endpoint = format!("orders/{}", order_id);
        self.send_get_request(&endpoint, request, EndpointType::Private)
            .await
    }
}

impl Default for GetOrdersRequest {
    fn default() -> Self {
        Self {
            profile_id: None,
            product_id: None,
            sorted_by: None,
            sorting: None,
            start_date: None,
            end_date: None,
            before: None,
            after: None,
            limit: Some(100),
            status: Some(vec![OrderStatus::Open]),
            market_type: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_orders_request_serialization() {
        let request = GetOrdersRequest {
            profile_id: None,
            product_id: Some("BTC-USD".to_string()),
            sorted_by: None,
            sorting: None,
            start_date: None,
            end_date: None,
            before: None,
            after: None,
            limit: Some(50),
            status: None, // serde_urlencoded doesn't support Vec serialization
            market_type: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("product_id=BTC-USD"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_order_deserialization() {
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
            "expire_time": null,
            "post_only": false,
            "created_at": "2021-01-01T00:00:00.000Z",
            "done_at": null,
            "done_reason": "",
            "reject_reason": "",
            "fill_fees": "0.00",
            "filled_size": "0.00",
            "executed_value": "0.00",
            "status": "open",
            "settled": false,
            "stop": null,
            "stop_price": "",
            "funding_amount": "",
            "client_oid": "",
            "market_type": "",
            "stop_limit_price": ""
        }"#;

        let order: Order = serde_json::from_str(json).unwrap();
        assert_eq!(order.id, "d0c5340b-6d6c-49d9-b567-48c4bfca13d2");
        assert_eq!(order.price, "50000.00");
        assert_eq!(order.size, "0.01");
        assert_eq!(order.product_id, "BTC-USD");
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.status, OrderStatus::Open);
    }

    #[test]
    fn test_get_orders_response_deserialization() {
        let json = r#"[{
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
            "expire_time": null,
            "post_only": false,
            "created_at": "2021-01-01T00:00:00.000Z",
            "done_at": null,
            "done_reason": "",
            "reject_reason": "",
            "fill_fees": "0.00",
            "filled_size": "0.00",
            "executed_value": "0.00",
            "status": "open",
            "settled": false,
            "stop": null,
            "stop_price": "",
            "funding_amount": "",
            "client_oid": "",
            "market_type": "",
            "stop_limit_price": ""
        }]"#;

        let orders: GetOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].id, "d0c5340b-6d6c-49d9-b567-48c4bfca13d2");
    }
}
