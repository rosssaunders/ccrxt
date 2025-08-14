use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult, WithdrawStatus};

const WITHDRAW_RECORDS_ENDPOINT: &str = "/openApi/wallets/v1/capital/withdraw/history";

/// Request for getting withdrawal records
#[derive(Debug, Clone, Serialize)]
pub struct GetWithdrawRecordsRequest {
    /// Unique ID of the withdrawal record (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Coin name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Custom withdrawal ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdraw_order_id: Option<String>,
    /// Status filter (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WithdrawStatus>,
    /// Starting time (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// End time (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Offset (optional, default 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Page size (optional, default 1000, max 1000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Execution window time, cannot be greater than 60000 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
    /// Request timestamp in milliseconds
    pub timestamp: i64,
}

/// Withdrawal record information
#[derive(Debug, Clone, Deserialize)]
pub struct WithdrawRecord {
    /// Withdrawal address
    pub address: String,
    /// Withdrawal amount
    pub amount: String,
    /// Application time
    #[serde(rename = "applyTime")]
    pub apply_time: String,
    /// Coin name
    pub coin: String,
    /// ID of the withdrawal
    pub id: String,
    /// Custom withdrawal order ID
    #[serde(rename = "withdrawOrderId")]
    pub withdraw_order_id: Option<String>,
    /// Withdrawal network
    pub network: String,
    /// Status: 4-Under Review, 5-Failed, 6-Completed
    pub status: WithdrawStatus,
    /// Transaction fee
    #[serde(rename = "transactionFee")]
    pub transaction_fee: String,
    /// Withdrawal confirmation times
    #[serde(rename = "confirmNo")]
    pub confirm_no: i32,

    /// Reason for withdrawal failure (if any)
    pub info: Option<String>,

    /// Withdrawal transaction ID
    #[serde(rename = "txId")]
    pub tx_id: Option<String>,

    /// Source address
    #[serde(rename = "sourceAddress")]
    pub source_address: Option<String>,

    /// Transfer type: 1 Withdrawal, 2 Internal transfer
    #[serde(rename = "transferType")]
    pub transfer_type: i32,

    /// Address tag (for some currencies like XRP/XMR)
    #[serde(rename = "addressTag")]
    pub address_tag: Option<String>,
}

/// Response for withdrawal records
#[derive(Debug, Clone, Deserialize)]
pub struct GetWithdrawRecordsResponse {
    /// List of withdrawal records
    pub data: Vec<WithdrawRecord>,
}

impl RestClient {
    /// Get withdrawal records
    ///
    /// Retrieves the withdrawal history for the account.
    ///
    /// # Arguments
    /// * `request` - The withdrawal records request parameters
    ///
    /// # Returns
    /// A result containing the withdrawal records response or an error
    ///
    /// # Rate Limits
    /// - UID rate limit: 10/s
    /// - IP rate limit group 2
    ///
    /// # API Permissions
    /// - Read permission required
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/spot/wallet-api.html#Withdraw%20records)
    pub async fn get_withdraw_records(
        &self,
        request: &GetWithdrawRecordsRequest,
    ) -> RestResult<GetWithdrawRecordsResponse> {
        self.send_get_signed_request(
            WITHDRAW_RECORDS_ENDPOINT,
            request,
            EndpointType::AccountApiGroup2,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_withdraw_records_request_serialization() {
        let request = GetWithdrawRecordsRequest {
            id: Some("12345".to_string()),
            coin: Some("BTC".to_string()),
            withdraw_order_id: Some("custom123".to_string()),
            status: Some(WithdrawStatus::Completed),
            start_time: Some(1658748648396),
            end_time: Some(1658748648396),
            offset: Some(0),
            limit: Some(100),
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("coin=BTC"));
        assert!(serialized.contains("status=6"));
        assert!(serialized.contains("id=12345"));
    }
    #[test]
    fn test_withdraw_record_deserialization() {
        let json = r#"{
            "address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
            "amount": "0.1",
            "applyTime": "2023-07-25 12:30:48",
            "coin": "BTC",
            "id": "12345",
            "withdrawOrderId": "custom123",
            "network": "BTC",
            "status": 6,
            "transactionFee": "0.0005",
            "confirmNo": 6,
            "info": null,
            "txId": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "sourceAddress": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
            "transferType": 1,
            "addressTag": null
        }"#;

        let record: WithdrawRecord =
            serde_json::from_str(json).expect("Failed to deserialize WithdrawRecord");
        assert_eq!(record.address, "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
        assert_eq!(record.amount, "0.1");
        assert_eq!(record.coin, "BTC");
        assert_eq!(record.status, WithdrawStatus::Completed);
        assert_eq!(record.transaction_fee, "0.0005");
        assert_eq!(record.confirm_no, 6);
        assert_eq!(record.withdraw_order_id, Some("custom123".to_string()));
        assert_eq!(record.transfer_type, 1);
    }

    #[test]
    fn test_withdraw_record_with_failure_info() {
        let json = r#"{
            "address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
            "amount": "0.1",
            "applyTime": "2023-07-25 12:30:48",
            "coin": "BTC",
            "id": "12346",
            "withdrawOrderId": null,
            "network": "BTC",
            "status": 5,
            "transactionFee": "0.0005",
            "confirmNo": 0,
            "info": "Insufficient balance",
            "txId": null,
            "sourceAddress": null,
            "transferType": 1,
            "addressTag": null
        }"#;

        let record: WithdrawRecord =
            serde_json::from_str(json).expect("Failed to deserialize WithdrawRecord");
        assert_eq!(record.status, WithdrawStatus::Failed);
        assert_eq!(record.info, Some("Insufficient balance".to_string()));
        assert!(record.tx_id.is_none());
    }

    #[test]
    fn test_withdraw_record_under_review() {
        let json = r#"{
            "address": "addr123",
            "amount": "10.5",
            "applyTime": "2023-07-26 10:00:00",
            "coin": "ETH",
            "id": "12347",
            "withdrawOrderId": "order456",
            "network": "ETH",
            "status": 4,
            "transactionFee": "0.01",
            "confirmNo": 0,
            "info": null,
            "txId": null,
            "sourceAddress": null,
            "transferType": 1,
            "addressTag": null
        }"#;

        let record: WithdrawRecord =
            serde_json::from_str(json).expect("Failed to deserialize WithdrawRecord");
        assert_eq!(record.status, WithdrawStatus::UnderReview);
        assert_eq!(record.coin, "ETH");
        assert_eq!(record.confirm_no, 0);
    }

