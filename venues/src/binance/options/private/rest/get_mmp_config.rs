use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::RestResult;

const GET_MMP_CONFIG_ENDPOINT: &str = "/eapi/v1/mmp";

/// Request parameters for getting MMP configuration.
#[derive(Debug, Clone, Serialize)]
pub struct GetMmpConfigRequest {
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

/// MMP configuration response.
#[derive(Debug, Clone, Deserialize)]
pub struct MmpConfigResponse {
    /// Underlying ID.
    #[serde(rename = "underlyingId")]
    pub underlying_id: u64,

    /// Underlying asset.
    #[serde(rename = "underlying")]
    pub underlying: String,

    /// MMP interval in milliseconds.
    #[serde(rename = "windowTimeInMilliseconds")]
    pub window_time_in_milliseconds: u64,

    /// MMP frozen time in milliseconds.
    #[serde(rename = "frozenTimeInMilliseconds")]
    pub frozen_time_in_milliseconds: u64,

    /// Quantity limit.
    #[serde(rename = "qtyLimit")]
    pub qty_limit: Decimal,

    /// Net delta limit.
    #[serde(rename = "deltaLimit")]
    pub delta_limit: Decimal,

    /// Last trigger time.
    #[serde(rename = "lastTriggerTime")]
    pub last_trigger_time: u64,
}

impl RestClient {
    /// Get Market Maker Protection (MMP) configuration
    ///
    /// Returns the current MMP configuration for the specified underlying.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/option/market-maker-endpoints/Get-Market-Maker-Protection-Config
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `params` - The get MMP configuration request parameters
    ///
    /// # Returns
    /// Current MMP configuration for the specified underlying
    pub async fn get_mmp_config(
        &self,
        params: GetMmpConfigRequest,
    ) -> RestResult<MmpConfigResponse> {
        self.send_get_signed_request(
            GET_MMP_CONFIG_ENDPOINT,
            params,
            1,
            false,
        )
        .await
    }
}
