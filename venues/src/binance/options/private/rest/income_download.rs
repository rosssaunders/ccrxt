use serde::{Deserialize, Serialize};

use crate::binance::options::RestResult;
use crate::binance::shared;

use super::client::RestClient;

/// Request parameters for initiating income download
#[derive(Debug, Clone, Serialize)]
pub struct IncomeDownloadRequest {
    /// Start time
    #[serde(rename = "startTime")]
    pub start_time: u64,

    /// End time
    #[serde(rename = "endTime")]
    pub end_time: u64,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Request parameters for checking income download status
#[derive(Debug, Clone, Serialize)]
pub struct IncomeDownloadStatusRequest {
    /// Download ID from the initiate response
    #[serde(rename = "downloadId")]
    pub download_id: String,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Response for income download initiation
#[derive(Debug, Clone, Deserialize)]
pub struct IncomeDownloadResponse {
    /// Average time estimate for file generation (in seconds)
    #[serde(rename = "avgCostTimestampOfEvery10m")]
    pub avg_cost_timestamp_of_every_10m: u64,

    /// Download ID for status checking
    #[serde(rename = "downloadId")]
    pub download_id: String,
}

/// Response for income download status check
#[derive(Debug, Clone, Deserialize)]
pub struct IncomeDownloadStatusResponse {
    /// Download ID
    #[serde(rename = "downloadId")]
    pub download_id: String,

    /// Download status: PROCESSING, COMPLETED, or FAILED
    #[serde(rename = "status")]
    pub status: String,

    /// Download URL (available when status is COMPLETED)
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Notification of successful download
    #[serde(rename = "notified")]
    pub notified: bool,

    /// Request time for the download
    #[serde(rename = "expirationTimestamp")]
    pub expiration_timestamp: u64,

    /// Whether the download link is expired
    #[serde(rename = "isExpired")]
    pub is_expired: bool,
}

impl RestClient {
    /// Initiate income download
    ///
    /// Starts the process of generating a downloadable income file for the specified time period.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/account/Account-Income-Download)
    /// Method: GET /eapi/v1/income/asyn
    /// Weight: 5
    /// Requires: API key and signature
    pub async fn initiate_income_download(
        &self,
        params: IncomeDownloadRequest,
    ) -> RestResult<IncomeDownloadResponse> {
        shared::send_signed_request(
            self,
            "/eapi/v1/income/asyn",
            reqwest::Method::GET,
            params,
            5,
            false,
        )
        .await
    }

    /// Check income download status
    ///
    /// Checks the status of a previously initiated income download.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/account/Get-Income-Download-Id-Info)
    /// Method: GET /eapi/v1/income/asyn/id
    /// Weight: 5
    /// Requires: API key and signature
    pub async fn get_income_download_status(
        &self,
        params: IncomeDownloadStatusRequest,
    ) -> RestResult<IncomeDownloadStatusResponse> {
        shared::send_signed_request(
            self,
            "/eapi/v1/income/asyn/id",
            reqwest::Method::GET,
            params,
            5,
            false,
        )
        .await
    }
}
