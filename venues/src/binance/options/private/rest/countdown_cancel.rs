use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::{options::RestResult, shared};

/// Request parameters for countdown cancel
#[derive(Debug, Clone, Serialize)]
pub struct CountdownCancelRequest {
    /// Underlying asset (e.g., "BTCUSDT")
    #[serde(rename = "underlying")]
    pub underlying: String,

    /// Countdown time in milliseconds (range: [10000, 120000])
    #[serde(rename = "countdownTime")]
    pub countdown_time: u64,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Request parameters for getting countdown cancel config
#[derive(Debug, Clone, Serialize)]
pub struct GetCountdownCancelRequest {
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

/// Request parameters for countdown cancel heartbeat
#[derive(Debug, Clone, Serialize)]
pub struct CountdownCancelHeartbeatRequest {
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

/// Response for countdown cancel operation
#[derive(Debug, Clone, Deserialize)]
pub struct CountdownCancelResponse {
    /// Underlying asset
    #[serde(rename = "underlying")]
    pub underlying: String,

    /// Countdown time in milliseconds
    #[serde(rename = "countdownTime")]
    pub countdown_time: u64,

    /// Trigger time (timestamp when countdown will trigger)
    #[serde(rename = "triggerTime")]
    pub trigger_time: u64,
}

impl RestClient {
    /// Get auto-cancel all open orders config
    ///
    /// Gets the current countdown cancel configuration for the specified underlying.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/market-maker-endpoints/Get-Auto-Cancel-All-Open-Orders-Config)
    /// Method: GET /eapi/v1/countdownCancelAll
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn get_countdown_cancel_config(
        &self,
        params: GetCountdownCancelRequest,
    ) -> RestResult<CountdownCancelResponse> {
        shared::send_signed_request(
            self,
            "/eapi/v1/countdownCancelAll",
            reqwest::Method::GET,
            params,
            1,
            false,
        )
        .await
    }

    /// Set countdown cancel timer
    ///
    /// Sets a countdown timer to cancel all open orders for the specified underlying
    /// after the countdown period expires. Used as a safety mechanism.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/market-maker-endpoints/Set-Auto-Cancel-All-Open-Orders-Config)
    /// Method: POST /eapi/v1/countdownCancelAll
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn set_countdown_cancel_config(
        &self,
        params: CountdownCancelRequest,
    ) -> RestResult<CountdownCancelResponse> {
        shared::send_signed_request(
            self,
            "/eapi/v1/countdownCancelAll",
            reqwest::Method::POST,
            params,
            1,
            false,
        )
        .await
    }

    /// Auto-cancel all open orders heartbeat
    ///
    /// Sends a heartbeat to reset the countdown timer. Must be called before the timer expires
    /// to prevent automatic cancellation of all orders.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/market-maker-endpoints/Auto-Cancel-All-Open-Orders-Heartbeat)
    /// Method: POST /eapi/v1/countdownCancelAllHeartBeat
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn countdown_cancel_heartbeat(
        &self,
        params: CountdownCancelHeartbeatRequest,
    ) -> RestResult<CountdownCancelResponse> {
        shared::send_signed_request(
            self,
            "/eapi/v1/countdownCancelAllHeartBeat",
            reqwest::Method::POST,
            params,
            1,
            false,
        )
        .await
    }
}
