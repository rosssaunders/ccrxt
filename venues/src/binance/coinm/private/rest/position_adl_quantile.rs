// Position ADL Quantile Estimation (USER_DATA) endpoint implementation for GET /dapi/v1/adlQuantile
// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Position-ADL-Quantile-Estimation>

use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{RestResult, private::rest::client::RestClient},
    shared,
};

/// Request parameters for getting position ADL quantile estimation (GET /dapi/v1/adlQuantile).
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetPositionAdlQuantileRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP"). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// ADL quantile values for different position sides.
#[derive(Debug, Clone, Deserialize)]
pub struct AdlQuantileValues {
    /// ADL quantile for LONG position in hedge mode or position in one-way mode.
    #[serde(rename = "LONG", skip_serializing_if = "Option::is_none")]
    pub long: Option<u32>,

    /// ADL quantile for SHORT position in hedge mode.
    #[serde(rename = "SHORT", skip_serializing_if = "Option::is_none")]
    pub short: Option<u32>,

    /// ADL quantile for position in one-way mode.
    #[serde(rename = "BOTH", skip_serializing_if = "Option::is_none")]
    pub both: Option<u32>,

    /// Sign for hedge mode (only a sign, ignore the value).
    #[serde(rename = "HEDGE", skip_serializing_if = "Option::is_none")]
    pub hedge: Option<u32>,
}

/// Position ADL quantile entry for a symbol.
#[derive(Debug, Clone, Deserialize)]
pub struct PositionAdlQuantileEntry {
    /// Trading symbol.
    pub symbol: String,

    /// ADL quantile values for different position sides.
    #[serde(rename = "adlQuantile")]
    pub adl_quantile: AdlQuantileValues,
}

/// Response for getting position ADL quantile estimation (GET /dapi/v1/adlQuantile).
pub type GetPositionAdlQuantileResponse = Vec<PositionAdlQuantileEntry>;

impl RestClient {
    /// Gets position ADL quantile estimation (USER_DATA) on Binance Coin-M Futures.
    ///
    /// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Position-ADL-Quantile-Estimation>
    /// GET /dapi/v1/adlQuantile
    /// Weight: 5
    /// Requires API key and signature.
    ///
    /// Query position ADL quantile estimation.
    ///
    /// - Values update every 30s.
    /// - Values 0, 1, 2, 3, 4 shows the queue position and possibility of ADL from low to high.
    /// - For positions of the symbol are in One-way Mode or isolated margined in Hedge Mode,
    ///   "LONG", "SHORT", and "BOTH" will be returned to show the positions' adl quantiles of different position sides.
    /// - If the positions of the symbol are crossed margined in Hedge Mode:
    ///   - "HEDGE" as a sign will be returned instead of "BOTH";
    ///   - A same value calculated on unrealized pnls on long and short sides' positions will be shown
    ///     for "LONG" and "SHORT" when there are positions in both of long and short sides.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`GetPositionAdlQuantileRequest`])
    ///
    /// # Returns
    /// A [`GetPositionAdlQuantileResponse`] - array of position ADL quantile entries.
    pub async fn get_position_adl_quantile(
        &self,
        params: GetPositionAdlQuantileRequest,
    ) -> RestResult<GetPositionAdlQuantileResponse> {
        let weight = 5;
        shared::send_signed_request(
            self,
            "/dapi/v1/adlQuantile",
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}
