use serde::{Deserialize, Serialize};

use crate::bingx::{
    enums::{CancelReplaceMode, CancelRestriction, OrderSide, OrderStatus, OrderType},
    errors::BingXError,
    BingXRestClient,
};

/// Request for canceling an existing order and placing a new one
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelReplaceOrderRequest {
    /// Trading symbol, e.g., BTC-USDT
    pub symbol: String,
    /// ID of the order to be canceled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_order_id: Option<i64>,
    /// User-defined ID of the order to be canceled
    #[serde(rename = "cancelClientOrderID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_client_order_id: Option<String>,
    /// Cancel orders with specified status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_restrictions: Option<CancelRestriction>,
    /// Cancel and replace mode
    #[serde(rename = "CancelReplaceMode")]
    pub cancel_replace_mode: CancelReplaceMode,
    /// Order side: BUY or SELL
    pub side: OrderSide,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Trigger price for stop orders
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
    /// Custom order ID for the new order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,
    /// Request valid time window in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
    /// Request timestamp in milliseconds
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
    /// Original quantity
    pub orig_qty: String,
    /// Executed quantity
    pub executed_qty: String,
    /// Cumulative quote quantity
    pub cummulative_quote_qty: String,
    /// Order status
    pub status: OrderStatus,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Order side
    pub side: OrderSide,
    /// User-defined order ID
    #[serde(rename = "clientOrderID")]
    pub client_order_id: String,
    /// Trigger price
    pub stop_price: String,
    /// Cancel restrictions
    pub cancel_restrictions: Option<CancelRestriction>,
    /// Transaction timestamp
    pub transact_time: i64,
}

impl BingXRestClient {
    /// Cancel an existing order and send a new order
    ///
    /// # Arguments
    /// * `request` - The cancel and replace order request
    ///
    /// # Returns
    /// * `Result<CancelReplaceOrderResponse, BingXError>` - The new order response or error
    pub async fn cancel_replace_order(
        &self,
        request: CancelReplaceOrderRequest,
    ) -> Result<CancelReplaceOrderResponse, BingXError> {
        self.send_signed_request(
            "POST",
            "/openApi/spot/v1/trade/order/cancelReplace",
            Some(&request),
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
