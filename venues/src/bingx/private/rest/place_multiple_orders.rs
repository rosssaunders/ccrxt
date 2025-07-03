use serde::{Deserialize, Serialize};

use crate::bingx::{
    BingXRestClient,
    enums::{OrderSide, OrderStatus, OrderType, TimeInForce},
    errors::BingXError,
};

/// Request for placing multiple orders in batch
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceMultipleOrdersRequest {
    /// Array of order data, limited to 5 orders
    pub data: Vec<OrderData>,
    /// Sync mode: false (default) = parallel ordering, true = serial ordering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync: Option<bool>,
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
    pub quantity: Option<f64>,
    /// Quote order quantity (order amount)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_order_qty: Option<f64>,
    /// Order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    /// Custom order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,
    /// Time in force
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    /// Request valid time window value in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
    /// Request timestamp in milliseconds
    pub timestamp: u64,
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

impl BingXRestClient {
    /// Place multiple orders in batch
    ///
    /// # Arguments
    /// * `request` - The batch order request
    ///
    /// # Returns
    /// * `Result<PlaceMultipleOrdersResponse, BingXError>` - The batch order response or error
    pub async fn place_multiple_orders(
        &self,
        request: PlaceMultipleOrdersRequest,
    ) -> Result<PlaceMultipleOrdersResponse, BingXError> {
        self.send_signed_request("POST", "/openApi/spot/v1/trade/batchOrders", Some(&request))
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
                    quantity: Some(0.001),
                    quote_order_qty: None,
                    price: Some(50000.0),
                    new_client_order_id: Some("order1".to_string()),
                    time_in_force: Some(TimeInForce::Gtc),
                    recv_window: None,
                    timestamp: 1658748648396,
                },
                OrderData {
                    symbol: "ETH-USDT".to_string(),
                    side: OrderSide::Sell,
                    order_type: OrderType::Market,
                    stop_price: None,
                    quantity: Some(0.1),
                    quote_order_qty: None,
                    price: None,
                    new_client_order_id: Some("order2".to_string()),
                    time_in_force: None,
                    recv_window: None,
                    timestamp: 1658748648396,
                },
            ],
            sync: Some(false),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("BTC-USDT"));
        assert!(json.contains("ETH-USDT"));
        assert!(json.contains("\"sync\":false"));
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
