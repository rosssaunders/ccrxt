use serde::{Deserialize, Serialize};

use super::{
    RestClient,
    place_order::{OrderSide, OrderStatus, OrderType},
};
use crate::bingx::{EndpointType, RestResult};

const OPEN_ORDERS_ENDPOINT: &str = "/openApi/spot/v1/trade/openOrders";

/// Order information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
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

    /// Trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,
}

/// Request to get open orders
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenOrdersRequest {
    /// Trading pair, e.g., BTC-USDT (optional - query all when left blank)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Request valid time window value, Unit: milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Timestamp of initiating the request, Unit: milliseconds
    pub timestamp: i64,
}

/// Response from getting open orders
#[derive(Debug, Clone, Deserialize)]
pub struct GetOpenOrdersResponse {
    /// Order list (max length is 2000)
    pub orders: Vec<Order>,
}

impl RestClient {
    /// Get current open orders
    ///
    /// Retrieves all current open orders for the account.
    /// Rate limit: 10/s by UID
    ///
    /// # Arguments
    /// * `request` - The get open orders request (symbol is optional)
    ///
    /// # Returns
    /// A result containing the open orders or an error
    pub async fn get_open_orders(
        &self,
        request: &GetOpenOrdersRequest,
    ) -> RestResult<GetOpenOrdersResponse> {
        self.send_request(
            OPEN_ORDERS_ENDPOINT,
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
    fn test_get_open_orders_request_serialization_all() {
        let request = GetOpenOrdersRequest {
            symbol: None,
            recv_window: None,
            timestamp: 1658748648396,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("timestamp=1658748648396"));
        assert!(!serialized.contains("symbol"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_get_open_orders_request_serialization_with_symbol() {
        let request = GetOpenOrdersRequest {
            symbol: Some("BTC-USDT".to_string()),
            recv_window: Some(5000),
            timestamp: 1658748648396,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-USDT"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1658748648396"));
    }

    #[test]
    fn test_get_open_orders_response_deserialization() {
        let json = r#"{
            "orders": [
                {
                    "symbol": "BTC-USDT",
                    "orderId": 123456789,
                    "price": "50000.0",
                    "origQty": "0.001",
                    "executedQty": "0.0005",
                    "cummulativeQuoteQty": "25.0",
                    "status": "PARTIALLY_FILLED",
                    "type": "LIMIT",
                    "side": "BUY",
                    "time": 1658748648396,
                    "updateTime": 1658748650000,
                    "origQuoteOrderQty": "50.0",
                    "stopPrice": "49000.0"
                },
                {
                    "symbol": "ETH-USDT",
                    "orderId": 987654321,
                    "price": "3000.0",
                    "origQty": "0.1",
                    "executedQty": "0.0",
                    "cummulativeQuoteQty": "0.0",
                    "status": "NEW",
                    "type": "LIMIT",
                    "side": "SELL",
                    "time": 1658748648400,
                    "updateTime": 1658748648400,
                    "origQuoteOrderQty": "300.0"
                }
            ]
        }"#;

        let response: GetOpenOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.orders.len(), 2);

        let btc_order = &response.orders[0];
        assert_eq!(btc_order.symbol, "BTC-USDT");
        assert_eq!(btc_order.order_id, 123456789);
        assert_eq!(btc_order.price, "50000.0");
        assert_eq!(btc_order.orig_qty, "0.001");
        assert_eq!(btc_order.executed_qty, "0.0005");
        assert_eq!(btc_order.cummulative_quote_qty, "25.0");
        assert!(matches!(btc_order.status, OrderStatus::PartiallyFilled));
        assert!(matches!(btc_order.order_type, OrderType::Limit));
        assert!(matches!(btc_order.side, OrderSide::Buy));
        assert_eq!(btc_order.time, 1658748648396);
        assert_eq!(btc_order.update_time, 1658748650000);
        assert_eq!(btc_order.orig_quote_order_qty, "50.0");
        assert_eq!(btc_order.stop_price, Some("49000.0".to_string()));

        let eth_order = &response.orders[1];
        assert_eq!(eth_order.symbol, "ETH-USDT");
        assert_eq!(eth_order.order_id, 987654321);
        assert!(matches!(eth_order.status, OrderStatus::New));
        assert!(matches!(eth_order.side, OrderSide::Sell));
        assert_eq!(eth_order.stop_price, None);
    }

    #[test]
    fn test_order_deserialization() {
        let json = r#"{
            "symbol": "BTC-USDT",
            "orderId": 123456789,
            "price": "50000.0",
            "origQty": "0.001",
            "executedQty": "0.0005",
            "cummulativeQuoteQty": "25.0",
            "status": "PARTIALLY_FILLED",
            "type": "LIMIT",
            "side": "BUY",
            "time": 1658748648396,
            "updateTime": 1658748650000,
            "origQuoteOrderQty": "50.0"
        }"#;

        let order: Order = serde_json::from_str(json).unwrap();
        assert_eq!(order.symbol, "BTC-USDT");
        assert_eq!(order.order_id, 123456789);
        assert!(matches!(order.status, OrderStatus::PartiallyFilled));
        assert!(matches!(order.order_type, OrderType::Limit));
        assert!(matches!(order.side, OrderSide::Buy));
        assert_eq!(order.stop_price, None);
    }
}
