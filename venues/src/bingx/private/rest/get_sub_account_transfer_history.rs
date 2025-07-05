use crate::bingx::enums::SubAccountTransferType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubAccountTransferHistoryRequest {
    /// Asset symbol
    pub asset: Option<String>,
    /// Transfer type
    pub r#type: Option<SubAccountTransferType>,
    /// Start time timestamp in ms
    pub start_time: Option<i64>,
    /// End time timestamp in ms
    pub end_time: Option<i64>,
    /// Page number, starting from 1
    pub page: Option<i32>,
    /// Number of items per page, max 500
    pub limit: Option<i32>,
    /// Timestamp in ms
    pub recv_window: Option<i64>,
    /// Timestamp in ms
    pub timestamp: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubAccountTransferHistoryResponse {
    /// List of transfer records
    pub data: Vec<SubAccountTransferRecord>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountTransferRecord {
    /// From email
    pub from: String,
    /// To email
    pub to: String,
    /// Asset symbol
    pub asset: String,
    /// Transfer amount
    pub qty: String,
    /// Transfer type (1: to sub-account, 2: to master)
    pub r#type: String,
    /// Transfer status
    pub status: String,
    /// Transaction ID
    pub tranId: i64,
    /// Transfer time
    pub time: i64,
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sub_account_transfer_history_request_serialization() {
        let request = GetSubAccountTransferHistoryRequest {
            asset: Some("USDT".to_string()),
            r#type: Some(SubAccountTransferType::ToSub),
            start_time: Some(1640908800000),
            end_time: Some(1640995200000),
            page: Some(1),
            limit: Some(100),
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"asset\":\"USDT\""));
        assert!(json.contains("\"type\":\"1\""));
        assert!(json.contains("\"startTime\":1640908800000"));
        assert!(json.contains("\"endTime\":1640995200000"));
        assert!(json.contains("\"page\":1"));
        assert!(json.contains("\"limit\":100"));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_get_sub_account_transfer_history_response_deserialization() {
        let json = r#"
        {
            "data": [
                {
                    "from": "master@example.com",
                    "to": "sub@example.com",
                    "asset": "USDT",
                    "qty": "100.0",
                    "type": "1",
                    "status": "SUCCESS",
                    "tranId": 123456789,
                    "time": 1640995200000
                },
                {
                    "from": "sub@example.com",
                    "to": "master@example.com",
                    "asset": "BTC",
                    "qty": "0.001",
                    "type": "2",
                    "status": "SUCCESS",
                    "tranId": 987654321,
                    "time": 1640995100000
                }
            ]
        }
        "#;

        let response: GetSubAccountTransferHistoryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].from, "master@example.com");
        assert_eq!(response.data[0].to, "sub@example.com");
        assert_eq!(response.data[0].asset, "USDT");
        assert_eq!(response.data[0].qty, "100.0");
        assert_eq!(response.data[0].r#type, "1");
        assert_eq!(response.data[0].status, "SUCCESS");
    }
}
