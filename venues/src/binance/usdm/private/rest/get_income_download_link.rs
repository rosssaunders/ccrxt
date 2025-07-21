use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const GET_INCOME_DOWNLOAD_LINK_ENDPOINT: &str = "/fapi/v1/income/asyn/id";

/// Request for getting income download link.
///
/// Parameters for retrieving the download link using a previously obtained download ID.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetIncomeDownloadLinkRequest {
    /// Download ID from the previous get download ID request.
    pub download_id: String,

    /// Receive window for request validity (optional, default 5000ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Download status enumeration.
///
/// Represents the current status of the download preparation.
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

/// Response from income download link endpoint.
///
/// Contains the download link and status information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIncomeDownloadLinkResponse {
    /// Download ID for tracking.
    pub download_id: String,

    /// Current status of the download.
    pub status: DownloadStatus,

    /// Download URL (available when status is completed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Notification flag (can be ignored).
    pub notified: bool,

    /// Link expiration timestamp (milliseconds since epoch).
    /// Set to -1 when link is not available.
    pub expiration_timestamp: i64,

    /// Whether the link has expired.
    pub is_expired: Option<bool>,
}

impl UsdmClient {
    /// Get Futures Transaction History Download Link by Id (USER_DATA)
    ///
    /// Get futures transaction history download link by Id
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Futures-Transaction-History-Download-Link-by-Id
    ///
    /// Rate limit: 10
    ///
    /// # Arguments
    /// * `request` - The download link request parameters
    ///
    /// # Returns
    /// Download link information including status and URL
    pub async fn get_income_download_link(
        &self,
        request: GetIncomeDownloadLinkRequest,
    ) -> RestResult<GetIncomeDownloadLinkResponse> {
        self.send_signed_request(
            GET_INCOME_DOWNLOAD_LINK_ENDPOINT,
            Method::GET,
            request,
            10,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_income_download_link_request_serialization() {
        let request = GetIncomeDownloadLinkRequest {
            download_id: "download123456".to_string(),
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("downloadId=download123456"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_get_income_download_link_request_serialization_minimal() {
        let request = GetIncomeDownloadLinkRequest {
            download_id: "download123456".to_string(),
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("downloadId=download123456"));
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
        assert_eq!(response.download_id, "545923594199212032");
        assert_eq!(response.status, DownloadStatus::Completed);
        assert!(response.url.is_some());
        assert_eq!(
            response.url.unwrap(),
            "https://bin-prod-user-rebate-bucket.s3.amazonaws.com/example"
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
        assert_eq!(response.download_id, "545923594199212032");
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
