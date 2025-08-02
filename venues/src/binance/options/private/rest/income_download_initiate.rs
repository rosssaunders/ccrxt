use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::RestResult;

const INCOME_DOWNLOAD_INITIATE_ENDPOINT: &str = "/eapi/v1/income/asyn";

/// Request parameters for initiating income download.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomeDownloadInitiateRequest {
    /// Start time for the income period (milliseconds since epoch).
    pub start_time: u64,

    /// End time for the income period (milliseconds since epoch).
    pub end_time: u64,

    /// Request timeout window in milliseconds (max 60000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp (milliseconds since epoch).
    pub timestamp: u64,
}

/// Response for income download initiation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomeDownloadInitiateResponse {
    /// Average time estimate for file generation (in seconds).
    pub avg_cost_timestamp_of_every_10m: u64,

    /// Download ID for status checking.
    pub download_id: String,
}

impl RestClient {
    /// Initiate income download
    ///
    /// Starts the process of generating a downloadable income file for the specified time period.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/option/account/Account-Income-Download
    ///
    /// Rate limit: Weight 5
    ///
    /// # Arguments
    /// * `request` - The income download initiation request parameters
    ///
    /// # Returns
    /// Response containing download ID and time estimate
    pub async fn initiate_income_download(
        &self,
        request: IncomeDownloadInitiateRequest,
    ) -> RestResult<IncomeDownloadInitiateResponse> {
        self.send_get_signed_request(
            INCOME_DOWNLOAD_INITIATE_ENDPOINT,
            request,
            5,
            false,
        )
        .await
    }
}
