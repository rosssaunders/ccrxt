use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult};

const GET_OCO_ORDER_ENDPOINT: &str = "/openApi/spot/v1/oco/orderList";

/// Request for getting an OCO order list
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOcoOrderRequest {
    /// OCO order group ID. Either orderListId or clientOrderId must be filled in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_list_id: Option<String>,

    /// User-defined OCO order group ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Request valid time window, in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Request timestamp, in milliseconds
    pub timestamp: i64,
}

/// Response for getting an OCO order list
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOcoOrderResponse {
    /// List of OCO orders
    pub data: Vec<OcoOrderInfo>,
}

/// Information about an OCO order
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcoOrderInfo {
    /// Order time
    pub transaction_time: i64,

    /// Order ID
    pub order_id: String,

    /// User-defined order ID
    pub client_order_id: String,

    /// Trading pair
    pub symbol: String,

    /// Order type: ocoLimit (OCO limit order), ocoTps (OCO stop-limit order)
    pub order_type: String,

    /// Order side: BUY for buy, SELL for sell
    pub side: String,

    /// Trigger price
    pub trigger_price: f64,

    /// Order price
    pub price: f64,

    /// Order quantity
    pub quantity: f64,

    /// OCO order group ID
    pub order_list_id: String,
}

impl RestClient {
    /// Query an OCO order list
    ///
    /// Get information about a specific OCO order list.
    ///
    /// # Arguments
    /// * `request` - The get OCO order request
    ///
    /// # Returns
    /// * `RestResult<GetOcoOrderResponse>` - The OCO order response or error
    pub async fn get_oco_order(
        &self,
        request: &GetOcoOrderRequest,
    ) -> RestResult<GetOcoOrderResponse> {
        self.send_request(
            GET_OCO_ORDER_ENDPOINT,
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
    fn test_get_oco_order_request_serialization_with_order_list_id() {
        let request = GetOcoOrderRequest {
            order_list_id: Some("123456789".to_string()),
            client_order_id: None,
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"orderListId\":\"123456789\""));
        assert!(!json.contains("\"clientOrderId\""));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_get_oco_order_request_serialization_with_client_order_id() {
        let request = GetOcoOrderRequest {
            order_list_id: None,
            client_order_id: Some("my_oco_order".to_string()),
            recv_window: None,
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(!json.contains("\"orderListId\""));
        assert!(json.contains("\"clientOrderId\":\"my_oco_order\""));
        assert!(!json.contains("\"recvWindow\""));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_get_oco_order_response_deserialization() {
        let json = r#"
        {
            "data": [
                {
                    "transactionTime": 1640995200000,
                    "orderId": "987654321",
                    "clientOrderId": "my_order_123",
                    "symbol": "BTCUSDT",
                    "orderType": "ocoLimit",
                    "side": "BUY",
                    "triggerPrice": 48500.0,
                    "price": 50000.0,
                    "quantity": 0.001,
                    "orderListId": "123456789"
                }
            ]
        }
        "#;

        let response: GetOcoOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].order_id, "987654321");
        assert_eq!(response.data[0].symbol, "BTCUSDT");
        assert_eq!(response.data[0].order_type, "ocoLimit");
        assert_eq!(response.data[0].side, "BUY");
        assert_eq!(response.data[0].price, 50000.0);
        assert_eq!(response.data[0].quantity, 0.001);
    }
}
