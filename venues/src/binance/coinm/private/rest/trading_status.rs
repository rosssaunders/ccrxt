use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{RestResult, private::rest::client::RestClient},
    shared,
};

/// Request parameters for API trading status.
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TradingStatusRequest {
    /// This parameter cannot be used in combination with other parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Indicator information for trading status.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingIndicator {
    /// Indicator name
    #[serde(rename = "i")]
    pub indicator: String,

    /// Count
    #[serde(rename = "c")]
    pub count: u32,

    /// Current value
    #[serde(rename = "v")]
    pub current_value: f64,

    /// Trigger value
    #[serde(rename = "t")]
    pub trigger_value: f64,
}

/// Response for API trading status.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingStatusResponse {
    /// Is locked
    pub is_locked: bool,

    /// Planned recover time
    pub planned_recover_time: u64,

    /// Trigger condition
    pub trigger_condition: Option<TradingIndicator>,

    /// Indicators
    pub indicators: Option<Vec<TradingIndicator>>,

    /// Update time
    pub update_time: u64,
}

impl RestClient {
    /// Get API trading status on Binance Coin-M Futures.
    ///
    /// See: <https://binance-docs.github.io/apidocs/delivery/en/>
    /// GET /dapi/v1/apiTradingStatus
    /// Weight: 1
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`TradingStatusRequest`])
    ///
    /// # Returns
    /// A [`TradingStatusResponse`] object with trading status details.
    pub async fn get_trading_status(
        &self,
        params: TradingStatusRequest,
    ) -> RestResult<TradingStatusResponse> {
        let weight = 1;
        shared::send_signed_request(
            self,
            "/dapi/v1/apiTradingStatus",
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}
