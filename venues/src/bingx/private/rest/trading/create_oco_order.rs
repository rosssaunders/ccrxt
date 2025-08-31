use serde::{Deserialize, Serialize};

use crate::bingx::{
    EndpointType, PrivateRestClient as RestClient, RestResult,
    enums::{OcoOrderType, OrderSide},
};

const CREATE_OCO_ORDER_ENDPOINT: &str = "/openApi/spot/v1/oco/order";

/// Request for creating an OCO (One-Cancels-Other) order
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOcoOrderRequest {
    /// Trading pair, e.g., BTC-USDT, please use uppercase letters (required)
    pub symbol: String,

    /// Order type, BUY for buy, SELL for sell (required)
    pub side: OrderSide,

    /// Order quantity, e.g., 0.1 BTC (required)
    pub quantity: String,

    /// Limit order price. e.g., 10000 USDT (required)
    pub limit_price: String,

    /// The limit order price set after a stop-limit order is triggered. e.g., 10000 USDT (required)
    pub order_price: String,

    /// The trigger price of the stop-limit order. e.g., 10000 USDT (required)
    pub trigger_price: String,

    /// Custom unique ID for the entire Order List, only supports numeric strings, e.g., "123456" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_client_order_id: Option<String>,

    /// Custom unique ID for the limit order, only supports numeric strings, e.g., "123456" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub above_client_order_id: Option<String>,

    /// Custom unique ID for the stop-limit order, only supports numeric strings, e.g., "123456" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub below_client_order_id: Option<String>,

    /// Request validity time window, in milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Request timestamp, in milliseconds (required)
    pub timestamp: i64,
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

    /// Order type: ocoLimit: OCO Limit Order, ocoTps: OCO Stop-Limit Order
    pub order_type: OcoOrderType,

    /// Trading pair
    pub symbol: String,

    /// Order price
    pub price: f64,

    /// Trigger price
    pub trigger_price: f64,

    /// Order quantity
    pub quantity: f64,

    /// Order status, NEW for new order, PENDING for pending, PARTIALLY_FILLED for partially filled, FILLED for fully filled, CANCELED for canceled, FAILED for failed
    pub status: String,

    /// Order type, BUY for buy, SELL for sell
    pub side: OrderSide,
}

impl RestClient {
    /// Create an OCO Order
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/spot/trade-api.html#Create%20an%20OCO%20Order)
    ///
    /// Rate limit: UID 2/s & IP rate limit group 2
    ///
    /// Send a new one-cancels-the-other (OCO) order, and initiating one of them immediately cancels the other order
    ///
    /// # Arguments
    /// * `request` - The OCO order creation request
    ///
    /// # Returns
    /// A result containing the OCO order response or an error
    pub async fn create_oco_order(
        &self,
        request: &CreateOcoOrderRequest,
    ) -> RestResult<CreateOcoOrderResponse> {
        self.send_post_signed_request(CREATE_OCO_ORDER_ENDPOINT, request, EndpointType::Trading)
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
            quantity: "0.001".to_string(),
            limit_price: "50000.0".to_string(),
            order_price: "48000.0".to_string(),
            trigger_price: "48500.0".to_string(),
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

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("timestamp=1658748648396"));
        assert!(serialized.contains("recvWindow=5000"));
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
