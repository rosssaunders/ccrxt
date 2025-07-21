//! Get Trade Download Link endpoint for Binance USDM REST API.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const TRADE_DOWNLOAD_LINK_ENDPOINT: &str = "/fapi/v1/trade/asyn/id";

/// Request for getting trade download link.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTradeDownloadLinkRequest {
    /// Download ID from the previous request.
    pub download_id: String,
    /// Timestamp in milliseconds.
    pub timestamp: u64,
    /// Signature for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// Download status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DownloadStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "failed")]
    Failed,
}

/// Response from trade download link endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeDownloadLinkResponse {
    /// Download link (available when status is completed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_link: Option<String>,
    /// Download status
    pub status: DownloadStatus,
}

impl UsdmClient {
    /// Get Futures Trade Download Link by ID
    ///
    /// Retrieves the download link for futures trade history using the previously obtained download ID.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Trade-History
    ///
    /// Rate limit: 5 requests per minute
    ///
    /// # Arguments
    /// * `request` - Parameters for trade download link request
    ///
    /// # Returns
    /// TradeDownloadLinkResponse containing the download link and status
    pub async fn get_trade_download_link(
        &self,
        request: GetTradeDownloadLinkRequest,
    ) -> RestResult<TradeDownloadLinkResponse> {
        self.send_signed_request(TRADE_DOWNLOAD_LINK_ENDPOINT, Method::GET, request, 5, true)
            .await
    }
}
