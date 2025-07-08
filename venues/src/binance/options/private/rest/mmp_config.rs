use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::{options::RestResult, shared};

/// Request parameters for getting MMP configuration
#[derive(Debug, Clone, Serialize)]
pub struct GetMmpConfigRequest {
    /// Underlying asset (e.g., "BTCUSDT")
    #[serde(rename = "underlying")]
    pub underlying: String,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Request parameters for setting MMP configuration
#[derive(Debug, Clone, Serialize)]
pub struct SetMmpConfigRequest {
    /// Underlying asset (e.g., "BTCUSDT")
    #[serde(rename = "underlying")]
    pub underlying: String,

    /// MMP interval in milliseconds (range: (0, 5000])
    #[serde(rename = "windowTimeInMilliseconds")]
    pub window_time_in_milliseconds: u64,

    /// MMP frozen time in milliseconds (set to 0 for manual reset)
    #[serde(rename = "frozenTimeInMilliseconds")]
    pub frozen_time_in_milliseconds: u64,

    /// Quantity limit
    #[serde(rename = "qtyLimit")]
    pub qty_limit: Decimal,

    /// Net delta limit
    #[serde(rename = "deltaLimit")]
    pub delta_limit: Decimal,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Request parameters for resetting MMP
#[derive(Debug, Clone, Serialize)]
pub struct ResetMmpRequest {
    /// Underlying asset (e.g., "BTCUSDT")
    #[serde(rename = "underlying")]
    pub underlying: String,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// MMP configuration response
#[derive(Debug, Clone, Deserialize)]
pub struct MmpConfigResponse {
    /// Underlying ID
    #[serde(rename = "underlyingId")]
    pub underlying_id: u64,

    /// Underlying asset
    #[serde(rename = "underlying")]
    pub underlying: String,

    /// MMP interval in milliseconds
    #[serde(rename = "windowTimeInMilliseconds")]
    pub window_time_in_milliseconds: u64,

    /// MMP frozen time in milliseconds
    #[serde(rename = "frozenTimeInMilliseconds")]
    pub frozen_time_in_milliseconds: u64,

    /// Quantity limit
    #[serde(rename = "qtyLimit")]
    pub qty_limit: Decimal,

    /// Net delta limit
    #[serde(rename = "deltaLimit")]
    pub delta_limit: Decimal,

    /// Last trigger time
    #[serde(rename = "lastTriggerTime")]
    pub last_trigger_time: u64,
}

impl RestClient {
    /// Get Market Maker Protection (MMP) configuration
    ///
    /// Returns the current MMP configuration for the specified underlying.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/market-maker-endpoints/Get-Market-Maker-Protection-Config)
    /// Method: GET /eapi/v1/mmp
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn get_mmp_config(
        &self,
        params: GetMmpConfigRequest,
    ) -> RestResult<MmpConfigResponse> {
        shared::send_signed_request(self, "/eapi/v1/mmp", reqwest::Method::GET, params, 1, false)
            .await
    }

    /// Set Market Maker Protection (MMP) configuration
    ///
    /// Sets the MMP configuration for the specified underlying. MMP is a protection
    /// mechanism for option market makers to prevent mass trading in short periods.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/market-maker-endpoints/Set-Market-Maker-Protection-Config)
    /// Method: POST /eapi/v1/mmpSet
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn set_mmp_config(
        &self,
        params: SetMmpConfigRequest,
    ) -> RestResult<MmpConfigResponse> {
        shared::send_signed_request(
            self,
            "/eapi/v1/mmpSet",
            reqwest::Method::POST,
            params,
            1,
            false,
        )
        .await
    }

    /// Reset Market Maker Protection (MMP)
    ///
    /// Resets MMP and allows MMP orders to start again.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/market-maker-endpoints/Reset-Market-Maker-Protection-Config)
    /// Method: POST /eapi/v1/mmpReset
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn reset_mmp(&self, params: ResetMmpRequest) -> RestResult<MmpConfigResponse> {
        shared::send_signed_request(
            self,
            "/eapi/v1/mmpReset",
            reqwest::Method::POST,
            params,
            1,
            false,
        )
        .await
    }
}
