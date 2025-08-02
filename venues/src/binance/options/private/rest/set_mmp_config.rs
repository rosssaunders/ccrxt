use rust_decimal::Decimal;
use serde::Serialize;

use super::client::RestClient;
// Re-export the response type from get_mmp_config
pub use super::get_mmp_config::MmpConfigResponse;
use crate::binance::options::RestResult;

const SET_MMP_CONFIG_ENDPOINT: &str = "/eapi/v1/mmpSet";

/// Request parameters for setting MMP configuration.
#[derive(Debug, Clone, Serialize)]
pub struct SetMmpConfigRequest {
    /// Underlying asset (e.g., "BTCUSDT").
    #[serde(rename = "underlying")]
    pub underlying: String,

    /// MMP interval in milliseconds (range: (0, 5000]).
    #[serde(rename = "windowTimeInMilliseconds")]
    pub window_time_in_milliseconds: u64,

    /// MMP frozen time in milliseconds (set to 0 for manual reset).
    #[serde(rename = "frozenTimeInMilliseconds")]
    pub frozen_time_in_milliseconds: u64,

    /// Quantity limit.
    #[serde(rename = "qtyLimit")]
    pub qty_limit: Decimal,

    /// Net delta limit.
    #[serde(rename = "deltaLimit")]
    pub delta_limit: Decimal,

    /// Request timeout window in milliseconds (max 60000).
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds.
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

impl RestClient {
    /// Set Market Maker Protection (MMP) configuration
    ///
    /// Sets the MMP configuration for the specified underlying. MMP is a protection
    /// mechanism for option market makers to prevent mass trading in short periods.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/option/market-maker-endpoints/Set-Market-Maker-Protection-Config
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `params` - The set MMP configuration request parameters
    ///
    /// # Returns
    /// Updated MMP configuration for the specified underlying
    pub async fn set_mmp_config(
        &self,
        params: SetMmpConfigRequest,
    ) -> RestResult<MmpConfigResponse> {
        self.send_post_signed_request(SET_MMP_CONFIG_ENDPOINT, params, 1, false)
            .await
    }
}
