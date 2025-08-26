use serde::Serialize;

use crate::binance::options::PrivateRestClient as RestClient;
// Re-export the response type from get_mmp_config
pub use super::get_mmp_config::MmpConfigResponse;
use crate::binance::options::RestResult;

const RESET_MMP_ENDPOINT: &str = "/eapi/v1/mmpReset";

/// Request parameters for resetting MMP.
#[derive(Debug, Clone, Serialize)]
pub struct ResetMmpRequest {
    /// Underlying asset (e.g., "BTCUSDT").
    #[serde(rename = "underlying")]
    pub underlying: String,

    /// Request timeout window in milliseconds (max 60000).
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds.
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

impl RestClient {
    /// Reset Market Maker Protection (MMP)
    ///
    /// Resets MMP and allows MMP orders to start again.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/option/market-maker-endpoints/Reset-Market-Maker-Protection-Config)
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `params` - The reset MMP request parameters
    ///
    /// # Returns
    /// MMP configuration after reset
    pub async fn reset_mmp(&self, params: ResetMmpRequest) -> RestResult<MmpConfigResponse> {
        self.send_post_signed_request(RESET_MMP_ENDPOINT, params, 1, false)
            .await
    }
}
