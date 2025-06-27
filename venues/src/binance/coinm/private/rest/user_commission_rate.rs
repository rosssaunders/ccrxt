// User Commission Rate (USER_DATA) endpoint implementation for GET /dapi/v1/commissionRate
// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/User-Commission-Rate>

use serde::{Deserialize, Serialize};

use crate::binance::coinm::RestResult;
use crate::binance::coinm::private::rest::client::RestClient;
use crate::binance::shared;

/// Request parameters for getting user commission rate (GET /dapi/v1/commissionRate).
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetUserCommissionRateRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Response for getting user commission rate (GET /dapi/v1/commissionRate).
#[derive(Debug, Clone, Deserialize)]
pub struct GetUserCommissionRateResponse {
    /// Trading symbol.
    pub symbol: String,

    /// Maker commission rate (e.g., "0.00015" for 0.015%).
    #[serde(rename = "makerCommissionRate")]
    pub maker_commission_rate: String,

    /// Taker commission rate (e.g., "0.00040" for 0.040%).
    #[serde(rename = "takerCommissionRate")]
    pub taker_commission_rate: String,
}

impl RestClient {
    /// Gets user commission rate (USER_DATA) on Binance Coin-M Futures.
    ///
    /// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/User-Commission-Rate>
    /// GET /dapi/v1/commissionRate
    /// Weight: 20
    /// Requires API key and signature.
    ///
    /// Query user commission rate.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`GetUserCommissionRateRequest`])
    ///
    /// # Returns
    /// A [`GetUserCommissionRateResponse`] with the commission rates.
    pub async fn get_user_commission_rate(
        &self,
        params: GetUserCommissionRateRequest,
    ) -> RestResult<GetUserCommissionRateResponse> {
        let weight = 20;
        shared::send_signed_request(
            self,
            "/dapi/v1/commissionRate",
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}
