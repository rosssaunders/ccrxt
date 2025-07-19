use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{RestResult, enums::DownloadStatus, private::rest::client::RestClient},
    shared,
};

const INCOME_ASYN_ENDPOINT: &str = "/dapi/v1/income/asyn";
const INCOME_ASYN_ID_ENDPOINT: &str = "/dapi/v1/income/asyn/id";

/// Request parameters for getting download ID for transaction history.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDownloadIdRequest {
    /// Start time in milliseconds
    pub start_time: u64,

    /// End time in milliseconds
    pub end_time: u64,

    /// This parameter cannot be used in combination with other parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Request parameters for getting download link.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDownloadLinkRequest {
    /// Download ID
    pub download_id: String,

    /// This parameter cannot be used in combination with other parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response for getting download ID.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDownloadIdResponse {
    /// Average time in seconds for processing the file
    pub avg_cost_timestamp: String,

    /// Download ID
    pub download_id: String,
}

/// Response for getting download link.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDownloadLinkResponse {
    /// Download ID
    pub download_id: String,

    /// Download status
    pub status: DownloadStatus,

    /// Download URL (only available when status is completed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// The link will expire after this timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_timestamp: Option<u64>,
}

impl RestClient {
    /// Get download ID for transaction history on Binance Coin-M Futures.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Get-Futures-Transaction-History-Download-Link-by-Id
    ///
    /// GET /dapi/v1/income/asyn
    /// Weight: 5
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`GetDownloadIdRequest`])
    ///
    /// # Returns
    /// A [`GetDownloadIdResponse`] object with download ID details.
    pub async fn get_download_id_for_transaction_history(
        &self,
        params: GetDownloadIdRequest,
    ) -> RestResult<GetDownloadIdResponse> {
        let weight = 5;
        shared::send_signed_request(
            self,
            INCOME_ASYN_ENDPOINT,
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }

    /// Get download link for transaction history on Binance Coin-M Futures.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Get-Futures-Transaction-History-Download-Link-by-Id
    ///
    /// GET /dapi/v1/income/asyn/id
    /// Weight: 5
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`GetDownloadLinkRequest`])
    ///
    /// # Returns
    /// A [`GetDownloadLinkResponse`] object with download link details.
    pub async fn get_download_link_for_transaction_history(
        &self,
        params: GetDownloadLinkRequest,
    ) -> RestResult<GetDownloadLinkResponse> {
        let weight = 5;
        shared::send_signed_request(
            self,
            INCOME_ASYN_ID_ENDPOINT,
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_download_id_request_serialization() {
        let request = GetDownloadIdRequest {
            start_time: 1625097600000,
            end_time: 1625184000000,
            recv_window: None,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
    }

    #[test]
    fn test_get_download_id_request_serialization_with_recv_window() {
        let request = GetDownloadIdRequest {
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
    fn test_get_download_link_request_serialization() {
        let request = GetDownloadLinkRequest {
            download_id: "abc123".to_string(),
            recv_window: None,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "downloadId=abc123");
    }

    #[test]
    fn test_get_download_link_request_serialization_with_recv_window() {
        let request = GetDownloadLinkRequest {
            download_id: "abc123".to_string(),
            recv_window: Some(5000),
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("downloadId=abc123"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_get_download_id_response_deserialization() {
        let json = r#"{
            "avgCostTimestamp": "300",
            "downloadId": "123456"
        }"#;
        let response: GetDownloadIdResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.avg_cost_timestamp, "300");
        assert_eq!(response.download_id, "123456");
    }

    #[test]
    fn test_get_download_link_response_deserialization_completed() {
        let json = r#"{
            "downloadId": "123456",
            "status": "completed",
            "url": "https://example.com/download",
            "expiredTimestamp": 1625270400000
        }"#;
        let response: GetDownloadLinkResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.download_id, "123456");
        assert_eq!(response.status, DownloadStatus::Completed);
        assert_eq!(
            response.url,
            Some("https://example.com/download".to_string())
        );
        assert_eq!(response.expired_timestamp, Some(1625270400000));
    }

    #[test]
    fn test_get_download_link_response_deserialization_processing() {
        let json = r#"{
            "downloadId": "123456",
            "status": "processing"
        }"#;
        let response: GetDownloadLinkResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.download_id, "123456");
        assert_eq!(response.status, DownloadStatus::Processing);
        assert_eq!(response.url, None);
        assert_eq!(response.expired_timestamp, None);
    }
}
