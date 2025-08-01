use serde::{Deserialize, Serialize};

use crate::bingx::spot::{EndpointType, RestResult, TransferType};

use super::RestClient;

const ASSET_TRANSFER_ENDPOINT: &str = "/openApi/api/v3/post/asset/transfer";

/// Request for asset transfer
#[derive(Debug, Clone, Serialize)]
pub struct AssetTransferRequest {
    /// Transfer type
    #[serde(rename = "type")]
    pub transfer_type: TransferType,

    /// Asset name (e.g., USDT)
    pub asset: String,

    /// Amount to transfer
    pub amount: f64,

    /// Execution window time, cannot be greater than 60000 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Timestamp for this request
    pub timestamp: i64,
}

/// Response for asset transfer
#[derive(Debug, Clone, Deserialize)]
pub struct AssetTransferResponse {
    /// Transaction ID
    #[serde(rename = "tranId")]
    pub tran_id: i64,
}

impl RestClient {
    /// Transfer assets between accounts
    ///
    /// Transfers assets between different account types (fund, futures, etc.).
    ///
    /// [docs]: https://bingx-api.github.io/docs/#/en-us/common/account-api.html#Asset%20Transfer
    ///
    /// Rate limit: UID 2/s, IP rate limit group 3
    ///
    /// # Arguments
    /// * `request` - The asset transfer request parameters
    ///
    /// # Returns
    /// A result containing the asset transfer response or an error
    pub async fn asset_transfer(
        &self,
        request: AssetTransferRequest,
    ) -> RestResult<AssetTransferResponse> {
        self.send_post_signed_request(ASSET_TRANSFER_ENDPOINT, &request, EndpointType::AccountApiGroup3,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_transfer_request_serialization() {
        let request = AssetTransferRequest {
            transfer_type: TransferType::FundToPerpetualFutures,
            asset: "USDT".to_string(),
            amount: 100.0,
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("type=FUND_PFUTURES"));
        assert!(serialized.contains("asset=USDT"));
        assert!(serialized.contains("amount=100"));
        assert!(serialized.contains("timestamp=1640995200000"));
    }

    #[test]
    fn test_asset_transfer_response_deserialization() {
        let json = r#"{
            "tranId": 12345
        }"#;

        let response: AssetTransferResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.tran_id, 12345);
    }
}
