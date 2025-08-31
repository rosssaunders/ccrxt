use serde::{Deserialize, Serialize};

use crate::bingx::{
    EndpointType, PrivateRestClient as RestClient, RestResult,
    enums::{CancelReplaceMode, CancelRestriction, OrderSide, OrderStatus, OrderType},
};

/// Cancel replace order endpoint URL
const CANCEL_REPLACE_ORDER_ENDPOINT: &str = "/openApi/spot/v1/trade/order/cancelReplace";

/// Request for canceling an existing order and placing a new one
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelReplaceOrderRequest {
    /// The trading pair, for example: BTC-USDT, please use uppercase letters (required)
    pub symbol: String,

    /// The ID of the order to be canceled (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_order_id: Option<i64>,

    /// The user-defined ID of the order to be canceled, character length limit: 1-40, different orders cannot use the same clientOrderID, only supports a query range of 2 hours (optional)
    #[serde(rename = "cancelClientOrderID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_client_order_id: Option<String>,

    /// Cancel orders with specified status: NEW: New order, PENDING: Pending order, PARTIALLY_FILLED: Partially filled (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_restrictions: Option<CancelRestriction>,

    /// STOP_ON_FAILURE: If the cancel order fails, it will not continue to place a new order. ALLOW_FAILURE: Regardless of whether the cancel order succeeds or fails, it will continue to place a new order (required)
    #[serde(rename = "CancelReplaceMode")]
    pub cancel_replace_mode: CancelReplaceMode,

    /// The type of transaction, BUY: Buy, SELL: Sell (required)
    pub side: OrderSide,

    /// Order type: MARKET/LIMIT/TAKE_STOP_LIMIT/TAKE_STOP_MARKET/TRIGGER_LIMIT/TRIGGER_MARKET (required)
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Trigger price used for TAKE_STOP_LIMIT, TAKE_STOP_MARKET, TRIGGER_LIMIT, TRIGGER_MARKET order types (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,

    /// Order quantity, e.g. 0.1BTC (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,

    /// Order amount, e.g. 100USDT (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_order_qty: Option<f64>,

    /// Order price, e.g. 10000USDT (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    /// Custom order ID consisting of letters, numbers, and _. Character length should be between 1-40. Different orders cannot use the same newClientOrderId (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,

    /// Request valid time window in milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds (required)
    pub timestamp: u64,
}

/// Response for cancel and replace order
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelReplaceOrderResponse {
    /// Trading symbol
    pub symbol: String,

    /// Order ID
    pub order_id: i64,

    /// Order price
    pub price: String,

    /// Order quantity
    pub orig_qty: String,

    /// Executed quantity
    pub executed_qty: String,

    /// Cumulative quote quantity
    pub cummulative_quote_qty: String,

    /// Order status: NEW (new order), PENDING (pending), PARTIALLY_FILLED (partially filled), FILLED (filled), CANCELED (cancelled), FAILED (failed)
    pub status: OrderStatus,

    /// Order type: MARKET/LIMIT/TAKE_STOP_LIMIT/TAKE_STOP_MARKET/TRIGGER_LIMIT/TRIGGER_MARKET
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Transaction type: BUY (buy), SELL (sell)
    pub side: OrderSide,

    /// User-defined order ID
    #[serde(rename = "clientOrderID")]
    pub client_order_id: String,

    /// Trigger price
    pub stop_price: String,

    /// Cancel orders in specific states: NEW (new order), PENDING (pending), PARTIALLY_FILLED (partially filled)
    pub cancel_restrictions: Option<CancelRestriction>,

    /// Transaction timestamp
    pub transact_time: i64,
}

impl RestClient {
    /// Cancel an Existing Order and Send a New Order
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/spot/trade-api.html#Cancel%20an%20Existing%20Order%20and%20Send%20a%20New%20Order)
    ///
    /// Rate limit: UID 2/s & IP rate limit group 3
    ///
    /// # Arguments
    /// * `request` - The cancel and replace order request parameters
    ///
    /// # Returns
    /// The new order response containing order details or error
    pub async fn cancel_replace_order(
        &self,
        request: CancelReplaceOrderRequest,
    ) -> RestResult<CancelReplaceOrderResponse> {
        self.send_post_signed_request(
            CANCEL_REPLACE_ORDER_ENDPOINT,
            &request,
            EndpointType::Trading,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_replace_order_request_serialization() {
        let request = CancelReplaceOrderRequest {
            symbol: "BTC-USDT".to_string(),
            cancel_order_id: Some(123456789),
            cancel_client_order_id: None,
            cancel_restrictions: Some(CancelRestriction::New),
            cancel_replace_mode: CancelReplaceMode::StopOnFailure,
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            stop_price: None,
            quantity: Some(0.001),
            quote_order_qty: None,
            price: Some(50000.0),
            new_client_order_id: Some("new_order_1".to_string()),
            recv_window: Some(5000),
            timestamp: 1658748648396,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("BTC-USDT"));
        assert!(json.contains("123456789"));
        assert!(json.contains("STOP_ON_FAILURE"));
        assert!(json.contains("new_order_1"));
    }

    #[test]
    fn test_cancel_replace_order_response_deserialization() {
        let json = r#"{
            "symbol": "BTC-USDT",
            "orderId": 987654321,
            "price": "51000.00",
            "origQty": "0.001",
            "executedQty": "0.000",
            "cummulativeQuoteQty": "0.00",
            "status": "NEW",
            "type": "LIMIT",
            "side": "BUY",
            "clientOrderID": "new_order_1",
            "stopPrice": "0.00",
            "transactTime": 1658748648400
        }"#;

        let response: CancelReplaceOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTC-USDT");
        assert_eq!(response.order_id, 987654321);
        assert_eq!(response.status, OrderStatus::New);
        assert_eq!(response.client_order_id, "new_order_1");
    }
}
