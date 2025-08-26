// No top-level comments per project instructions.

use serde::{Deserialize, Serialize};

use crate::binance::usdm::PrivateRestClient as RestClient;
use crate::binance::usdm::RestResult;

/// Endpoint path for Get Futures Trade Download Link by Id.
const TRADE_DOWNLOAD_LINK_ENDPOINT: &str = "/fapi/v1/trade/asyn/id";

/// Request parameters for the Get Futures Trade Download Link by Id endpoint.
///
/// This struct contains the parameters required to retrieve a download link for futures trade history.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTradeDownloadLinkRequest {
    /// The download ID obtained from the previous request.
    /// Must be a valid string as returned by the download ID API.
    pub download_id: String,

    /// Timestamp in milliseconds since epoch.
    /// Used for request authentication and replay protection.
    pub timestamp: u64,

    /// Optional signature for the request. Required for signed endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// Status of the trade download link request.
///
/// Represents the current state of the download link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DownloadStatus {
    /// The download link is ready and available.
    Completed,

    /// The download link is still being generated.
    Processing,

    /// The download link generation failed.
    Failed,
}

/// Response from the Get Futures Trade Download Link by Id endpoint.
///
/// Contains the download link and status information for the requested trade history.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeDownloadLinkResponse {
    /// The download link for the trade history file.
    /// Only present when status is `completed`. The link expires after 24 hours.
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_link: Option<String>,

    /// The status of the download link request.
    /// Possible values: completed, processing, failed.
    pub status: DownloadStatus,

    /// The download ID associated with this request.
    pub download_id: String,

    /// Expiration timestamp for the download link (milliseconds since epoch).
    pub expiration_timestamp: Option<u64>,

    /// Whether the link has expired. May be null in some responses.
    pub is_expired: Option<bool>,
}

impl RestClient {
    /// Get Futures Trade Download Link by Id
    ///
    /// Retrieves the download link for futures trade history using the previously obtained download ID.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Futures-Trade-Download-Link-by-Id)
    ///
    /// Rate limit: 10 requests per minute
    ///
    /// # Arguments
    /// * `request` - The request parameters for the download link endpoint
    ///
    /// # Returns
    /// `TradeDownloadLinkResponse` containing the download link and status information
    pub async fn get_trade_download_link(
        &self,
        request: GetTradeDownloadLinkRequest,
    ) -> RestResult<TradeDownloadLinkResponse> {
        self.send_get_signed_request(TRADE_DOWNLOAD_LINK_ENDPOINT, request, 10, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_download_status_deserialize() {
        let completed = serde_json::from_str::<DownloadStatus>(r#""completed""#).unwrap();
        let processing = serde_json::from_str::<DownloadStatus>(r#""processing""#).unwrap();
        let failed = serde_json::from_str::<DownloadStatus>(r#""failed""#).unwrap();
        assert_eq!(completed, DownloadStatus::Completed);
        assert_eq!(processing, DownloadStatus::Processing);
        assert_eq!(failed, DownloadStatus::Failed);
    }

    #[test]
    fn test_trade_download_link_response_deserialize_completed() {
        let data = json!({
            "url": "https://binance.com/download/abc123",
            "status": "completed",
            "downloadId": "545923594199212032",
            "expirationTimestamp": 1645009771000u64,
            "isExpired": false
        });
        let resp: TradeDownloadLinkResponse = serde_json::from_value(data).unwrap();
        assert_eq!(
            resp.download_link.as_deref(),
            Some("https://binance.com/download/abc123")
        );
        assert_eq!(resp.status, DownloadStatus::Completed);
        assert_eq!(resp.download_id, "545923594199212032");
        assert_eq!(resp.expiration_timestamp, Some(1645009771000u64));
        assert_eq!(resp.is_expired, Some(false));
    }

    #[test]
    fn test_trade_download_link_response_deserialize_processing() {
        let data = json!({
            "url": "",
            "status": "processing",
            "downloadId": "545923594199212032",
            "expirationTimestamp": null,
            "isExpired": null
        });
        let resp: TradeDownloadLinkResponse = serde_json::from_value(data).unwrap();
        assert_eq!(resp.download_link, Some(String::new()));
        assert_eq!(resp.status, DownloadStatus::Processing);
        assert_eq!(resp.download_id, "545923594199212032");
        assert_eq!(resp.expiration_timestamp, None);
        assert_eq!(resp.is_expired, None);
    }
}
