use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, PrivateRestClient as RestClient, RestResult};

const GET_SUB_ACCOUNT_DEPOSIT_RECORDS_ENDPOINT: &str =
    "/openApi/wallets/v1/capital/deposit/subHisrec";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubAccountDepositRecordsRequest {
    /// Transfer currency name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,

    /// Sub-user UID, when not filled, query the deposit records of all sub-accounts under the parent username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_uid: Option<i64>,

    /// Status (0-In progress 6-Chain uploaded 1-Completed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,

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

    /// Transaction ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,

    /// Request timestamp in milliseconds
    pub timestamp: i64,

    /// Request valid time window, in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
}

/// Individual deposit record information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositRecord {
    /// Sub-account UID
    pub sub_uid: i64,

    /// Transfer amount
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    pub amount: Option<String>,

    /// Currency name
    pub coin: String,

    /// Network name
    pub network: String,

    /// Status (0-In progress 6-Chain uploaded 1-Completed)
    pub status: i32,

    /// Deposit address
    pub address: String,

    /// Deposit address tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_tag: Option<String>,

    /// Transaction ID
    pub tx_id: String,

    /// Transaction scan time
    pub insert_time: i64,

    /// Transfer type: 0-deposit
    pub transfer_type: i32,

    /// Number of confirmations required to unlock the deposit
    pub unlock_confirm_times: i32,

    /// Number of confirmations
    pub confirm_times: i32,
}

/// Response for getting sub-account deposit records
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubAccountDepositRecordsResponse {
    /// Internal transfer record list
    pub data: Vec<DepositRecord>,

    /// Total number of addresses
    pub total: i32,
}

impl RestClient {
    /// Get sub-account deposit records
    ///
    /// This node is used for the main user to query the deposit history of the
    /// sub-user. The user who verifies the signature of this API must be main account.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/common/sub-account#Get%20sub-account%20deposit%20records)
    ///
    /// Rate limit: UID 5/s & IP rate limit group 2
    ///
    /// # Arguments
    /// * `request` - The get deposit records request parameters
    ///
    /// # Returns
    /// A result containing the deposit records information or an error
    pub async fn get_sub_account_deposit_records(
        &self,
        request: &GetSubAccountDepositRecordsRequest,
    ) -> RestResult<GetSubAccountDepositRecordsResponse> {
        self.send_get_signed_request(
            GET_SUB_ACCOUNT_DEPOSIT_RECORDS_ENDPOINT,
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
    fn test_get_sub_account_deposit_records_request_serialization() {
        let request = GetSubAccountDepositRecordsRequest {
            coin: Some("USDT".to_string()),
            sub_uid: Some(123456789),
            status: Some(1),
            start_time: Some(1640995200000),
            end_time: Some(1641081600000),
            offset: Some(0),
            limit: Some(100),
            tx_id: Some("0x1234567890abcdef".to_string()),
            timestamp: 1640995200000,
            recv_window: Some(5000),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"coin\":\"USDT\""));
        assert!(json.contains("\"subUid\":123456789"));
        assert!(json.contains("\"status\":1"));
        assert!(json.contains("\"startTime\":1640995200000"));
        assert!(json.contains("\"endTime\":1641081600000"));
        assert!(json.contains("\"offset\":0"));
        assert!(json.contains("\"limit\":100"));
        assert!(json.contains("\"txId\":\"0x1234567890abcdef\""));
        assert!(json.contains("\"timestamp\":1640995200000"));
        assert!(json.contains("\"recvWindow\":5000"));
    }

    #[test]
    fn test_get_sub_account_deposit_records_request_serialization_minimal() {
        let request = GetSubAccountDepositRecordsRequest {
            coin: None,
            sub_uid: None,
            status: None,
            start_time: None,
            end_time: None,
            offset: None,
            limit: None,
            tx_id: None,
            timestamp: 1640995200000,
            recv_window: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"timestamp\":1640995200000"));
        assert!(!json.contains("\"coin\""));
        assert!(!json.contains("\"subUid\""));
        assert!(!json.contains("\"status\""));
        assert!(!json.contains("\"startTime\""));
        assert!(!json.contains("\"endTime\""));
        assert!(!json.contains("\"offset\""));
        assert!(!json.contains("\"limit\""));
        assert!(!json.contains("\"txId\""));
        assert!(!json.contains("\"recvWindow\""));
    }

    #[test]
    fn test_deposit_record_deserialization() {
        let json = r#"{
            "subUid": 123456789,
            "amount": "100.50000000",
            "coin": "USDT",
            "network": "TRC20",
            "status": 1,
            "address": "TUELLQv4jAQYLnQu7qhLLmv2kfQ6N1nC1s",
            "addressTag": "memo123",
            "txId": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            "insertTime": 1640995200000,
            "transferType": 0,
            "unlockConfirmTimes": 12,
            "confirmTimes": 12
        }"#;

