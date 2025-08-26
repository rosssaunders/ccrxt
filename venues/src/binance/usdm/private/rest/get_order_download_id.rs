use serde::{Deserialize, Serialize};

use crate::binance::usdm::PrivateRestClient as RestClient;
use crate::binance::usdm::RestResult;

const GET_ORDER_DOWNLOAD_ID_ENDPOINT: &str = "/fapi/v1/order/asyn";
const GET_ORDER_DOWNLOAD_LINK_BY_ID_ENDPOINT: &str = "/fapi/v1/order/asyn/id";

/// Request parameters for retrieving a download ID for order history.
///
/// The time between start_time and end_time cannot be longer than 1 year.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderDownloadIdRequest {
    /// Start time for filtering orders (milliseconds since epoch).
    pub start_time: u64,

    /// End time for filtering orders (milliseconds since epoch).
    pub end_time: u64,

    /// Receive window for request validity (optional, default 5000ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Request parameters for retrieving the download link by download ID.
///
/// Used to fetch the download link for a previously requested order history download.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderDownloadLinkByIdRequest {
    /// Download ID obtained from the download ID endpoint.
    pub download_id: String,

    /// Receive window for request validity (optional, default 5000ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp (milliseconds since epoch).
    pub timestamp: u64,
}

/// Response from order download ID endpoint.
///
/// Contains the download ID and average processing time information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderDownloadIdResponse {
    /// Average time taken for data download in the past 30 days.
    pub avg_cost_timestamp: String,

    /// Download ID to be used for retrieving the download link.
    pub download_id: String,
}

/// Status of the download link request.
///
/// Enum for the status field returned by the download link endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DownloadStatus {
    /// The download is completed and the link is available.
    Completed,
    /// The download is still processing.
    Processing,
}

/// Response from the download link by ID endpoint.
///
/// Contains the download link and related metadata.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderDownloadLinkByIdResponse {
    /// Download ID used for this request.
    pub download_id: String,

    /// Status of the download request (completed or processing).
    pub status: DownloadStatus,

    /// The download link URL (empty if processing).
    pub url: String,

    /// Whether the user was notified (ignored).
    pub notified: bool,

    /// Expiration timestamp for the download link (milliseconds since epoch).
    pub expiration_timestamp: i64,

    /// Whether the link is expired (null or bool).
    pub is_expired: Option<bool>,
}

impl RestClient {
    /// Get Download Id For Order History (USER_DATA)
    ///
    /// Get download id for order history.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Order-History)
    ///
    /// Rate limit: 5
    ///
    /// # Arguments
    /// * `request` - The download ID request parameters
    ///
    /// # Returns
    /// Download ID and estimated processing time information
    pub async fn get_order_download_id(
        &self,
        request: GetOrderDownloadIdRequest,
    ) -> RestResult<GetOrderDownloadIdResponse> {
        self.send_get_signed_request(GET_ORDER_DOWNLOAD_ID_ENDPOINT, request, 5, false)
            .await
    }

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
    pub async fn get_order_download_link_by_id(
        &self,
        request: GetOrderDownloadLinkByIdRequest,
    ) -> RestResult<GetOrderDownloadLinkByIdResponse> {
        self.send_get_signed_request(GET_ORDER_DOWNLOAD_LINK_BY_ID_ENDPOINT, request, 10, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_order_download_id_request_serialization() {
        let request = GetOrderDownloadIdRequest {
            start_time: 1625097600000,
            end_time: 1625184000000,
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_get_order_download_id_request_serialization_minimal() {
        let request = GetOrderDownloadIdRequest {
            start_time: 1625097600000,
            end_time: 1625184000000,
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_get_order_download_id_response_deserialization() {
        let json = r#"{
            "avgCostTimestamp": "946684800000",
            "downloadId": "download123456"
        }"#;

        let response: GetOrderDownloadIdResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.avg_cost_timestamp, "946684800000");
        assert_eq!(response.download_id, "download123456");
    }

    #[test]
    fn test_get_order_download_link_by_id_request_serialization() {
        let request = GetOrderDownloadLinkByIdRequest {
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
    fn test_get_order_download_link_by_id_response_deserialization_completed() {
        let json = r#"{
            "downloadId": "545923594199212032",
            "status": "completed",
            "url": "www.binance.com",
            "notified": true,
            "expirationTimestamp": 1645009771000,
            "isExpired": null
        }"#;
        let response: GetOrderDownloadLinkByIdResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.download_id, "545923594199212032");
        assert_eq!(response.status, DownloadStatus::Completed);
        assert_eq!(response.url, "www.binance.com");
        assert!(response.notified);
        assert_eq!(response.expiration_timestamp, 1645009771000);
        assert_eq!(response.is_expired, None);
    }

    #[test]
    fn test_get_order_download_link_by_id_response_deserialization_processing() {
        let json = r#"{
            "downloadId": "545923594199212032",
            "status": "processing",
            "url": "",
            "notified": false,
            "expirationTimestamp": -1,
            "isExpired": null
        }"#;
        let response: GetOrderDownloadLinkByIdResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.download_id, "545923594199212032");
        assert_eq!(response.status, DownloadStatus::Processing);
        assert_eq!(response.url, "");
        assert!(!response.notified);
        assert_eq!(response.expiration_timestamp, -1);
        assert_eq!(response.is_expired, None);
    }
}
