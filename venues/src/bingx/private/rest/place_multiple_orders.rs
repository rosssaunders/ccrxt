use serde::{Deserialize, Serialize};

use super::{
    RestClient,
    place_order::{OrderSide, OrderStatus, OrderType, TimeInForce},
};
use crate::bingx::{EndpointType, RestResult};

const PLACE_MULTIPLE_ORDERS_ENDPOINT: &str = "/openApi/spot/v1/trade/batchOrders";

/// Request for placing multiple orders in batch
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceMultipleOrdersRequest {
    /// Array of order data, limited to 5 orders
    pub data: Vec<OrderData>,
    /// Sync mode: false (default) = parallel ordering, true = serial ordering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync: Option<bool>,
    /// Request valid time window value in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
    /// Request timestamp in milliseconds
    pub timestamp: i64,
}

/// Individual order data for batch orders
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderData {
    /// Trading symbol, e.g., BTC-USDT
    pub symbol: String,
    /// Order side: BUY or SELL
    pub side: OrderSide,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Order trigger price for stop orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,
    /// Order quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    /// Quote order quantity (order amount)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_order_qty: Option<String>,
    /// Order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Custom order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,
    /// Time in force
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
}

/// Response for placing multiple orders
#[derive(Debug, Clone, Deserialize)]
pub struct PlaceMultipleOrdersResponse {
    /// Array of order responses
    pub orders: Vec<BatchOrderResponse>,
}

/// Individual order response in batch
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderResponse {
    /// Trading symbol
    pub symbol: String,
    /// Order ID
    pub order_id: i64,
    /// Transaction timestamp
    pub transact_time: i64,
    /// Order price
    pub price: String,
    /// Original quantity
    pub orig_qty: String,
    /// Executed quantity
    pub executed_qty: String,
    /// Cumulative quote asset transacted quantity
    pub cummulative_quote_qty: String,
    /// Order status
    pub status: OrderStatus,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Order side
    pub side: OrderSide,
    /// Custom order ID
    #[serde(rename = "clientOrderID")]
    pub client_order_id: String,
}

impl RestClient {
    /// Place multiple orders in batch
    ///
    /// Places multiple orders in a single request (up to 5 orders).
    /// Rate limit: 10/s by UID
    ///
    /// # Arguments
    /// * `request` - The batch order request containing multiple orders
    ///
    /// # Returns
    /// A result containing the batch order response or an error
    ///
    /// # Notes
    /// - Maximum of 5 orders per batch
    /// - sync=false (default): parallel ordering, sync=true: serial ordering
    pub async fn place_multiple_orders(
        &self,
        request: &PlaceMultipleOrdersRequest,
    ) -> RestResult<PlaceMultipleOrdersResponse> {
        self.send_request(
            PLACE_MULTIPLE_ORDERS_ENDPOINT,
            reqwest::Method::POST,
            Some(request),
            EndpointType::Trading,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_multiple_orders_request_serialization() {
        let request = PlaceMultipleOrdersRequest {
            data: vec![
                OrderData {
                    symbol: "BTC-USDT".to_string(),
                    side: OrderSide::Buy,
                    order_type: OrderType::Limit,
                    stop_price: None,
                    quantity: Some("0.001".to_string()),
                    quote_order_qty: None,
                    price: Some("50000.0".to_string()),
                    new_client_order_id: Some("order1".to_string()),
                    time_in_force: Some(TimeInForce::Gtc),
                },
                OrderData {
                    symbol: "ETH-USDT".to_string(),
                    side: OrderSide::Sell,
                    order_type: OrderType::Market,
                    stop_price: None,
                    quantity: Some("0.1".to_string()),
                    quote_order_qty: None,
                    price: None,
                    new_client_order_id: Some("order2".to_string()),
                    time_in_force: None,
                },
            ],
            sync: Some(false),
            recv_window: Some(5000),
            timestamp: 1658748648396,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("BTC-USDT"));
        assert!(json.contains("ETH-USDT"));
        assert!(json.contains("\"sync\":false"));
        assert!(json.contains("1658748648396"));
        assert!(json.contains("5000"));
    }

    #[test]
    fn test_place_multiple_orders_response_deserialization() {
        let json = r#"{
            "orders": [
                {
                    "symbol": "BTC-USDT",
                    "orderId": 123456789,
                    "transactTime": 1658748648396,
                    "price": "50000.00",
                    "origQty": "0.001",
                    "executedQty": "0.000",
                    "cummulativeQuoteQty": "0.00",
                    "status": "NEW",
                    "type": "LIMIT",
                    "side": "BUY",
                    "clientOrderID": "order1"
                }
            ]
        }"#;

        let response: PlaceMultipleOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.orders.len(), 1);
        assert_eq!(response.orders[0].symbol, "BTC-USDT");
        assert_eq!(response.orders[0].order_id, 123456789);
        assert_eq!(response.orders[0].status, OrderStatus::New);
    }
}
