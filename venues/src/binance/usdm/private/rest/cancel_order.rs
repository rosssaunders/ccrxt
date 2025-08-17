use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::{
    RestResult,
    enums::{OrderSide, OrderStatus, OrderType, PositionSide, TimeInForce},
};

/// Request parameters for the cancel order endpoint.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    /// Trading symbol (e.g., "BTCUSDT").
    ///
    /// Required. Must be a valid symbol listed on Binance USDM Futures.
    pub symbol: Cow<'static, str>,

    /// Order ID to cancel.
    ///
    /// Either this or `orig_client_order_id` must be provided. If both are provided, `order_id` is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID to cancel.
    ///
    /// Either this or `order_id` must be provided. If both are provided, `order_id` is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<Cow<'static, str>>,

    /// The value cannot be greater than 60000 (milliseconds).
    ///
    /// Optional. The number of milliseconds after timestamp the request is valid for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp (milliseconds since epoch).
    ///
    /// Required. Must be the current server time in milliseconds.
    pub timestamp: u64,
}

/// Response containing details of the cancelled order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    /// Trading symbol.
    pub symbol: Cow<'static, str>,

    /// Order ID.
    pub order_id: u64,

    /// Client order ID.
    pub client_order_id: Cow<'static, str>,

    /// Price (as string for precision).
    pub price: Cow<'static, str>,

    /// Original quantity (as string for precision).
    pub orig_qty: Cow<'static, str>,

    /// Executed quantity (as string for precision).
    pub executed_qty: Cow<'static, str>,

    /// Cumulative quote asset transacted quantity (as string).
    pub cum_quote: Cow<'static, str>,

    /// Order status.
    pub status: OrderStatus,

    /// Time in force.
    pub time_in_force: TimeInForce,

    /// Order type.
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Order side.
    pub side: OrderSide,

    /// Position side.
    pub position_side: PositionSide,

    /// Update time (milliseconds since epoch).
    pub update_time: u64,
}

/// Endpoint path for cancelling an order.
const CANCEL_ORDER_ENDPOINT: &str = "/fapi/v1/order";

impl UsdmClient {
    /// Cancel Order
    ///
    /// Cancels an existing order on Binance USDM Futures.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Cancel-Order)
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `request` - The cancel order request parameters
    ///
    /// # Returns
    /// Response containing cancelled order details
    pub async fn cancel_order(
        &self,
        request: CancelOrderRequest,
    ) -> RestResult<CancelOrderResponse> {
        self.send_delete_signed_request(CANCEL_ORDER_ENDPOINT, request, 1, false)
            .await
    }
}
#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_cancel_order_request_default() {
        let req = CancelOrderRequest::default();
        // Default instance should have empty symbol, None for IDs, None for recv_window, and 0 timestamp
        assert_eq!(req.symbol, Cow::Borrowed(""));
        assert!(req.order_id.is_none());
        assert!(req.orig_client_order_id.is_none());
        assert!(req.recv_window.is_none());
        assert_eq!(req.timestamp, 0);
    }

    #[test]
    fn test_cancel_order_response_deserialization() {
        let data = json!({
            "symbol": "BTCUSDT",
            "orderId": 123456,
            "clientOrderId": "myorder123",
            "price": "0.0",
            "origQty": "1.0",
            "executedQty": "0.0",
            "cumQuote": "0.0",
            "status": "CANCELED",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "SELL",
            "positionSide": "BOTH",
            "updateTime": 1620000000000u64
        });
        let resp: CancelOrderResponse = serde_json::from_value(data).unwrap();
        assert_eq!(resp.symbol, "BTCUSDT");
        assert_eq!(resp.order_id, 123456);
        assert_eq!(resp.client_order_id, "myorder123");
        assert_eq!(resp.price, "0.0");
        assert_eq!(resp.orig_qty, "1.0");
        assert_eq!(resp.executed_qty, "0.0");
        assert_eq!(resp.cum_quote, "0.0");
        assert_eq!(resp.status, OrderStatus::Canceled);
        assert_eq!(resp.time_in_force, TimeInForce::GTC);
        assert_eq!(resp.order_type, OrderType::Limit);
        assert_eq!(resp.side, OrderSide::Sell);
        assert_eq!(resp.position_side, PositionSide::Both);
        assert_eq!(resp.update_time, 1620000000000u64);
    }
}
