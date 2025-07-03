use serde::{Deserialize, Serialize};

use crate::bingx::{
    enums::{OcoOrderType, OrderSide, OrderStatus},
    errors::BingXError,
    BingXRestClient,
};

/// Request for creating an OCO (One-Cancels-Other) order
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOcoOrderRequest {
    /// Trading symbol, e.g., BTC-USDT
    pub symbol: String,
    /// Order side: BUY or SELL
    pub side: OrderSide,
    /// Order quantity
    pub quantity: f64,
    /// Limit order price
    pub limit_price: f64,
    /// Limit order price set after a stop-limit order is triggered
    pub order_price: f64,
    /// Trigger price of the stop-limit order
    pub trigger_price: f64,
    /// Custom unique ID for the entire Order List
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_client_order_id: Option<String>,
    /// Custom unique ID for the limit order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub above_client_order_id: Option<String>,
    /// Custom unique ID for the stop-limit order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub below_client_order_id: Option<String>,
    /// Request validity time window in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
    /// Request timestamp in milliseconds
    pub timestamp: u64,
}

/// Response for creating an OCO order
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOcoOrderResponse {
    /// Array of OCO orders created
    pub orders: Vec<OcoOrderInfo>,
}

/// Information about an OCO order
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcoOrderInfo {
    /// Order ID
    pub order_id: String,
    /// Custom order ID
    pub client_order_id: String,
    /// Order type: ocoLimit or ocoTps
    pub order_type: OcoOrderType,
    /// Trading symbol
    pub symbol: String,
    /// Order price
    pub price: f64,
    /// Trigger price
    pub trigger_price: f64,
    /// Order quantity
    pub quantity: f64,
    /// Order status
    pub status: OrderStatus,
    /// Order side
    pub side: OrderSide,
}

impl BingXRestClient {
    /// Create an OCO (One-Cancels-Other) order
    ///
    /// Sends a new one-cancels-the-other (OCO) order, initiating one of them
    /// immediately cancels the other order.
    ///
    /// # Arguments
    /// * `request` - The OCO order creation request
    ///
    /// # Returns
    /// * `Result<CreateOcoOrderResponse, BingXError>` - The OCO order response or error
    pub async fn create_oco_order(
        &self,
        request: CreateOcoOrderRequest,
    ) -> Result<CreateOcoOrderResponse, BingXError> {
        self.send_signed_request("POST", "/openApi/spot/v1/oco/order", Some(&request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_oco_order_request_serialization() {
        let request = CreateOcoOrderRequest {
            symbol: "BTC-USDT".to_string(),
            side: OrderSide::Buy,
            quantity: 0.001,
            limit_price: 50000.0,
            order_price: 48000.0,
            trigger_price: 48500.0,
            list_client_order_id: Some("oco_list_1".to_string()),
            above_client_order_id: Some("limit_order_1".to_string()),
            below_client_order_id: Some("stop_order_1".to_string()),
            recv_window: Some(5000),
            timestamp: 1658748648396,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("BTC-USDT"));
        assert!(json.contains("50000"));
        assert!(json.contains("48000"));
        assert!(json.contains("48500"));
        assert!(json.contains("oco_list_1"));
    }

    #[test]
    fn test_create_oco_order_response_deserialization() {
        let json = r#"{
            "orders": [
                {
                    "orderId": "123456789",
                    "clientOrderId": "limit_order_1",
                    "orderType": "ocoLimit",
                    "symbol": "BTC-USDT",
                    "price": 50000.0,
                    "triggerPrice": 0.0,
                    "quantity": 0.001,
                    "status": "NEW",
                    "side": "BUY"
                },
                {
                    "orderId": "123456790",
                    "clientOrderId": "stop_order_1",
                    "orderType": "ocoTps",
                    "symbol": "BTC-USDT",
                    "price": 48000.0,
                    "triggerPrice": 48500.0,
                    "quantity": 0.001,
                    "status": "NEW",
                    "side": "BUY"
                }
            ]
        }"#;

        let response: CreateOcoOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.orders.len(), 2);
        assert_eq!(response.orders[0].order_type, OcoOrderType::OcoLimit);
        assert_eq!(response.orders[1].order_type, OcoOrderType::OcoTps);
        assert_eq!(response.orders[0].symbol, "BTC-USDT");
    }
}
