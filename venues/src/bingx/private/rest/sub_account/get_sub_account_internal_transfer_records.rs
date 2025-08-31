use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, PrivateRestClient as RestClient, RestResult};

const GET_SUB_ACCOUNT_INTERNAL_TRANSFER_RECORDS_ENDPOINT: &str =
    "/openApi/wallets/v1/capital/subAccount/innerTransfer/records";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubAccountInternalTransferRecordsRequest {
    /// Transfer currency name
    pub coin: String,

    /// Client's self-defined internal transfer ID. When both platform ID and transferClientId are provided as input, the query will be based on the platform ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_client_id: Option<String>,

    /// Start time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,

    /// End time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,

    /// Starting record number, default is 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// Page size, default is 100, maximum is 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Request timestamp in milliseconds
    pub timestamp: i64,

    /// Request valid time window, in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
}

/// Individual internal transfer record information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalTransferRecord {
    /// Internal transfer ID
    pub id: i64,

    /// Currency name
    pub coin: String,

    /// Receiver's UID
    pub receiver: i64,

    /// Transfer amount
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    pub amount: Option<String>,

    /// Internal transfer time
    pub time: i64,

    /// Status 4-Pending review 5-Failed 6-Completed
    pub status: i32,

    /// Client's self-defined internal transfer ID. When both platform ID and transferClientId are provided as input, the query will be based on the platform ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_client_id: Option<String>,

    /// Payer's account
    pub from_uid: i64,

    /// Record type: Out: transfer out record, in: transfer in record
    pub record_type: String,
}

/// Response for getting sub-account internal transfer records
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubAccountInternalTransferRecordsResponse {
    /// Internal transfer record list
    pub data: Vec<InternalTransferRecord>,

    /// Total number of addresses
    pub total: i32,
}

impl RestClient {
    /// Query sub-account internal transfer records
    ///
    /// This node is used for sub-accounts to query their own internal transfer
    /// records. The user who verifies the signature of this API must be sub-account.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/common/sub-account#Query%20sub-account%20internal%20transfer%20records)
    ///
    /// Rate limit: UID 10/s & IP rate limit group 2
    ///
    /// # Arguments
    /// * `request` - The get internal transfer records request parameters
    ///
    /// # Returns
    /// A result containing the internal transfer records information or an error
    pub async fn get_sub_account_internal_transfer_records(
        &self,
        request: &GetSubAccountInternalTransferRecordsRequest,
    ) -> RestResult<GetSubAccountInternalTransferRecordsResponse> {
        self.send_get_signed_request(
            GET_SUB_ACCOUNT_INTERNAL_TRANSFER_RECORDS_ENDPOINT,
            request,
            EndpointType::AccountApiGroup2,
        )
        .await
    }
}

// Serde helper module for handling empty strings as None
mod serde_with {
    pub mod rust {
        pub mod string_empty_as_none {
            use serde::{Deserialize, Deserializer};

            pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                if s.is_empty() { Ok(None) } else { Ok(Some(s)) }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sub_account_internal_transfer_records_request_serialization() {
        let request = GetSubAccountInternalTransferRecordsRequest {
            coin: "USDT".to_string(),
            transfer_client_id: Some("client_transfer_123".to_string()),
            start_time: Some(1640995200000),
            end_time: Some(1641081600000),
            offset: Some(0),
            limit: Some(100),
            timestamp: 1640995200000,
            recv_window: Some(5000),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"coin\":\"USDT\""));
        assert!(json.contains("\"transferClientId\":\"client_transfer_123\""));
        assert!(json.contains("\"startTime\":1640995200000"));
        assert!(json.contains("\"endTime\":1641081600000"));
        assert!(json.contains("\"offset\":0"));
        assert!(json.contains("\"limit\":100"));
        assert!(json.contains("\"timestamp\":1640995200000"));
        assert!(json.contains("\"recvWindow\":5000"));
    }

    #[test]
    fn test_get_sub_account_internal_transfer_records_request_serialization_minimal() {
        let request = GetSubAccountInternalTransferRecordsRequest {
            coin: "BTC".to_string(),
            transfer_client_id: None,
            start_time: None,
            end_time: None,
            offset: None,
            limit: None,
            timestamp: 1640995200000,
            recv_window: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"coin\":\"BTC\""));
        assert!(json.contains("\"timestamp\":1640995200000"));
        assert!(!json.contains("\"transferClientId\""));
        assert!(!json.contains("\"startTime\""));
        assert!(!json.contains("\"endTime\""));
        assert!(!json.contains("\"offset\""));
        assert!(!json.contains("\"limit\""));
        assert!(!json.contains("\"recvWindow\""));
    }

    #[test]
    fn test_internal_transfer_record_deserialization() {
        let json = r#"{
            "id": 1234567890,
            "coin": "USDT",
            "receiver": 987654321,
            "amount": "100.50000000",
            "time": 1640995200000,
            "status": 6,
            "transferClientId": "client_transfer_123",
            "fromUid": 123456789,
            "recordType": "out"
        }"#;

