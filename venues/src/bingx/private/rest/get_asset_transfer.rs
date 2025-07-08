use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::{EndpointType, RestResult, TransferType};

const ASSET_TRANSFER_RECORDS_ENDPOINT: &str = "/openApi/api/v3/asset/transfer";

/// Request for asset transfer records
#[derive(Debug, Clone, Serialize)]
pub struct GetAssetTransferRecordsRequest {
    /// Transfer type
    #[serde(rename = "type")]
    pub transfer_type: TransferType,
    /// Transaction ID (optional, used with transfer_type for query)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tran_id: Option<i64>,
    /// Starting time (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// End time (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Current page (optional, default 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<i32>,
    /// Page size (optional, default 10, max 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// Execution window time, cannot be greater than 60000 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Current timestamp (required)
    pub timestamp: i64,
}

/// Asset transfer record
#[derive(Debug, Clone, Deserialize)]
pub struct AssetTransferRecord {
    /// Asset name
    pub asset: String,
    /// Transfer amount
    pub amount: String,
    /// Transfer type
    #[serde(rename = "type")]
    pub transfer_type: TransferType,
    /// Status (always "CONFIRMED")
    pub status: String,
    /// Transaction ID
    #[serde(rename = "tranId")]
    pub tran_id: i64,
    /// Transfer timestamp
    pub timestamp: i64,
}

/// Response for asset transfer records
#[derive(Debug, Clone, Deserialize)]
pub struct GetAssetTransferRecordsResponse {
    /// Total count
    pub total: i64,
    /// Transfer records
    pub rows: Vec<AssetTransferRecord>,
}

impl RestClient {
    /// Get asset transfer records
    ///
    /// Retrieves the history of asset transfers between accounts.
    ///
    /// # Arguments
    /// * `request` - The asset transfer records request parameters
    ///
    /// # Returns
    /// A result containing the asset transfer records response or an error
    ///
    /// # Rate Limits
    /// - UID rate limit: 10/s
    /// - IP rate limit group 2
    ///
    /// # API Permissions
    /// - Read permission required
    pub async fn get_asset_transfer_records(
        &self,
        request: &GetAssetTransferRecordsRequest,
    ) -> RestResult<GetAssetTransferRecordsResponse> {
        self.send_request(
            ASSET_TRANSFER_RECORDS_ENDPOINT,
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
    fn test_asset_transfer_records_request_serialization() {
        let request = GetAssetTransferRecordsRequest {
            transfer_type: TransferType::FundToPerpetualFutures,
            tran_id: Some(12345),
            start_time: Some(1658748648396),
            end_time: Some(1658748648396),
            current: Some(1),
            size: Some(10),
            recv_window: Some(5000),
            timestamp: 1658748648396,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("type=FUND_PFUTURES"));
        assert!(serialized.contains("tran_id=12345"));
        assert!(serialized.contains("current=1"));
    }

    #[test]
    fn test_asset_transfer_record_deserialization() {
        let json = r#"{
            "asset": "USDT",
            "amount": "100.0",
            "type": "FUND_PFUTURES",
            "status": "CONFIRMED",
            "tranId": 12345,
            "timestamp": 1658748648396
        }"#;

        let record: AssetTransferRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.asset, "USDT");
        assert_eq!(record.amount, "100.0");
        assert_eq!(record.transfer_type, TransferType::FundToPerpetualFutures);
        assert_eq!(record.status, "CONFIRMED");
    }
}
