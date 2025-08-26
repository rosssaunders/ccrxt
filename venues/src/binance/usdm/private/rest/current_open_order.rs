use serde::{Deserialize, Serialize};

use crate::binance::usdm::PrivateRestClient as RestClient;
use crate::binance::usdm::{RestResult, enums::*};

const CURRENT_OPEN_ORDER_ENDPOINT: &str = "/fapi/v1/openOrder";

/// Request parameters for the Query Current Open Order endpoint.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CurrentOpenOrderRequest {
    /// Trading symbol (e.g., "BTCUSDT").
    ///
    /// This field is required.
    pub symbol: String,

    /// Order ID to query. Either `order_id` or `orig_client_order_id` must be sent.
    ///
    /// If both are provided, `order_id` is used.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID. Either `order_id` or `orig_client_order_id` must be sent.
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,

    /// The value cannot be greater than 60000
    ///
    /// Optional. Default: 5000.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp (milliseconds since epoch).
    pub timestamp: u64,
}

/// Response for the Query Current Open Order endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentOpenOrderResponse {
    /// Trading symbol.
    pub symbol: String,

    /// Order ID.
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Client order ID.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Price.
    pub price: String,

    /// Average price.
    pub avg_price: String,

    /// Original quantity.
    #[serde(rename = "origQty")]
    pub orig_qty: String,

    /// Executed quantity.
    #[serde(rename = "executedQty")]
    pub executed_qty: String,

    /// Cumulative quote asset transacted quantity.
    pub cum_quote: String,

    /// Status of the order.
    pub status: OrderStatus,

    /// Time in force policy for the order.
    pub time_in_force: TimeInForce,

    /// Order type.
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Order side.
    pub side: OrderSide,

    /// Position side.
    pub position_side: PositionSide,

    /// Reduce only flag.
    pub reduce_only: bool,

    /// Close position flag.
    pub close_position: bool,

    /// Stop price (if applicable).
    pub stop_price: Option<String>,

    /// Working type.
    pub working_type: WorkingType,

    /// Price protect flag.
    pub price_protect: bool,

    /// Original order type.
    pub orig_type: OrderType,

    /// Price match mode.
    pub price_match: PriceMatch,

    /// Self trade prevention mode.
    pub self_trade_prevention_mode: SelfTradePreventionMode,

    /// Good till date (for GTD orders).
    pub good_till_date: u64,

    /// Order time (milliseconds since epoch).
    pub time: u64,

    /// Update time (milliseconds since epoch).
    pub update_time: u64,

    /// Activation price (for trailing stop orders).
    pub activate_price: Option<String>,

    /// Price rate (for trailing stop orders).
    pub price_rate: Option<String>,
}

impl RestClient {
    /// Query Current Open Order (USER_DATA)
    ///
    /// Check an order's status. Either `orderId` or `origClientOrderId` must be sent.
    /// If the queried order has been filled or cancelled, the error message "Order does not exist" will be returned.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Query-Current-Open-Order)
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `request` - The query current open order request parameters
    ///
    /// # Returns
    /// Current open order details if found
    pub async fn query_current_open_order(
        &self,
        request: CurrentOpenOrderRequest,
    ) -> RestResult<CurrentOpenOrderResponse> {
        self.send_get_signed_request(CURRENT_OPEN_ORDER_ENDPOINT, request, 1, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_open_order_request_with_order_id() {
        let request = CurrentOpenOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some(12345),
            orig_client_order_id: None,
            recv_window: Some(5000),
            timestamp: 1234567890,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("orderId=12345"));
        assert!(!serialized.contains("origClientOrderId"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1234567890"));
    }

    #[test]
    fn test_current_open_order_request_with_client_order_id() {
        let request = CurrentOpenOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: None,
            orig_client_order_id: Some("myOrder123".to_string()),
            recv_window: None,
            timestamp: 9876543210,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("origClientOrderId=myOrder123"));
        assert!(!serialized.contains("orderId"));
        assert!(serialized.contains("timestamp=9876543210"));
    }

    #[test]
    fn test_current_open_order_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 12345,
            "clientOrderId": "myOrder1",
            "price": "50000.0",
            "avgPrice": "0.0",
            "origQty": "1.0",
            "executedQty": "0.5",
            "cumQuote": "25000.0",
            "status": "PARTIALLY_FILLED",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "positionSide": "BOTH",
            "reduceOnly": false,
            "closePosition": false,
            "stopPrice": null,
            "workingType": "CONTRACT_PRICE",
            "priceProtect": false,
            "origType": "LIMIT",
            "priceMatch": "NONE",
            "selfTradePreventionMode": "NONE",
            "goodTillDate": 0,
            "time": 1641038400000,
            "updateTime": 1641038400000,
            "activatePrice": null,
            "priceRate": null
        }"#;

        let response: CurrentOpenOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.order_id, 12345);
        assert_eq!(response.status, OrderStatus::PartiallyFilled);
        assert_eq!(response.side, OrderSide::Buy);
        assert_eq!(response.position_side, PositionSide::Both);
        assert!(!response.reduce_only);
        assert!(!response.close_position);
        assert_eq!(response.avg_price, "0.0");
    }

    #[test]
    fn test_current_open_order_request_default() {
        let request = CurrentOpenOrderRequest::default();
        assert!(request.symbol.is_empty());
        assert!(request.order_id.is_none());
        assert!(request.orig_client_order_id.is_none());
        assert!(request.recv_window.is_none());
        assert_eq!(request.timestamp, 0);
    }

    #[test]
    fn test_current_open_order_request_with_both_ids() {
        let request = CurrentOpenOrderRequest {
            symbol: "ETHUSDT".to_string(),
            order_id: Some(67890),
            orig_client_order_id: Some("clientOrder456".to_string()),
            recv_window: None,
            timestamp: 1111111111,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSDT"));
        assert!(serialized.contains("orderId=67890"));
        assert!(serialized.contains("origClientOrderId=clientOrder456"));
        assert!(serialized.contains("timestamp=1111111111"));
    }
}