        let record: InternalTransferRecord = serde_json::from_str(json).unwrap();

        assert_eq!(record.id, 1234567890);
        assert_eq!(record.coin, "USDT");
        assert_eq!(record.receiver, 987654321);
        assert_eq!(record.amount, Some("100.50000000".to_string()));
        assert_eq!(record.time, 1640995200000);
        assert_eq!(record.status, 6);
        assert_eq!(
            record.transfer_client_id,
            Some("client_transfer_123".to_string())
        );
        assert_eq!(record.from_uid, 123456789);
        assert_eq!(record.record_type, "out");
    }

    #[test]
    fn test_internal_transfer_record_deserialization_empty_amount() {
        let json = r#"{
            "id": 1234567890,
            "coin": "USDT",
            "receiver": 987654321,
            "amount": "",
            "time": 1640995200000,
            "status": 4,
            "fromUid": 123456789,
            "recordType": "in"
        }"#;

        let record: InternalTransferRecord = serde_json::from_str(json).unwrap();

        assert_eq!(record.id, 1234567890);
        assert_eq!(record.coin, "USDT");
        assert_eq!(record.receiver, 987654321);
        assert_eq!(record.amount, None);
        assert_eq!(record.time, 1640995200000);
        assert_eq!(record.status, 4);
        assert_eq!(record.transfer_client_id, None);
        assert_eq!(record.from_uid, 123456789);
        assert_eq!(record.record_type, "in");
    }

    #[test]
    fn test_get_sub_account_internal_transfer_records_response_deserialization() {
        let json = r#"{
            "data": [
                {
                    "id": 1234567890,
                    "coin": "USDT",
                    "receiver": 987654321,
                    "amount": "100.50000000",
                    "time": 1640995200000,
                    "status": 6,
                    "transferClientId": "client_transfer_123",
                    "fromUid": 123456789,
                    "recordType": "out"
                },
                {
                    "id": 1234567891,
                    "coin": "BTC",
                    "receiver": 123456789,
                    "amount": "0.001",
                    "time": 1640995300000,
                    "status": 5,
                    "fromUid": 987654321,
                    "recordType": "in"
                }
            ],
            "total": 2
        }"#;

        let response: GetSubAccountInternalTransferRecordsResponse =
            serde_json::from_str(json).unwrap();

        assert_eq!(response.total, 2);
        assert_eq!(response.data.len(), 2);

        let first_record = &response.data[0];
        assert_eq!(first_record.id, 1234567890);
        assert_eq!(first_record.coin, "USDT");
        assert_eq!(first_record.receiver, 987654321);
        assert_eq!(first_record.amount, Some("100.50000000".to_string()));
        assert_eq!(first_record.status, 6);
        assert_eq!(first_record.record_type, "out");

        let second_record = &response.data[1];
        assert_eq!(second_record.id, 1234567891);
        assert_eq!(second_record.coin, "BTC");
        assert_eq!(second_record.receiver, 123456789);
        assert_eq!(second_record.amount, Some("0.001".to_string()));
        assert_eq!(second_record.status, 5);
        assert_eq!(second_record.record_type, "in");
        assert_eq!(second_record.transfer_client_id, None);
    }

    #[test]
    fn test_internal_transfer_record_status_values() {
        // Test different status values
        let test_cases = vec![(4, "Pending review"), (5, "Failed"), (6, "Completed")];

        for (status_code, _description) in test_cases {
            let json = format!(
                r#"{{
                "id": 1234567890,
                "coin": "TEST",
                "receiver": 987654321,
                "amount": "100.0",
                "time": 1640995200000,
                "status": {},
                "fromUid": 123456789,
                "recordType": "out"
            }}"#,
                status_code
            );

            let record: InternalTransferRecord = serde_json::from_str(&json).unwrap();
            assert_eq!(record.status, status_code);
        }
    }

    #[test]
    fn test_internal_transfer_record_record_type_values() {
        // Test different record type values
        let test_cases = vec!["out", "in"];

        for record_type in test_cases {
            let json = format!(
                r#"{{
                "id": 1234567890,
                "coin": "TEST",
                "receiver": 987654321,
                "amount": "100.0",
                "time": 1640995200000,
                "status": 6,
                "fromUid": 123456789,
                "recordType": "{}"
            }}"#,
                record_type
            );

            let record: InternalTransferRecord = serde_json::from_str(&json).unwrap();
            assert_eq!(record.record_type, record_type);
        }
    }

    #[test]
    fn test_get_sub_account_internal_transfer_records_response_empty() {
        let json = r#"{"data": [], "total": 0}"#;
        let response: GetSubAccountInternalTransferRecordsResponse =
            serde_json::from_str(json).unwrap();

        assert_eq!(response.total, 0);
        assert_eq!(response.data.len(), 0);
    }
}
