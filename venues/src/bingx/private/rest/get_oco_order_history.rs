use crate::bingx::enums::OcoOrderStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOcoOrderHistoryRequest {
    /// Start time timestamp in ms
    pub start_time: Option<i64>,
    /// End time timestamp in ms
    pub end_time: Option<i64>,
    /// From which order list id to return, used for pagination
    pub from_id: Option<i64>,
    /// Limit of results, default 500, max 1000
    pub limit: Option<i32>,
    /// Timestamp in ms
    pub recv_window: Option<i64>,
    /// Timestamp in ms
    pub timestamp: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOcoOrderHistoryResponse {
    pub data: Vec<OcoOrderHistory>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcoOrderHistory {
    pub order_list_id: i64,
    pub contingency_type: String,
    pub list_status_type: OcoOrderStatus,
    pub list_order_status: OcoOrderStatus,
    pub list_client_order_id: String,
    pub transaction_time: i64,
    pub symbol: String,
    pub orders: Vec<OcoHistorySubOrder>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcoHistorySubOrder {
    pub symbol: String,
    pub order_id: i64,
    pub client_order_id: String,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: String,
    pub time_in_force: String,
    pub r#type: String,
    pub side: String,
    pub stop_price: Option<String>,
    pub iceberg_qty: Option<String>,
    pub time: i64,
    pub update_time: i64,
    pub is_working: bool,
    pub orig_quote_order_qty: Option<String>,
}

impl GetOcoOrderHistoryRequest {
    pub fn new(timestamp: i64) -> Self {
        Self {
            start_time: None,
            end_time: None,
            from_id: None,
            limit: None,
            recv_window: None,
            timestamp,
        }
    }

    pub fn start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn from_id(mut self, from_id: i64) -> Self {
        self.from_id = Some(from_id);
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_oco_order_history_request_serialization() {
        let request = GetOcoOrderHistoryRequest::new(1640995200000)
            .start_time(1640908800000)
            .end_time(1640995200000)
            .limit(100)
            .recv_window(5000);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"startTime\":1640908800000"));
        assert!(json.contains("\"endTime\":1640995200000"));
        assert!(json.contains("\"limit\":100"));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_get_oco_order_history_response_deserialization() {
        let json = r#"
        {
            "data": [
                {
                    "orderListId": 123456789,
                    "contingencyType": "OCO",
                    "listStatusType": "ALL_DONE",
                    "listOrderStatus": "DONE",
                    "listClientOrderId": "client123",
                    "transactionTime": 1640995200000,
                    "symbol": "BTCUSDT",
                    "orders": [
                        {
                            "symbol": "BTCUSDT",
                            "orderId": 987654321,
                            "clientOrderId": "order1",
                            "price": "50000.00",
                            "origQty": "0.001",
                            "executedQty": "0.001",
                            "cummulativeQuoteQty": "50.0",
                            "status": "FILLED",
                            "timeInForce": "GTC",
                            "type": "LIMIT",
                            "side": "BUY",
                            "time": 1640995200000,
                            "updateTime": 1640995200000,
                            "isWorking": false
                        }
                    ]
                }
            ]
        }
        "#;

        let response: GetOcoOrderHistoryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].order_list_id, 123456789);
        assert_eq!(response.data[0].symbol, "BTCUSDT");
        assert_eq!(response.data[0].orders.len(), 1);
        assert_eq!(response.data[0].orders[0].status, "FILLED");
    }
}
