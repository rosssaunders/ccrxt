use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::{DepositStatus, EndpointType, RestResult};

const DEPOSIT_RECORDS_ENDPOINT: &str = "/openApi/wallets/v1/capital/deposit/hisrec";

/// Request for getting deposit records
#[derive(Debug, Clone, Serialize)]
pub struct GetDepositRecordsRequest {
    /// Coin name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Status filter (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DepositStatus>,
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

/// Deposit record information
#[derive(Debug, Clone, Deserialize)]
pub struct DepositRecord {
    /// Recharge amount
    pub amount: String,
    /// Coin name
    pub coin: String,
    /// Recharge network
    pub network: String,
    /// Status: 0-In progress, 6-Chain uploaded, 1-Completed
    pub status: DepositStatus,
    /// Recharge address
    pub address: String,
    /// Remark/tag
    #[serde(rename = "addressTag")]
    pub address_tag: Option<String>,
    /// Transaction ID
    #[serde(rename = "txId")]
    pub tx_id: String,
    /// Transaction time
    #[serde(rename = "insertTime")]
    pub insert_time: i64,
    /// Transaction type: 0 = Recharge
    #[serde(rename = "transferType")]
    pub transfer_type: i32,
    /// Confirm times for unlocking
    #[serde(rename = "unlockConfirm")]
    pub unlock_confirm: i32,
    /// Network confirmation times
    #[serde(rename = "confirmTimes")]
    pub confirm_times: i32,
    /// Source address
    #[serde(rename = "sourceAddress")]
    pub source_address: String,
}

/// Response for deposit records
#[derive(Debug, Clone, Deserialize)]
pub struct GetDepositRecordsResponse {
    /// List of deposit records
    pub data: Vec<DepositRecord>,
}

impl RestClient {
    /// Get deposit records
    ///
    /// Retrieves the deposit history for the account.
    ///
    /// # Arguments
    /// * `request` - The deposit records request parameters
    ///
    /// # Returns
    /// A result containing the deposit records response or an error
    ///
    /// # Rate Limits
    /// - UID rate limit: 10/s
    /// - IP rate limit group 2
    ///
    /// # API Permissions
    /// - Read permission required
    pub async fn get_deposit_records(
        &self,
        request: &GetDepositRecordsRequest,
    ) -> RestResult<GetDepositRecordsResponse> {
        self.send_request(
            DEPOSIT_RECORDS_ENDPOINT,
            reqwest::Method::GET,
            Some(request),
            EndpointType::AccountApiGroup2,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit_records_request_serialization() {
        let request = GetDepositRecordsRequest {
            coin: Some("BTC".to_string()),
            status: Some(DepositStatus::Completed),
            start_time: Some(1658748648396),
            end_time: Some(1658748648396),
            offset: Some(0),
            limit: Some(100),
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("coin=BTC"));
        assert!(serialized.contains("status=1"));
        assert!(serialized.contains("start_time=1658748648396"));
        assert!(serialized.contains("timestamp=1640995200000"));
    }

    #[test]
    fn test_deposit_record_deserialization() {
        let json = r#"{
            "amount": "0.1",
            "coin": "BTC",
            "network": "BTC",
            "status": 1,
            "address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
            "addressTag": "",
            "txId": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "insertTime": 1658748648396,
            "transferType": 0,
            "unlockConfirm": 1,
            "confirmTimes": 6,
            "sourceAddress": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"
        }"#;

        let record: DepositRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.amount, "0.1");
        assert_eq!(record.coin, "BTC");
        assert_eq!(record.status, DepositStatus::Completed);
    }
}
