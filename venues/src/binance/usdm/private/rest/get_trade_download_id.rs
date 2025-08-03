// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

/// Endpoint path for Get Futures Trade Download Link by Id.
const TRADE_DOWNLOAD_LINK_BY_ID_ENDPOINT: &str = "/fapi/v1/trade/asyn/id";

/// Request parameters for the Get Futures Trade Download Link by Id endpoint.
///
/// All fields must match the Binance API documentation exactly.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTradeDownloadLinkByIdRequest {
    /// Download ID obtained from the previous download ID request.
    /// Must be a valid download ID string as returned by the download ID endpoint.
    pub download_id: Cow<'static, str>,

    /// Optional window of time in milliseconds for the request to be valid.
    /// If not provided, Binance default is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds since epoch.
    /// Must be the current system time.
    pub timestamp: u64,
}

/// Status of the download link request.
///
/// Enum values match the Binance API exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TradeDownloadStatus {
    /// The download link is ready and completed.
    Completed,

    /// The download link is still being processed.
    Processing,
}

/// Response from the Get Futures Trade Download Link by Id endpoint.
///
/// All fields are mapped directly from the Binance API response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeDownloadLinkByIdResponse {
    /// Download ID for the trade history file.
    /// This matches the download ID provided in the request.
    pub download_id: Cow<'static, str>,

    /// Status of the download link request. Enum: "completed" or "processing".
    pub status: TradeDownloadStatus,

    /// Download URL mapped to the download ID. Empty if still processing.
    pub url: Cow<'static, str>,

    /// Whether the user has been notified. Ignored by client.
    pub notified: bool,

    /// Expiration timestamp for the download link (milliseconds since epoch).
    /// The link will expire after this timestamp.
    pub expiration_timestamp: u64,

    /// Whether the link is expired. May be null if not determined.
    pub is_expired: Option<bool>,
}

impl UsdmClient {
    /// Get Futures Trade Download Link by Id
    ///
    /// Retrieves the download link for futures trade history using a previously obtained download ID.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Futures-Trade-Download-Link-by-Id
    ///
    /// Rate limit: 10 requests per minute
    ///
    /// # Arguments
    /// * `request` - Parameters for trade download link by ID request
    ///
    /// # Returns
    /// TradeDownloadLinkByIdResponse containing the download link and status
    pub async fn get_trade_download_link_by_id(
        &self,
        request: GetTradeDownloadLinkByIdRequest,
    ) -> RestResult<TradeDownloadLinkByIdResponse> {
        self.send_get_signed_request(TRADE_DOWNLOAD_LINK_BY_ID_ENDPOINT, request, 10, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_request_serialization() {
        let req = GetTradeDownloadLinkByIdRequest {
            download_id: Cow::Borrowed("123456"),
            recv_window: Some(5000),
            timestamp: 1650000000000,
        };
        let ser = serde_json::to_string(&req).unwrap();
        assert!(ser.contains("downloadId"));
        assert!(ser.contains("recvWindow"));
        assert!(ser.contains("timestamp"));
    }

    #[test]
    fn test_response_deserialization_completed() {
        let data = json!({
            "downloadId": "545923594199212032",
            "status": "completed",
            "url": "www.binance.com",
            "notified": true,
            "expirationTimestamp": 1645009771000u64,
            "isExpired": null
        });
        let resp: TradeDownloadLinkByIdResponse = serde_json::from_value(data).unwrap();
        assert_eq!(resp.download_id, "545923594199212032");
        assert_eq!(resp.status, TradeDownloadStatus::Completed);
        assert_eq!(resp.url, "www.binance.com");
        assert_eq!(resp.notified, true);
        assert_eq!(resp.expiration_timestamp, 1645009771000u64);
        assert_eq!(resp.is_expired, None);
    }

    #[test]
    fn test_response_deserialization_processing() {
        let data = json!({
            "downloadId": "545923594199212032",
            "status": "processing",
            "url": "",
            "notified": false,
            "expirationTimestamp": 0u64,
            "isExpired": null
        });
        let resp: TradeDownloadLinkByIdResponse = serde_json::from_value(data).unwrap();
        assert_eq!(resp.status, TradeDownloadStatus::Processing);
        assert_eq!(resp.url, "");
        assert_eq!(resp.notified, false);
    }
}
