use serde::{Deserialize, Serialize};

use crate::bingx::{
    EndpointType, PrivateRestClient as RestClient, RestResult,
    enums::{OrderSide, OrderStatus, OrderType},
};

const CANCEL_MULTIPLE_ORDERS_ENDPOINT: &str = "/openApi/spot/v1/trade/cancelOrders";

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

    /// Order status: NEW, PENDING, PARTIALLY_FILLED, FILLED, CANCELED, FAILED
    pub status: OrderStatus,

    /// Order type: MARKET/LIMIT/TAKE_STOP_LIMIT/TAKE_STOP_MARKET/TRIGGER_LIMIT/TRIGGER_MARKET
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Order side: BUY/SELL
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
    /// Trading pair, e.g., BTC-USDT (required)
    pub symbol: String,

    /// Processing mode: 0 or 1, default 0. If process=1, will handle valid orderIds partially and return invalid orderIds in fails list. If process=0, if one of orderIds invalid, will all fail (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<i32>,

    /// Order Ids: for example:orderIds=id1,id2,id3 (required)
    #[serde(rename = "orderIds")]
    pub order_ids: String,

    /// Custom order IDs, for example: clientOrderIDs=id1,id2,id3 (optional)
    #[serde(rename = "clientOrderIDs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_ids: Option<String>,

    /// Request valid time window value in milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Timestamp of initiating the request in milliseconds (required)
    pub timestamp: i64,
}

/// Response from canceling multiple orders
#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct CancelMultipleOrdersResponse {
    /// List of canceled orders
    pub orders: Vec<CancelMultipleOrdersResponseItem>,
}

impl RestClient {
    /// Cancel multiple orders
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/spot/trade-api.html#Cancel%20multiple%20orders)
    ///
    /// Rate limit: UID 2/s & IP rate limit group 3
    ///
    /// # Arguments
    /// * `request` - The cancel multiple orders request
    ///
    /// # Returns
    /// A result containing the canceled orders or an error
    pub async fn cancel_multiple_orders(
        &self,
        request: &CancelMultipleOrdersRequest,
    ) -> RestResult<CancelMultipleOrdersResponse> {
        self.send_post_signed_request(
            CANCEL_MULTIPLE_ORDERS_ENDPOINT,
            request,
            EndpointType::Trading,
        )
        .await
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
            timestamp: 1658748648396,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-USDT"));
        assert!(serialized.contains("process=1"));
        assert!(serialized.contains("orderIds=123456789%2C123456790%2C123456791")); // URL encoded commas
        assert!(serialized.contains("clientOrderIDs=order1%2Corder2%2Corder3"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1658748648396"));
    }

    #[test]
    fn test_cancel_multiple_orders_by_order_ids() {
        let order_ids = [123456789, 123456790, 123456791];
        let order_ids_str = order_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let request = CancelMultipleOrdersRequest {
            symbol: "BTC-USDT".to_string(),
            process: None,
            order_ids: order_ids_str,
            client_order_ids: None,
            recv_window: None,
            timestamp: 1658748648396,
        };

        assert_eq!(request.symbol, "BTC-USDT");
        assert_eq!(request.order_ids, "123456789,123456790,123456791");
        assert!(request.client_order_ids.is_none());
        assert!(request.process.is_none());
    }

    #[test]
    fn test_cancel_multiple_orders_by_client_order_ids() {
        let client_order_ids = [
            "order1".to_string(),
            "order2".to_string(),
            "order3".to_string(),
        ];
        let order_ids = [123456789, 123456790, 123456791];

        let order_ids_str = order_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let client_order_ids_str = client_order_ids.join(",");

        let request = CancelMultipleOrdersRequest {
            symbol: "BTC-USDT".to_string(),
            process: None,
            order_ids: order_ids_str,
            client_order_ids: Some(client_order_ids_str),
            recv_window: None,
            timestamp: 1658748648396,
        };

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

        let first_order = response
            .orders
            .first()
            .expect("Expected at least one order");
        assert_eq!(first_order.symbol, "BTC-USDT");
        assert_eq!(first_order.order_id, 123456789);
        assert_eq!(first_order.price, "50000.0");
        assert!(matches!(first_order.status, OrderStatus::Canceled));
        assert!(matches!(first_order.order_type, OrderType::Limit));
        assert!(matches!(first_order.side, OrderSide::Buy));
        assert_eq!(first_order.client_order_id, Some("order1".to_string()));
        assert_eq!(first_order.stop_price, Some(49000.0));

        let second_order = response
            .orders
            .get(1)
            .expect("Expected at least two orders");
        assert_eq!(second_order.symbol, "BTC-USDT");
        assert_eq!(second_order.order_id, 123456790);
        assert!(matches!(second_order.status, OrderStatus::Canceled));
        assert!(matches!(second_order.side, OrderSide::Sell));
        assert_eq!(second_order.stop_price, None);
    }
}
