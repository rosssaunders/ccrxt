use serde::{Deserialize, Serialize};

use super::{
    RestClient,
    place_order::{OrderSide, OrderStatus, OrderType},
};
use crate::bingx::spot::{EndpointType, RestResult};

const ORDER_HISTORY_ENDPOINT: &str = "/openApi/spot/v1/trade/historyOrders";

/// Historical order information (similar to Order but with fee as f64)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalOrder {
    /// Trading pair
    pub symbol: String,

    /// Order ID
    pub order_id: i64,

    /// Price
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

    /// Order timestamp
    pub time: i64,

    /// Update timestamp
    pub update_time: i64,

    /// Original quote order quantity
    pub orig_quote_order_qty: String,

    /// Fee
    pub fee: f64,

    /// Trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,

    /// Average fill price
    pub avg_price: String,
}

/// Request to get order history
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderHistoryRequest {
    /// Trading pair, e.g., BTC-USDT (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// If orderId is set, orders >= orderId. Otherwise, the most recent orders will be returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,

    /// Start timestamp, Unit: ms
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,

    /// End timestamp, Unit: ms
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,

    /// Page number, must >0, defaults to 1. Restriction: pageIndex * pageSize <= 10,000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_index: Option<i64>,

    /// Page size, must >0, max 100, defaults to 100. Restriction: pageIndex * pageSize <= 10,000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,

    /// Order status filter: FILLED, CANCELED, FAILED
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,

    /// Order type filter
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<OrderType>,

    /// Request valid time window value, Unit: milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Timestamp of initiating the request, Unit: milliseconds
    pub timestamp: i64,
}

/// Response from getting order history
#[derive(Debug, Clone, Deserialize)]
pub struct GetOrderHistoryResponse {
    /// Order list (max length is 2000)
    pub orders: Vec<HistoricalOrder>,
}

impl RestClient {
    /// Get order history
    ///
    /// Retrieves historical orders for the account.
    /// Rate limit: 10/s by UID
    ///
    /// # Arguments
    /// * `request` - The get order history request with optional filters
    ///
    /// # Returns
    /// A result containing the order history or an error
    ///
    /// # Notes
    /// - If orderId is set, orders >= orderId will be returned
    /// - If startTime and endTime are provided, orderId is not required
    /// - Max page size is 100, and pageIndex * pageSize <= 10,000
    ///
    /// # API Documentation
    /// - [docs]: https://bingx-api.github.io/docs/#/en-us/spot/trade-api.html#Query%20Order%20history
    pub async fn get_order_history(
        &self,
        request: &GetOrderHistoryRequest,
    ) -> RestResult<GetOrderHistoryResponse> {
        self.send_request(
            ORDER_HISTORY_ENDPOINT,
            reqwest::Method::GET,
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
    fn test_get_order_history_request_serialization_minimal() {
        let request = GetOrderHistoryRequest {
            symbol: None,
            order_id: None,
            start_time: None,
            end_time: None,
            page_index: None,
            page_size: None,
            status: None,
            order_type: None,
            recv_window: None,
            timestamp: 1640995200000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("timestamp=1640995200000"));
    }

    #[test]
    fn test_get_order_history_request_serialization_with_filters() {
        let request = GetOrderHistoryRequest {
            symbol: Some("BTC-USDT".to_string()),
            start_time: Some(1658748648000),
            end_time: Some(1658748648400),
            page_index: Some(1),
            page_size: Some(50),
            status: Some(OrderStatus::Filled),
            order_type: Some(OrderType::Limit),
            recv_window: Some(5000),
            order_id: None,
            timestamp: 1658748648396,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-USDT"));
        assert!(serialized.contains("startTime=1658748648000"));
        assert!(serialized.contains("endTime=1658748648400"));
        assert!(serialized.contains("pageIndex=1"));
        assert!(serialized.contains("pageSize=50"));
        assert!(serialized.contains("status=FILLED"));
        assert!(serialized.contains("type=LIMIT"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(!serialized.contains("orderId"));
    }

    #[test]
    fn test_get_order_history_response_deserialization() {
        let json = r#"{
            "orders": [
                {
                    "symbol": "BTC-USDT",
                    "orderId": 123456789,
                    "price": "50000.0",
                    "origQty": "0.001",
                    "executedQty": "0.001",
                    "cummulativeQuoteQty": "50.0",
                    "status": "FILLED",
                    "type": "LIMIT",
                    "side": "BUY",
                    "time": 1658748648396,
                    "updateTime": 1658748650000,
                    "origQuoteOrderQty": "50.0",
                    "fee": 0.05,
                    "stopPrice": "49000.0",
                    "avgPrice": "50000.0"
                },
                {
                    "symbol": "ETH-USDT",
                    "orderId": 987654321,
                    "price": "3000.0",
                    "origQty": "0.1",
                    "executedQty": "0.05",
                    "cummulativeQuoteQty": "150.0",
                    "status": "CANCELED",
                    "type": "LIMIT",
                    "side": "SELL",
                    "time": 1658748648400,
                    "updateTime": 1658748648500,
                    "origQuoteOrderQty": "300.0",
                    "fee": 0.0,
                    "avgPrice": "3000.0"
                }
            ]
        }"#;

        let response: GetOrderHistoryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.orders.len(), 2);

        let btc_order = &response.orders[0];
        assert_eq!(btc_order.symbol, "BTC-USDT");
        assert_eq!(btc_order.order_id, 123456789);
        assert_eq!(btc_order.price, "50000.0");
        assert_eq!(btc_order.orig_qty, "0.001");
        assert_eq!(btc_order.executed_qty, "0.001");
        assert_eq!(btc_order.cummulative_quote_qty, "50.0");
        assert!(matches!(btc_order.status, OrderStatus::Filled));
        assert!(matches!(btc_order.order_type, OrderType::Limit));
        assert!(matches!(btc_order.side, OrderSide::Buy));
        assert_eq!(btc_order.time, 1658748648396);
        assert_eq!(btc_order.update_time, 1658748650000);
        assert_eq!(btc_order.orig_quote_order_qty, "50.0");
        assert_eq!(btc_order.fee, 0.05);
        assert_eq!(btc_order.stop_price, Some("49000.0".to_string()));
        assert_eq!(btc_order.avg_price, "50000.0");

        let eth_order = &response.orders[1];
        assert_eq!(eth_order.symbol, "ETH-USDT");
        assert_eq!(eth_order.order_id, 987654321);
        assert!(matches!(eth_order.status, OrderStatus::Canceled));
        assert!(matches!(eth_order.side, OrderSide::Sell));
        assert_eq!(eth_order.fee, 0.0);
        assert_eq!(eth_order.stop_price, None);
    }

    #[test]
    fn test_historical_order_deserialization() {
        let json = r#"{
            "symbol": "BTC-USDT",
            "orderId": 123456789,
            "price": "50000.0",
            "origQty": "0.001",
            "executedQty": "0.001",
            "cummulativeQuoteQty": "50.0",
            "status": "FILLED",
            "type": "LIMIT",
            "side": "BUY",
            "time": 1658748648396,
            "updateTime": 1658748650000,
            "origQuoteOrderQty": "50.0",
            "fee": 0.05,
            "avgPrice": "50000.0"
        }"#;

        let order: HistoricalOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.symbol, "BTC-USDT");
        assert_eq!(order.order_id, 123456789);
        assert!(matches!(order.status, OrderStatus::Filled));
        assert!(matches!(order.order_type, OrderType::Limit));
        assert!(matches!(order.side, OrderSide::Buy));
        assert_eq!(order.fee, 0.05);
        assert_eq!(order.stop_price, None);
    }
}
