use serde::{Deserialize, Serialize};

use crate::binance::usdm::PrivateRestClient as RestClient;
use crate::binance::usdm::RestResult;

/// Endpoint path for Get Futures Order History Download Link by Id.
const GET_ORDER_DOWNLOAD_LINK_ENDPOINT: &str = "/fapi/v1/order/asyn/id";

/// Status of the download link request.
///
/// Enum for the status field returned by the download link endpoint.
///
/// - "completed": The download is completed and the link is available.
/// - "processing": The download is still processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DownloadStatus {
    /// The download is completed and the link is available.
    Completed,

    /// The download is still processing.
    Processing,
}

/// Request parameters for retrieving the download link by download ID.
///
/// Used to fetch the download link for a previously requested order history download.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderDownloadLinkRequest {
    /// Download ID obtained from the download ID endpoint.
    /// Must be a valid download ID string as returned by the download ID endpoint.
    pub download_id: String,

    /// Receive window for request validity (optional, default 5000ms).
    /// Optional. If not provided, defaults to 5000ms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp (milliseconds since epoch).
    /// Required. Must be the current timestamp in milliseconds.
    pub timestamp: u64,
}

// DownloadStatus enum is defined above, as this endpoint only uses it locally.

/// Response from the download link by ID endpoint.
///
/// Contains the download link and related metadata.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderDownloadLinkResponse {
    /// Download ID used for this request.
    /// Matches the download ID provided in the request.
    pub download_id: String,

    /// Status of the download request.
    /// Enum values: "completed", "processing"
    /// See `DownloadStatus` enum for possible values.
    pub status: DownloadStatus,

    /// The download link URL. Empty if status is `processing`.
    pub url: String,

    /// Whether the user was notified. This field is ignored by the API.
    pub notified: bool,

    /// Expiration timestamp for the download link (milliseconds since epoch).
    /// The link will expire after this timestamp.
    pub expiration_timestamp: i64,

    /// Whether the link is expired. May be null or a boolean.
    pub is_expired: Option<bool>,
}

impl RestClient {
    /// Get Futures Order History Download Link by Id (USER_DATA)
    ///
    /// Retrieves the download link for a previously requested order history download.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Futures-Order-History-Download-Link-by-Id)
    ///
    /// Rate limit: 10
    ///
    /// # Arguments
    /// * `request` - The download link request parameters
    ///
    /// # Returns
    /// Download link and related metadata
    pub async fn get_order_download_link(
        &self,
        request: GetOrderDownloadLinkRequest,
    ) -> RestResult<GetOrderDownloadLinkResponse> {
        self.send_get_signed_request(GET_ORDER_DOWNLOAD_LINK_ENDPOINT, request, 10, false)
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
            timestamp: 1625184000000,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("downloadId=download123456"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625184000000"));
    }

    #[test]
    fn test_get_order_download_link_response_deserialization_completed() {
        let json = r#"{
            "downloadId": "545923594199212032",
            "status": "completed",
            "url": "www.binance.com",
            "notified": true,
            "expirationTimestamp": 1645009771000,
            "isExpired": null
        }"#;
        let response: GetOrderDownloadLinkResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.download_id, "545923594199212032");
        assert_eq!(response.status, DownloadStatus::Completed);
        assert_eq!(response.url, "www.binance.com");
        assert!(response.notified);
        assert_eq!(response.expiration_timestamp, 1645009771000);
        assert_eq!(response.is_expired, None);
    }

    #[test]
    fn test_get_order_download_link_response_deserialization_processing() {
        let json = r#"{
            "downloadId": "545923594199212032",
            "status": "processing",
            "url": "",
            "notified": false,
            "expirationTimestamp": -1,
            "isExpired": null
        }"#;
        let response: GetOrderDownloadLinkResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.download_id, "545923594199212032");
        assert_eq!(response.status, DownloadStatus::Processing);
        assert_eq!(response.url, "");
        assert!(!response.notified);
        assert_eq!(response.expiration_timestamp, -1);
        assert_eq!(response.is_expired, None);
    }
}
