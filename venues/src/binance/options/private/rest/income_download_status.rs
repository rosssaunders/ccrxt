use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::RestResult;

const INCOME_DOWNLOAD_STATUS_ENDPOINT: &str = "/eapi/v1/income/asyn/id";

/// Request parameters for checking income download status.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomeDownloadStatusRequest {
    /// Download ID from the initiate response.
    pub download_id: String,

    /// Request timeout window in milliseconds (max 60000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp (milliseconds since epoch).
    pub timestamp: u64,
}

/// Response for income download status check.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomeDownloadStatusResponse {
    /// Download ID.
    pub download_id: String,

    /// Download status: PROCESSING, COMPLETED, or FAILED.
    pub status: String,

    /// Download URL (available when status is COMPLETED).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Notification of successful download.
    pub notified: bool,

    /// Request time for the download (milliseconds since epoch).
    pub expiration_timestamp: u64,

    /// Whether the download link is expired.
    pub is_expired: bool,
}

impl RestClient {
    /// Check income download status
    ///
    /// Checks the status of a previously initiated income download.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/option/account/Get-Income-Download-Id-Info
    ///
    /// Rate limit: Weight 5
    ///
    /// # Arguments
    /// * `request` - The income download status request parameters
    ///
    /// # Returns
    /// Response containing download status and URL if completed
    pub async fn get_income_download_status(
        &self,
        request: IncomeDownloadStatusRequest,
    ) -> RestResult<IncomeDownloadStatusResponse> {
        self.send_get_signed_request(
            INCOME_DOWNLOAD_STATUS_ENDPOINT,
            request,
            5,
            false,
        )
        .await
    }
}
