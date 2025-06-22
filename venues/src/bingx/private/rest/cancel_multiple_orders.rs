use serde::{Deserialize, Serialize};

use super::RestClient;
use super::place_order::{OrderSide, OrderStatus, OrderType};
use crate::bingx::{EndpointType, RestResult};

/// Cancel multiple orders response item
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelMultipleOrdersResponseItem {
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

    /// Customized order ID for users
    #[serde(rename = "clientOrderID")]
    pub client_order_id: Option<String>,

    /// Trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<f64>,
}

/// Request to cancel multiple orders
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelMultipleOrdersRequest {
    /// Trading pair, e.g., BTC-USDT
    pub symbol: String,

    /// Processing mode: 0 or 1, default 0
    /// If process=1, will handle valid orderIds partially, and return invalid orderIds in fails list
    /// If process=0, if one of orderIds invalid, will all fail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<i32>,

    /// Order IDs: comma-separated list, e.g., "id1,id2,id3"
    #[serde(rename = "orderIds")]
    pub order_ids: String,

    /// Custom order IDs: comma-separated list, e.g., "id1,id2,id3" (optional)
    #[serde(rename = "clientOrderIDs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_ids: Option<String>,

    /// Request valid time window value, Unit: milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
}

/// Response from canceling multiple orders
#[derive(Debug, Clone)]
pub struct CancelMultipleOrdersResponse {
    /// List of canceled orders
    pub orders: Vec<CancelMultipleOrdersResponseItem>,
}

// Custom deserialization since the response is a direct array
impl<'de> Deserialize<'de> for CancelMultipleOrdersResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let orders = Vec::<CancelMultipleOrdersResponseItem>::deserialize(deserializer)?;
        Ok(CancelMultipleOrdersResponse { orders })
    }
}

impl RestClient {
    /// Cancel multiple orders
    ///
    /// Cancels multiple active orders.
    /// Rate limit: 2/s by UID
    ///
    /// # Arguments
    /// * `request` - The cancel multiple orders request
    ///
    /// # Returns
    /// A result containing the canceled orders or an error
    ///
    /// # Notes
    /// - Process mode controls error handling behavior
    /// - Either orderIds or clientOrderIds can be used to identify orders
    pub async fn cancel_multiple_orders(&self, request: &CancelMultipleOrdersRequest) -> RestResult<CancelMultipleOrdersResponse> {
        self.send_request(
            "/openApi/spot/v1/trade/cancelOrders",
            reqwest::Method::POST,
            Some(request),
            EndpointType::Trading,
        )
        .await
    }
}

impl CancelMultipleOrdersRequest {
    /// Create a request to cancel multiple orders by order IDs
    pub fn by_order_ids(symbol: String, order_ids: Vec<i64>) -> Self {
        let order_ids_str = order_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        Self {
            symbol,
            process: None,
            order_ids: order_ids_str,
            client_order_ids: None,
            recv_window: None,
        }
    }

    /// Create a request to cancel multiple orders by client order IDs
    pub fn by_client_order_ids(symbol: String, client_order_ids: Vec<String>, order_ids: Vec<i64>) -> Self {
        let order_ids_str = order_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let client_order_ids_str = client_order_ids.join(",");

        Self {
            symbol,
            process: None,
            order_ids: order_ids_str,
            client_order_ids: Some(client_order_ids_str),
            recv_window: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_multiple_orders_request_serialization() {
        let request = CancelMultipleOrdersRequest {
            symbol: "BTC-USDT".to_string(),
            process: Some(1),
            order_ids: "123456789,123456790,123456791".to_string(),
            client_order_ids: Some("order1,order2,order3".to_string()),
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-USDT"));
        assert!(serialized.contains("process=1"));
        assert!(serialized.contains("orderIds=123456789%2C123456790%2C123456791")); // URL encoded commas
        assert!(serialized.contains("clientOrderIDs=order1%2Corder2%2Corder3"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_cancel_multiple_orders_by_order_ids() {
        let request = CancelMultipleOrdersRequest::by_order_ids(
            "BTC-USDT".to_string(),
            vec![123456789, 123456790, 123456791],
        );

        assert_eq!(request.symbol, "BTC-USDT");
        assert_eq!(request.order_ids, "123456789,123456790,123456791");
        assert!(request.client_order_ids.is_none());
        assert!(request.process.is_none());
    }

    #[test]
    fn test_cancel_multiple_orders_by_client_order_ids() {
        let request = CancelMultipleOrdersRequest::by_client_order_ids(
            "BTC-USDT".to_string(),
            vec![
                "order1".to_string(),
                "order2".to_string(),
                "order3".to_string(),
            ],
            vec![123456789, 123456790, 123456791],
        );

        assert_eq!(request.symbol, "BTC-USDT");
        assert_eq!(request.order_ids, "123456789,123456790,123456791");
        assert_eq!(
            request.client_order_ids,
            Some("order1,order2,order3".to_string())
        );
        assert!(request.process.is_none());
    }

    #[test]
    fn test_cancel_multiple_orders_response_deserialization() {
        let json = r#"[
            {
                "symbol": "BTC-USDT",
                "orderId": 123456789,
                "price": "50000.0",
                "origQty": "0.001",
                "executedQty": "0.0005",
                "cummulativeQuoteQty": "25.0",
                "status": "CANCELED",
                "type": "LIMIT",
                "side": "BUY",
                "clientOrderID": "order1",
                "stopPrice": 49000.0
            },
            {
                "symbol": "BTC-USDT",
                "orderId": 123456790,
                "price": "50100.0",
                "origQty": "0.002",
                "executedQty": "0.0",
                "cummulativeQuoteQty": "0.0",
                "status": "CANCELED",
                "type": "LIMIT",
                "side": "SELL",
                "clientOrderID": "order2"
            }
        ]"#;

        let response: CancelMultipleOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.orders.len(), 2);

        let first_order = response.orders.get(0).expect("Expected at least one order");
        assert_eq!(first_order.symbol, "BTC-USDT");
        assert_eq!(first_order.order_id, 123456789);
        assert_eq!(first_order.price, "50000.0");
        assert!(matches!(first_order.status, OrderStatus::Canceled));
        assert!(matches!(first_order.order_type, OrderType::Limit));
        assert!(matches!(first_order.side, OrderSide::Buy));
        assert_eq!(first_order.client_order_id, Some("order1".to_string()));
        assert_eq!(first_order.stop_price, Some(49000.0));

        let second_order = response.orders.get(1).expect("Expected at least two orders");
        assert_eq!(second_order.symbol, "BTC-USDT");
        assert_eq!(second_order.order_id, 123456790);
        assert!(matches!(second_order.status, OrderStatus::Canceled));
        assert!(matches!(second_order.side, OrderSide::Sell));
        assert_eq!(second_order.stop_price, None);
    }
}
