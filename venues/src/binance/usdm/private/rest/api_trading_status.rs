use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;
use crate::binance::usdm::enums::TriggerCondition;

const API_TRADING_STATUS_ENDPOINT: &str = "/fapi/v1/apiTradingStatus";

/// Request parameters for the API trading status endpoint.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetApiTradingStatusRequest {
    /// Timestamp in milliseconds since epoch. Required.
    pub timestamp: u64,
}

/// Indicator representing trading function status.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingStatusIndicator {
    /// Whether the trading function is locked.
    pub is_locked: bool,

    /// Planned recovery time in milliseconds (0 if no recovery is planned).
    pub planned_recover_time: u64,

    /// Trigger condition that caused the lock.
    pub trigger_condition: TriggerCondition,

    /// Update time of this status in milliseconds since epoch.
    pub update_time: u64,
}

/// Response from the API trading status endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiTradingStatusResponse {
    /// Whether the account trading is locked.
    pub is_locked: bool,

    /// Planned recovery time in milliseconds (0 if no recovery is planned).
    pub planned_recover_time: u64,

    /// Trigger condition that caused the lock.
    pub trigger_condition: TriggerCondition,

    /// Update time of this status in milliseconds since epoch.
    pub update_time: u64,

    /// Trading status indicators for various trading functions.
    pub indicators: Vec<TradingStatusIndicator>,
}

impl UsdmClient {
    /// Get API Trading Status
    ///
    /// Retrieves the trading status for the account, including lock status, planned recovery time, and trigger conditions.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Futures-Trading-Quantitative-Rules-Indicators
    ///
    /// Rate limit: 5
    ///
    /// # Arguments
    /// * `params` - The request parameters for API trading status
    ///
    /// # Returns
    /// Returns the trading status for the account.
    pub async fn get_api_trading_status(
        &self,
        params: GetApiTradingStatusRequest,
    ) -> RestResult<ApiTradingStatusResponse> {
        self.send_signed_request(API_TRADING_STATUS_ENDPOINT, Method::GET, params, 5, false)
            .await
    }
}
