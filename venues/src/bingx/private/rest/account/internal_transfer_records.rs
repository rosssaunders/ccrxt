use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, PrivateRestClient as RestClient, RestResult};

const INTERNAL_TRANSFER_RECORDS_ENDPOINT: &str =
    "/openApi/wallets/v1/capital/innerTransfer/records";

/// Request to get internal transfer records
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalTransferRecordsRequest {
    /// Internal transfer ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Transfer coin name (required)
    pub coin: String,

    /// Client's self-defined internal transfer ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_client_id: Option<String>,

    /// Start time (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,

    /// End time (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,

    /// Starting record number (optional) - default is 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// Page size (optional) - default is 100, maximum is 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Request timestamp in milliseconds (required)
    pub timestamp: i64,

    /// Request valid time window in milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
}

/// Internal transfer record
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalTransferRecord {
    /// Inner transfer ID
    pub id: i64,

    /// Coin name
    pub coin: String,

    /// Receiver UID
    pub receiver: i64,

    /// Transfer amount
    pub amount: Decimal,

    /// Internal transfer time
    pub time: i64,

    /// Status: 4-Pending review, 5-Failed, 6-Completed
    pub status: i32,

    /// Client's self-defined internal transfer ID
    pub transfer_client_id: String,

    /// Payer's account
    pub from_uid: i64,

    /// Record type: "out" for transfer out record, "in" for transfer in record
    pub record_type: String,
}

/// Response for internal transfer records
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InternalTransferRecordsResponse {
    /// Inner transfer records list
    pub data: Vec<InternalTransferRecord>,

    /// Total number of addresses
    pub total: i32,
}

impl RestClient {
    /// Main account internal transfer records
    ///
    /// This endpoint is used for the parent user to query their own inner transfer records.
    /// Only available for parent users.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/spot/account-api.html#Main%20account%20internal%20transfer%20records)
    ///
    /// Rate limit: UID 10/s & IP rate limit group 2
    ///
    /// # Arguments
    /// * `request` - The internal transfer records request with optional filters
    ///
    /// # Returns
    /// A result containing the transfer records or an error
    pub async fn get_internal_transfer_records(
        &self,
        request: &InternalTransferRecordsRequest,
    ) -> RestResult<InternalTransferRecordsResponse> {
        self.send_get_signed_request(
            INTERNAL_TRANSFER_RECORDS_ENDPOINT,
            request,
            EndpointType::Account,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_internal_transfer_records_request_serialization() {
        let request = InternalTransferRecordsRequest {
            id: Some("123456".to_string()),
            coin: "USDT".to_string(),
            transfer_client_id: Some("CLIENT123".to_string()),
            start_time: Some(1640995200000),
            end_time: Some(1641081600000),
            offset: Some(0),
            limit: Some(20),
            timestamp: 1658748648396,
            recv_window: Some(5000),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"id\":\"123456\""));
        assert!(json.contains("\"coin\":\"USDT\""));
        assert!(json.contains("\"transferClientId\":\"CLIENT123\""));
        assert!(json.contains("\"startTime\":1640995200000"));
        assert!(json.contains("\"endTime\":1641081600000"));
        assert!(json.contains("\"offset\":0"));
        assert!(json.contains("\"limit\":20"));
        assert!(json.contains("\"timestamp\":1658748648396"));
        assert!(json.contains("\"recvWindow\":5000"));
    }

    #[test]
    fn test_minimal_request() {
        let request = InternalTransferRecordsRequest {
            id: None,
            coin: "BTC".to_string(),
            transfer_client_id: None,
            start_time: None,
            end_time: None,
            offset: None,
            limit: None,
            timestamp: 1658748648396,
            recv_window: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"coin\":\"BTC\""));
        assert!(json.contains("\"timestamp\":1658748648396"));
        assert!(!json.contains("id"));
        assert!(!json.contains("transferClientId"));
        assert!(!json.contains("recvWindow"));
    }

    #[test]
    fn test_internal_transfer_records_response_deserialization() {
        let json = r#"
        {
            "data": [
                {
                    "id": 123456,
                    "coin": "USDT",
                    "receiver": 789012,
                    "amount": "100.50",
                    "time": 1640995200000,
                    "status": 6,
                    "transferClientId": "CLIENT123",
                    "fromUid": 345678,
                    "recordType": "out"
                },
                {
                    "id": 123457,
                    "coin": "BTC",
                    "receiver": 789013,
                    "amount": "0.001",
                    "time": 1640995300000,
                    "status": 6,
                    "transferClientId": "CLIENT124",
                    "fromUid": 345678,
                    "recordType": "in"
                }
            ],
            "total": 2
        }
        "#;

        let response: InternalTransferRecordsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.total, 2);

        let first_record = &response.data[0];
        assert_eq!(first_record.id, 123456);
        assert_eq!(first_record.coin, "USDT");
        assert_eq!(first_record.receiver, 789012);
        assert_eq!(first_record.amount, dec!(100.50));
        assert_eq!(first_record.time, 1640995200000);
        assert_eq!(first_record.status, 6);
        assert_eq!(first_record.transfer_client_id, "CLIENT123");
        assert_eq!(first_record.from_uid, 345678);
        assert_eq!(first_record.record_type, "out");

        let second_record = &response.data[1];
        assert_eq!(second_record.id, 123457);
        assert_eq!(second_record.coin, "BTC");
        assert_eq!(second_record.amount, dec!(0.001));
        assert_eq!(second_record.record_type, "in");
    }
}
