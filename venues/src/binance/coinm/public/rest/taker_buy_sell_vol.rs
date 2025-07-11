use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, enums::Period, public::rest::RestClient};

/// Parameters for Taker Buy/Sell Volume
#[derive(Debug, Clone, Serialize)]
pub struct TakerBuySellVolParams {
    /// Trading pair (e.g., "BTCUSD")
    pub pair: String,
    /// Contract type (e.g., "PERPETUAL", "CURRENT_QUARTER", "NEXT_QUARTER")
    #[serde(rename = "contractType")]
    pub contract_type: String,
    /// The time interval
    pub period: Period,
    /// Maximum 500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Start time
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// End time
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

/// Taker buy/sell volume
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TakerBuySellVol {
    /// Pair name
    pub pair: String,
    /// Contract type
    pub contract_type: String,
    /// Taker buy volume
    pub taker_buy_vol: Decimal,
    /// Taker sell volume
    pub taker_sell_vol: Decimal,
    /// Taker buy volume value
    pub taker_buy_vol_value: Decimal,
    /// Taker sell volume value
    pub taker_sell_vol_value: Decimal,
    /// Timestamp
    pub timestamp: i64,
}

impl RestClient {
    /// Get taker buy/sell volume
    ///
    /// Weight: 1
    pub async fn get_taker_buy_sell_vol(
        &self,
        params: TakerBuySellVolParams,
    ) -> RestResult<Vec<TakerBuySellVol>> {
        self.send_request(
            "/futures/data/takerBuySellVol",
            reqwest::Method::GET,
            Some(params),
            1,
        )
        .await
    }
}