    #[test]
    fn test_withdraw_record_with_address_tag() {
        let json = r#"{
            "address": "rXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
            "amount": "100",
            "applyTime": "2023-07-27 15:30:00",
            "coin": "XRP",
            "id": "12348",
            "withdrawOrderId": "xrp789",
            "network": "XRP",
            "status": 6,
            "transactionFee": "0.25",
            "confirmNo": 12,
            "info": null,
            "txId": "ABCDEF123456789",
            "sourceAddress": "rYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY",
            "transferType": 1,
            "addressTag": "123456789"
        }"#;

        let record: WithdrawRecord =
            serde_json::from_str(json).expect("Failed to deserialize WithdrawRecord");
        assert_eq!(record.address_tag, Some("123456789".to_string()));
        assert_eq!(record.coin, "XRP");
    }

    #[test]
    fn test_withdraw_records_response() {
        let json = r#"{
            "data": [
                {
                    "address": "addr1",
                    "amount": "1.0",
                    "applyTime": "2023-07-25 12:00:00",
                    "coin": "BTC",
                    "id": "1",
                    "withdrawOrderId": null,
                    "network": "BTC",
                    "status": 6,
                    "transactionFee": "0.0005",
                    "confirmNo": 6,
                    "info": null,
                    "txId": "tx1",
                    "sourceAddress": "source1",
                    "transferType": 1,
                    "addressTag": null
                },
                {
                    "address": "addr2",
                    "amount": "2.0",
                    "applyTime": "2023-07-25 13:00:00",
                    "coin": "ETH",
                    "id": "2",
                    "withdrawOrderId": "order2",
                    "network": "ETH",
                    "status": 4,
                    "transactionFee": "0.01",
                    "confirmNo": 0,
                    "info": null,
                    "txId": null,
                    "sourceAddress": null,
                    "transferType": 1,
                    "addressTag": null
                }
            ]
        }"#;

        let response: GetWithdrawRecordsResponse =
            serde_json::from_str(json).expect("Failed to deserialize GetWithdrawRecordsResponse");
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].coin, "BTC");
        assert_eq!(response.data[1].coin, "ETH");
        assert_eq!(response.data[1].status, WithdrawStatus::UnderReview);
    }

    #[test]
    fn test_withdraw_record_internal_transfer() {
        let json = r#"{
            "address": "internal_addr",
            "amount": "50.0",
            "applyTime": "2023-07-28 09:00:00",
            "coin": "USDT",
            "id": "12349",
            "withdrawOrderId": "internal123",
            "network": "TRC20",
            "status": 6,
            "transactionFee": "0",
            "confirmNo": 1,
            "info": null,
            "txId": "internal_tx_123",
            "sourceAddress": "source_internal",
            "transferType": 2,
            "addressTag": null
        }"#;

        let record: WithdrawRecord =
            serde_json::from_str(json).expect("Failed to deserialize WithdrawRecord");
        assert_eq!(record.transfer_type, 2); // Internal transfer
        assert_eq!(record.transaction_fee, "0");
    }
}