        let record: DepositRecord = serde_json::from_str(json).unwrap();

        assert_eq!(record.sub_uid, 123456789);
        assert_eq!(record.amount, Some("100.50000000".to_string()));
        assert_eq!(record.coin, "USDT");
        assert_eq!(record.network, "TRC20");
        assert_eq!(record.status, 1);
        assert_eq!(record.address, "TUELLQv4jAQYLnQu7qhLLmv2kfQ6N1nC1s");
        assert_eq!(record.address_tag, Some("memo123".to_string()));
        assert_eq!(
            record.tx_id,
            "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
        );
        assert_eq!(record.insert_time, 1640995200000);
        assert_eq!(record.transfer_type, 0);
        assert_eq!(record.unlock_confirm_times, 12);
        assert_eq!(record.confirm_times, 12);
    }

    #[test]
    fn test_deposit_record_deserialization_empty_amount() {
        let json = r#"{
            "subUid": 123456789,
            "amount": "",
            "coin": "USDT",
            "network": "TRC20",
            "status": 0,
            "address": "TUELLQv4jAQYLnQu7qhLLmv2kfQ6N1nC1s",
            "txId": "0x1234567890abcdef",
            "insertTime": 1640995200000,
            "transferType": 0,
            "unlockConfirmTimes": 12,
            "confirmTimes": 0
        }"#;

        let record: DepositRecord = serde_json::from_str(json).unwrap();

        assert_eq!(record.sub_uid, 123456789);
        assert_eq!(record.amount, None);
        assert_eq!(record.coin, "USDT");
        assert_eq!(record.status, 0);
        assert_eq!(record.confirm_times, 0);
    }

    #[test]
    fn test_get_sub_account_deposit_records_response_deserialization() {
        let json = r#"{
            "data": [
                {
                    "subUid": 123456789,
                    "amount": "100.50000000",
                    "coin": "USDT",
                    "network": "TRC20",
                    "status": 1,
                    "address": "TUELLQv4jAQYLnQu7qhLLmv2kfQ6N1nC1s",
                    "addressTag": "memo123",
                    "txId": "0x1234567890abcdef",
                    "insertTime": 1640995200000,
                    "transferType": 0,
                    "unlockConfirmTimes": 12,
                    "confirmTimes": 12
                },
                {
                    "subUid": 987654321,
                    "amount": "0.001",
                    "coin": "BTC",
                    "network": "BTC",
                    "status": 0,
                    "address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
                    "txId": "0xabcdef1234567890",
                    "insertTime": 1640995300000,
                    "transferType": 0,
                    "unlockConfirmTimes": 6,
                    "confirmTimes": 3
                }
            ],
            "total": 2
        }"#;

        let response: GetSubAccountDepositRecordsResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.total, 2);
        assert_eq!(response.data.len(), 2);

        let first_record = &response.data[0];
        assert_eq!(first_record.sub_uid, 123456789);
        assert_eq!(first_record.amount, Some("100.50000000".to_string()));
        assert_eq!(first_record.coin, "USDT");
        assert_eq!(first_record.status, 1);
        assert_eq!(first_record.confirm_times, 12);

        let second_record = &response.data[1];
        assert_eq!(second_record.sub_uid, 987654321);
        assert_eq!(second_record.amount, Some("0.001".to_string()));
        assert_eq!(second_record.coin, "BTC");
        assert_eq!(second_record.status, 0);
        assert_eq!(second_record.confirm_times, 3);
    }

    #[test]
    fn test_deposit_record_status_values() {
        // Test different status values
        let test_cases = vec![(0, "In progress"), (1, "Completed"), (6, "Chain uploaded")];

        for (status_code, _description) in test_cases {
            let json = format!(
                r#"{{
                "subUid": 123456789,
                "amount": "100.0",
                "coin": "TEST",
                "network": "TEST",
                "status": {},
                "address": "test_address",
                "txId": "test_tx",
                "insertTime": 1640995200000,
                "transferType": 0,
                "unlockConfirmTimes": 1,
                "confirmTimes": 1
            }}"#,
                status_code
            );

            let record: DepositRecord = serde_json::from_str(&json).unwrap();
            assert_eq!(record.status, status_code);
        }
    }

    #[test]
    fn test_get_sub_account_deposit_records_response_empty() {
        let json = r#"{"data": [], "total": 0}"#;
        let response: GetSubAccountDepositRecordsResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.total, 0);
        assert_eq!(response.data.len(), 0);
    }
}
