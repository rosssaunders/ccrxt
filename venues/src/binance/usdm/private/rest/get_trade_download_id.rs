//! Get Trade Download ID endpoint for Binance USDM REST API.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const TRADE_DOWNLOAD_ID_ENDPOINT: &str = "/fapi/v1/trade/asyn";

/// Request for getting trade download ID.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTradeDownloadIdRequest {
    /// Start time in milliseconds.
    pub start_time: u64,
    /// End time in milliseconds.
    pub end_time: u64,
    /// Timestamp in milliseconds.
    pub timestamp: u64,
    /// Signature for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// Response from trade download ID endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeDownloadIdResponse {
    /// Average time taken for data download in the past 30 days
    pub avg_cost_timestamp: String,
    /// Download ID
    pub download_id: String,
}

impl UsdmClient {
    /// Get Futures Trade Download ID
    ///
    /// Initiates an async request to generate a download ID for futures trade history.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Futures-Trade-Download-Link-by-Id
    ///
    /// Rate limit: 5 requests per minute
    ///
    /// # Arguments
    /// * `request` - Parameters for trade download ID request
    ///
    /// # Returns
    /// TradeDownloadIdResponse containing the download ID and average cost timestamp
    pub async fn get_trade_download_id(
        &self,
        request: GetTradeDownloadIdRequest,
    ) -> RestResult<TradeDownloadIdResponse> {
        self.send_signed_request(TRADE_DOWNLOAD_ID_ENDPOINT, Method::GET, request, 5, true)
            .await
    }
}
