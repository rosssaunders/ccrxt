use serde::{Deserialize, Serialize};

use crate::bingx::{
    EndpointType, PrivateRestClient as RestClient, RestResult,
    enums::{OrderSide, OrderStatus, OrderType},
};

/// Cancel all orders endpoint URL
const CANCEL_ALL_ORDERS_ENDPOINT: &str = "/openApi/spot/v1/trade/cancelOpenOrders";

/// Request for canceling all open orders on a symbol
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersRequest {
    /// Trading pair, e.g., BTC-USDT. If not filled out, cancel all orders (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Request valid time window value in milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp of initiating the request in milliseconds (required)
    pub timestamp: u64,
}

/// Response for canceled orders
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersResponse {
    /// Array of canceled orders
    pub orders: Vec<CanceledOrderInfo>,
}

/// Information about a canceled order
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanceledOrderInfo {
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
    pub client_order_id: String,

    /// Trigger price
    pub stop_price: String,
}

impl RestClient {
    /// Cancel all Open Orders on a Symbol
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/spot/trade-api.html#Cancel%20all%20Open%20Orders%20on%20a%20Symbol)
    ///
    /// Rate limit: UID 2/s & IP rate limit group 3
    ///
    /// # Arguments
    /// * `request` - The cancel all orders request
    ///
    /// # Returns
    /// * `RestResult<CancelAllOrdersResponse>` - The canceled orders response or error
    pub async fn cancel_all_orders(
        &self,
        request: &CancelAllOrdersRequest,
    ) -> RestResult<CancelAllOrdersResponse> {
        self.send_post_signed_request(CANCEL_ALL_ORDERS_ENDPOINT, request, EndpointType::Trading)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_all_orders_request_serialization() {
        let request = CancelAllOrdersRequest {
            symbol: Some("BTC-USDT".to_string()),
            recv_window: Some(5000),
            timestamp: 1658748648396,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("BTC-USDT"));
        assert!(json.contains("1658748648396"));
    }

    #[test]
    fn test_cancel_all_orders_request_serialization_no_symbol() {
        let request = CancelAllOrdersRequest {
            symbol: None,
            recv_window: None,
            timestamp: 1658748648396,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(!json.contains("symbol"));
        assert!(json.contains("1658748648396"));
    }

    #[test]
    fn test_cancel_all_orders_response_deserialization() {
        let json = r#"{
            "orders": [
                {
                    "symbol": "BTC-USDT",
                    "orderId": 123456789,
                    "price": "50000.00",
                    "origQty": "0.001",
                    "executedQty": "0.000",
                    "cummulativeQuoteQty": "0.00",
                    "status": "CANCELED",
                    "type": "LIMIT",
                    "side": "BUY",
                    "clientOrderID": "order1",
                    "stopPrice": "0.00"
                }
            ]
        }"#;

        let response: CancelAllOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.orders.len(), 1);
        assert_eq!(response.orders[0].symbol, "BTC-USDT");
        assert_eq!(response.orders[0].order_id, 123456789);
        assert_eq!(response.orders[0].status, OrderStatus::Canceled);
    }
}
