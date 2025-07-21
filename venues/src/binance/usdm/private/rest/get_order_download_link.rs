use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const GET_ORDER_DOWNLOAD_LINK_ENDPOINT: &str = "/fapi/v1/order/asyn/id";

/// Request for getting order download link.
///
/// Parameters for retrieving the download link using a previously obtained download ID.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderDownloadLinkRequest {
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

/// Response from order download link endpoint.
///
/// Contains the download link and status information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderDownloadLinkResponse {
    /// Download link (available when status is completed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_link: Option<String>,

    /// Current status of the download.
    pub status: DownloadStatus,
}

impl UsdmClient {
    /// Get Order History Download Link by Id (USER_DATA)
    ///
    /// Get order history download link by Id
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Order-History-Download-Link-by-Id
    ///
    /// Rate limit: 5
    ///
    /// # Arguments
    /// * `request` - The download link request parameters
    ///
    /// # Returns
    /// Download link information including status and URL
    pub async fn get_order_download_link(
        &self,
        request: GetOrderDownloadLinkRequest,
    ) -> RestResult<GetOrderDownloadLinkResponse> {
        self.send_signed_request(
            GET_ORDER_DOWNLOAD_LINK_ENDPOINT,
            Method::GET,
            request,
            5,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_order_download_link_request_serialization() {
        let request = GetOrderDownloadLinkRequest {
            download_id: "download123456".to_string(),
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("downloadId=download123456"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_get_order_download_link_request_serialization_minimal() {
        let request = GetOrderDownloadLinkRequest {
            download_id: "download123456".to_string(),
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("downloadId=download123456"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_get_order_download_link_response_deserialization_completed() {
        let json = r#"{
            "downloadLink": "https://bin-prod-user-rebate-bucket.s3.amazonaws.com/...",
            "status": "completed"
        }"#;

        let response: GetOrderDownloadLinkResponse = serde_json::from_str(json).unwrap();
        assert!(response.download_link.is_some());
        assert_eq!(
            response.download_link.unwrap(),
            "https://bin-prod-user-rebate-bucket.s3.amazonaws.com/..."
        );
        assert_eq!(response.status, DownloadStatus::Completed);
    }

    #[test]
    fn test_get_order_download_link_response_deserialization_processing() {
        let json = r#"{
            "status": "processing"
        }"#;

        let response: GetOrderDownloadLinkResponse = serde_json::from_str(json).unwrap();
        assert!(response.download_link.is_none());
        assert_eq!(response.status, DownloadStatus::Processing);
    }

    #[test]
    fn test_get_order_download_link_response_deserialization_failed() {
        let json = r#"{
            "status": "failed"
        }"#;

        let response: GetOrderDownloadLinkResponse = serde_json::from_str(json).unwrap();
        assert!(response.download_link.is_none());
        assert_eq!(response.status, DownloadStatus::Failed);
    }
}
