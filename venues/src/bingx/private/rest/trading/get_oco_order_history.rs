use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, PrivateRestClient as RestClient, RestResult};

const GET_OCO_ORDER_HISTORY_ENDPOINT: &str = "/openApi/spot/v1/oco/historyOrderList";

/// Request for getting OCO historical order list
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOcoOrderHistoryRequest {
    /// Page number (required)
    pub page_index: i64,

    /// Number of items per page (required)
    pub page_size: i64,

    /// Start time, timestamp, in milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,

    /// End time, timestamp, in milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,

    /// Request validity window, in milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Request timestamp, in milliseconds (required)
    pub timestamp: i64,
}

/// Response for getting OCO historical order list
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOcoOrderHistoryResponse {
    /// List of historical OCO orders
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
    /// Query OCO Historical Order List
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/spot/trade-api.html#Query%20OCO%20Historical%20Order%20List)
    ///
    /// Rate limit: UID 5/s & IP rate limit group 2
    ///
    /// Query OCO historical order list
    ///
    /// # Arguments
    /// * `request` - The get OCO order history request
    ///
    /// # Returns
    /// * `RestResult<GetOcoOrderHistoryResponse>` - The OCO order history response or error
    pub async fn get_oco_order_history(
        &self,
        request: &GetOcoOrderHistoryRequest,
    ) -> RestResult<GetOcoOrderHistoryResponse> {
        self.send_get_signed_request(
            GET_OCO_ORDER_HISTORY_ENDPOINT,
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
    fn test_get_oco_order_history_request_serialization() {
        let request = GetOcoOrderHistoryRequest {
            page_index: 1,
            page_size: 10,
            start_time: Some(1640995200000),
            end_time: Some(1640998800000),
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"pageIndex\":1"));
        assert!(json.contains("\"pageSize\":10"));
        assert!(json.contains("\"startTime\":1640995200000"));
        assert!(json.contains("\"endTime\":1640998800000"));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_get_oco_order_history_request_minimal() {
        let request = GetOcoOrderHistoryRequest {
            page_index: 1,
            page_size: 20,
            start_time: None,
            end_time: None,
            recv_window: None,
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"pageIndex\":1"));
        assert!(json.contains("\"pageSize\":20"));
        assert!(!json.contains("\"startTime\""));
        assert!(!json.contains("\"endTime\""));
        assert!(!json.contains("\"recvWindow\""));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_get_oco_order_history_response_deserialization() {
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

        let response: GetOcoOrderHistoryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].order_id, "987654321");
        assert_eq!(response.data[0].symbol, "BTCUSDT");
        assert_eq!(response.data[0].order_type, "ocoLimit");
        assert_eq!(response.data[0].side, "BUY");
        assert_eq!(response.data[0].price, 50000.0);
        assert_eq!(response.data[0].quantity, 0.001);
    }
}
