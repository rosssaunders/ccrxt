use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, PrivateRestClient as RestClient, RestResult};

const ASSET_TRANSFER_RECORDS_NEW_ENDPOINT: &str = "/openApi/api/v3/asset/transferRecord";

/// Request for new asset transfer records
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetTransferRecordsNewRequest {
    /// From account (optional)
    /// fund: Funding Account, spot: Spot Account, stdFutures: Standard Contract,
    /// coinMPerp: COIN-M Perpetual Future, USDTMPerp: Perpetual Future
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_account: Option<String>,

    /// To account (required)  
    /// fund: Funding Account, spot: Spot Account, stdFutures: Standard Contract,
    /// coinMPerp: COIN-M Perpetual Future, USDTMPerp: Perpetual Future
    pub to_account: String,

    /// Transaction ID (optional, query by fromAccount/toAccount or transferId)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<String>,

    /// Starting time (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,

    /// End time (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,

    /// Current page (optional, default 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_index: Option<i32>,

    /// Page size (optional, default 10, max 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,

    /// Execution window time, cannot be greater than 60000 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Current timestamp (required)
    pub timestamp: i64,
}

/// New asset transfer record
#[derive(Debug, Clone, Deserialize)]
pub struct AssetTransferRecordNew {
    /// Transfer ID
    #[serde(rename = "transferId")]
    pub transfer_id: String,

    /// Asset/coin name
    pub asset: String,

    /// Transfer amount
    pub amount: String,

    /// From account
    /// fund: Funding Account, spot: Spot Account, stdFutures: Standard Contract,
    /// coinMPerp: COIN-M Perpetual Future, USDTMPerp: Perpetual Future
    #[serde(rename = "fromAccount")]
    pub from_account: String,

    /// To account  
    /// fund: Funding Account, spot: Spot Account, stdFutures: Standard Contract,
    /// coinMPerp: COIN-M Perpetual Future, USDTMPerp: Perpetual Future
    #[serde(rename = "toAccount")]
    pub to_account: String,

    /// Transfer timestamp
    pub timestamp: i64,
}

/// Response for new asset transfer records
pub type GetAssetTransferRecordsNewResponse = Vec<AssetTransferRecordNew>;

impl RestClient {
    /// Asset transfer records new
    ///
    /// Retrieves the history of asset transfers between accounts using the new API.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/spot/account-api.html#Asset%20transfer%20records%20new)
    ///
    /// Rate limit: UID 2/s & IP rate limit group 2
    ///
    /// # Arguments
    /// * `request` - The asset transfer records request parameters
    ///
    /// # Returns
    /// A result containing the asset transfer records response or an error
    pub async fn get_asset_transfer_records_new(
        &self,
        request: &GetAssetTransferRecordsNewRequest,
    ) -> RestResult<GetAssetTransferRecordsNewResponse> {
        self.send_get_signed_request(
            ASSET_TRANSFER_RECORDS_NEW_ENDPOINT,
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
    fn test_asset_transfer_records_new_request_serialization() {
        let request = GetAssetTransferRecordsNewRequest {
            from_account: Some("fund".to_string()),
            to_account: "spot".to_string(),
            transfer_id: None,
            start_time: Some(1658748648396),
            end_time: Some(1658748648496),
            page_index: Some(1),
            page_size: Some(50),
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("fromAccount=fund"));
        assert!(serialized.contains("toAccount=spot"));
        assert!(serialized.contains("startTime=1658748648396"));
        assert!(serialized.contains("endTime=1658748648496"));
        assert!(serialized.contains("pageIndex=1"));
        assert!(serialized.contains("pageSize=50"));
        assert!(serialized.contains("timestamp=1640995200000"));
    }

    #[test]
    fn test_asset_transfer_records_new_minimal_request() {
        let request = GetAssetTransferRecordsNewRequest {
            from_account: None,
            to_account: "spot".to_string(),
            transfer_id: None,
            start_time: None,
            end_time: None,
            page_index: None,
            page_size: None,
            recv_window: None,
            timestamp: 1640995200000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("toAccount=spot"));
        assert!(serialized.contains("timestamp=1640995200000"));
        assert!(!serialized.contains("fromAccount"));
        assert!(!serialized.contains("transferId"));
    }

    #[test]
    fn test_asset_transfer_record_new_deserialization() {
        let json = r#"[
            {
                "transferId": "12345",
                "asset": "USDT",
                "amount": "100.50",
                "fromAccount": "fund",
                "toAccount": "spot",
                "timestamp": 1658748648396
            },
            {
                "transferId": "12346", 
                "asset": "BTC",
                "amount": "0.001",
                "fromAccount": "spot",
                "toAccount": "USDTMPerp",
                "timestamp": 1658748648496
            }
        ]"#;

        let response: GetAssetTransferRecordsNewResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);

        let first_record = &response[0];
        assert_eq!(first_record.transfer_id, "12345");
        assert_eq!(first_record.asset, "USDT");
        assert_eq!(first_record.amount, "100.50");
        assert_eq!(first_record.from_account, "fund");
        assert_eq!(first_record.to_account, "spot");
        assert_eq!(first_record.timestamp, 1658748648396);

        let second_record = &response[1];
        assert_eq!(second_record.transfer_id, "12346");
        assert_eq!(second_record.asset, "BTC");
        assert_eq!(second_record.amount, "0.001");
        assert_eq!(second_record.from_account, "spot");
        assert_eq!(second_record.to_account, "USDTMPerp");
        assert_eq!(second_record.timestamp, 1658748648496);
    }
}
