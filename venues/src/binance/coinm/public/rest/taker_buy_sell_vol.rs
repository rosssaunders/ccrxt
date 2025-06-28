use crate::binance::coinm::RestResult;
use crate::binance::coinm::enums::Period;
use crate::binance::coinm::public::rest::RestClient;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Parameters for Taker Buy/Sell Volume
#[derive(Debug, Clone, Serialize)]
pub struct TakerBuySellVolParams {
    /// Symbol name
    pub symbol: String,
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
    /// Buy volume
    pub buy_vol: Decimal,
    /// Sell volume
    pub sell_vol: Decimal,
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
