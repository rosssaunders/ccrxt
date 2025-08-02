use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::coinm::{
    RestResult,
    enums::{ContractTypeFilter, Period},
    public::rest::RestClient,
};

const TAKER_BUY_SELL_VOL_ENDPOINT: &str = "/futures/data/takerBuySellVol";

/// Request parameters for the Taker Buy/Sell Volume endpoint.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TakerBuySellVolRequest {
    /// Trading pair (e.g., "BTCUSD"). Required.
    /// Must be a valid Coin-M futures symbol.
    pub pair: String,

    /// Contract type. Required.
    /// One of: ALL, CURRENT_QUARTER, NEXT_QUARTER, PERPETUAL.
    /// See [`ContractTypeFilter`] enum for valid values.
    pub contract_type: ContractTypeFilter,

    /// Time interval. Required.
    /// One of: "5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d".
    /// See [`Period`] enum for valid values.
    pub period: Period,

    /// Number of data points to return. Optional. Default: 30, Max: 500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Start time in milliseconds since epoch. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,

    /// End time in milliseconds since epoch. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

/// Represents a single taker buy/sell volume data point.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TakerBuySellVol {
    /// Trading pair (e.g., "BTCUSD").
    pub pair: String,

    /// Contract type (e.g., "PERPETUAL", "CURRENT_QUARTER", "NEXT_QUARTER").
    pub contract_type: ContractTypeFilter,

    /// Taker buy volume (unit: contracts).
    pub taker_buy_vol: Decimal,

    /// Taker sell volume (unit: contracts).
    pub taker_sell_vol: Decimal,

    /// Taker buy volume value (unit: base asset).
    pub taker_buy_vol_value: Decimal,

    /// Taker sell volume value (unit: base asset).
    pub taker_sell_vol_value: Decimal,

    /// Timestamp in milliseconds since epoch.
    pub timestamp: i64,
}

impl RestClient {
    /// Taker Buy/Sell Volume
    ///
    /// Returns the total volume of buy and sell orders filled by takers within the specified period.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Taker-Buy-Sell-Volume
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `request` - The request parameters for taker buy/sell volume
    ///
    /// # Returns
    /// A vector of [`TakerBuySellVol`] data points for the requested period.
    pub async fn get_taker_buy_sell_vol(
        &self,
        request: TakerBuySellVolRequest,
    ) -> RestResult<Vec<TakerBuySellVol>> {
        self.send_request(
            TAKER_BUY_SELL_VOL_ENDPOINT,
            reqwest::Method::GET,
            Some(request),
            1,
        )
        .await
    }
}
