use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::{EndpointType, RestResult};

const OPEN_OCO_ORDERS_ENDPOINT: &str = "/openApi/spot/v1/oco/openOrderList";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenOcoOrdersRequest {
    /// Page number
    pub page_index: i64,

    /// Number of items per page
    pub page_size: i64,

    /// Request validity window, in milliseconds
    pub recv_window: Option<i64>,

    /// Request timestamp, in milliseconds
    pub timestamp: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenOcoOrdersResponse {
    pub data: Vec<OpenOcoOrder>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenOcoOrder {
    /// Order time
    pub transaction_time: i64,

    /// Order ID
    pub order_id: String,

    /// User-defined order ID
    pub client_order_id: String,

    /// Trading pair
    pub symbol: String,

    /// ocoLimit: OCO Limit Order, ocoTps: OCO Stop-Limit Order
    pub order_type: String,

    /// Trade type, BUY for buy, SELL for sell
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
    /// Query all open OCO orders
    ///
    /// Retrieves the list of orders that are currently in the pending order state.
    /// Rate limit: 5/s by UID & 2 by IP in group
    ///
    /// # Arguments
    /// * `request` - The get open OCO orders request with pagination parameters
    ///
    /// # Returns
    /// A result containing the open OCO orders or an error
    pub async fn get_open_oco_orders(
        &self,
        request: &GetOpenOcoOrdersRequest,
    ) -> RestResult<GetOpenOcoOrdersResponse> {
        self.send_request(
            OPEN_OCO_ORDERS_ENDPOINT,
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
    fn test_get_open_oco_orders_request_serialization() {
        let request = GetOpenOcoOrdersRequest {
            page_index: 1,
            page_size: 10,
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"pageIndex\":1"));
        assert!(json.contains("\"pageSize\":10"));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_get_open_oco_orders_response_deserialization() {
        let json = r#"
        {
            "data": [
                {
                    "transactionTime": 1640995200000,
                    "orderId": "123456789",
                    "clientOrderId": "client123",
                    "symbol": "BTCUSDT",
                    "orderType": "ocoLimit",
                    "side": "BUY",
                    "triggerPrice": 50000.0,
                    "price": 49000.0,
                    "quantity": 0.001,
                    "orderListId": "456789123"
                }
            ]
        }
        "#;

        let response: GetOpenOcoOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].order_id, "123456789");
        assert_eq!(response.data[0].symbol, "BTCUSDT");
        assert_eq!(response.data[0].order_type, "ocoLimit");
        assert_eq!(response.data[0].side, "BUY");
        assert_eq!(response.data[0].trigger_price, 50000.0);
        assert_eq!(response.data[0].price, 49000.0);
        assert_eq!(response.data[0].quantity, 0.001);
        assert_eq!(response.data[0].order_list_id, "456789123");
    }
}
