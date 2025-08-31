use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::binance::usdm::{RestResult, private_client::UsdmClient};

/// Endpoint path for getting the futures transaction history download link by ID.
const GET_INCOME_DOWNLOAD_LINK_ENDPOINT: &str = "/fapi/v1/income/asyn/id";

/// Request parameters for the Get Futures Transaction History Download Link by Id endpoint.
///
/// Used to retrieve the download link for a previously requested futures transaction history file.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetIncomeDownloadLinkRequest {
    /// Download ID obtained from the previous download ID request.
    /// Must match the value returned by the download ID endpoint.
    pub download_id: Cow<'static, str>,

    /// Request timestamp (milliseconds since epoch).
    /// Required by Binance API for all signed requests.
    pub timestamp: u64,

    /// Receive window for request validity (optional, default 5000ms).
    /// If not provided, Binance will use the default value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Status of the download link preparation.
///
/// Represents the current state of the download request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DownloadStatus {
    /// Download is ready and link is available.
    Completed,

    /// Download is still being processed.
    Processing,

    /// Download processing has failed.
    Failed,
}

/// Response for the Get Futures Transaction History Download Link by Id endpoint.
///
/// Contains the download link and status information for the requested file.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIncomeDownloadLinkResponse {
    /// Download ID for tracking.
    /// Matches the request download ID.
    pub download_id: Cow<'static, str>,

    /// Current status of the download.
    /// Indicates whether the file is ready, processing, or failed.
    pub status: DownloadStatus,

    /// Download URL (available when status is completed).
    /// Will be None or empty if not completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'static, str>>,

    /// Notification flag (can be ignored).
    pub notified: bool,

    /// Link expiration timestamp (milliseconds since epoch).
    /// Set to -1 when link is not available.
    pub expiration_timestamp: i64,

    /// Whether the link has expired.
    /// None if not applicable.
    pub is_expired: Option<bool>,
}

impl UsdmClient {
    /// Get Futures Transaction History Download Link by Id (USER_DATA)
    ///
    /// Get futures transaction history download link by Id.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Futures-Transaction-History-Download-Link-by-Id)
    ///
    /// Rate limit: 10
    ///
    /// # Arguments
    /// * `request` - The request parameters for the download link
    ///
    /// # Returns
    /// Download link information including status and URL
    pub async fn get_income_download_link(
        &self,
        request: GetIncomeDownloadLinkRequest,
    ) -> RestResult<GetIncomeDownloadLinkResponse> {
        self.send_get_signed_request(GET_INCOME_DOWNLOAD_LINK_ENDPOINT, request, 10, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;
    use serde_urlencoded;

    use super::*;

    #[test]
    fn test_get_income_download_link_request_serialization() {
        let request = GetIncomeDownloadLinkRequest {
            download_id: Cow::Borrowed("download123456"),
            timestamp: 1650000000000,
            recv_window: Some(5000),
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("downloadId=download123456"));
        assert!(serialized.contains("timestamp=1650000000000"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_get_income_download_link_request_serialization_minimal() {
        let request = GetIncomeDownloadLinkRequest {
            download_id: Cow::Borrowed("download123456"),
            timestamp: 1650000000000,
            recv_window: None,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("downloadId=download123456"));
        assert!(serialized.contains("timestamp=1650000000000"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_get_income_download_link_response_deserialization_completed() {
        let json = r#"{
            "downloadId": "545923594199212032",
            "status": "completed",
            "url": "https://bin-prod-user-rebate-bucket.s3.amazonaws.com/example",
            "notified": true,
            "expirationTimestamp": 1645009771000,
            "isExpired": false
        }"#;
        let response: GetIncomeDownloadLinkResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.download_id, Cow::Borrowed("545923594199212032"));
        assert_eq!(response.status, DownloadStatus::Completed);
        assert!(response.url.is_some());
        assert_eq!(
            response.url.as_deref(),
            Some("https://bin-prod-user-rebate-bucket.s3.amazonaws.com/example")
        );
        assert!(response.notified);
        assert_eq!(response.expiration_timestamp, 1645009771000);
        assert_eq!(response.is_expired, Some(false));
    }

    #[test]
    fn test_get_income_download_link_response_deserialization_processing() {
        let json = r#"{
            "downloadId": "545923594199212032",
            "status": "processing",
            "url": null,
            "notified": false,
            "expirationTimestamp": -1,
            "isExpired": null
        }"#;
        let response: GetIncomeDownloadLinkResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.download_id, Cow::Borrowed("545923594199212032"));
        assert_eq!(response.status, DownloadStatus::Processing);
        assert!(response.url.is_none());
        assert!(!response.notified);
        assert_eq!(response.expiration_timestamp, -1);
        assert_eq!(response.is_expired, None);
    }

    #[test]
    fn test_download_status_serialization() {
        assert_eq!(
            serde_json::to_string(&DownloadStatus::Completed).unwrap(),
            "\"completed\""
        );
        assert_eq!(
            serde_json::to_string(&DownloadStatus::Processing).unwrap(),
            "\"processing\""
        );
        assert_eq!(
            serde_json::to_string(&DownloadStatus::Failed).unwrap(),
            "\"failed\""
        );
    }

    #[test]
    fn test_download_status_deserialization() {
        let completed: DownloadStatus = serde_json::from_str("\"completed\"").unwrap();
        assert_eq!(completed, DownloadStatus::Completed);
        let processing: DownloadStatus = serde_json::from_str("\"processing\"").unwrap();
        assert_eq!(processing, DownloadStatus::Processing);
        let failed: DownloadStatus = serde_json::from_str("\"failed\"").unwrap();
        assert_eq!(failed, DownloadStatus::Failed);
    }
}
