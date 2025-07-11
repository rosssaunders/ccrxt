use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

/// Parameters for 24hr Ticker Price Change Statistics
#[derive(Debug, Clone, Serialize, Default)]
pub struct Ticker24hrParams {
    /// Symbol name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Contract type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
}

/// 24hr ticker price change statistics
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker24hr {
    /// Symbol name
    pub symbol: String,
    /// Pair name
    pub pair: String,
    /// Price change
    pub price_change: Decimal,
    /// Price change percent
    pub price_change_percent: Decimal,
    /// Weighted average price
    pub weighted_avg_price: Decimal,
    /// Last price
    pub last_price: Decimal,
    /// Last quantity
    pub last_qty: Decimal,
    /// Open price
    pub open_price: Decimal,
    /// High price
    pub high_price: Decimal,
    /// Low price
    pub low_price: Decimal,
    /// Total traded base asset volume
    pub volume: Decimal,
    /// Total traded quote asset volume (called baseVolume in API response)
    #[serde(rename = "baseVolume")]
    pub quote_volume: Decimal,
    /// Statistics open time
    pub open_time: i64,
    /// Statistics close time
    pub close_time: i64,
    /// First trade id
    pub first_id: i64,
    /// Last trade id
    pub last_id: i64,
    /// Trade count
    pub count: i64,
}

impl RestClient {
    /// Get 24hr ticker price change statistics
    ///
    /// https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/24hr-Ticker-Price-Change-Statistics
    ///
    /// Weight: 1 for a single symbol; 40 when the symbol parameter is omitted
    pub async fn get_ticker_24hr(&self, params: Ticker24hrParams) -> RestResult<Vec<Ticker24hr>> {
        let weight = if params.symbol.is_some() || params.pair.is_some() {
            1
        } else {
            40
        };

        // The API always returns an array, even for single symbols
        self.send_request(
            "/dapi/v1/ticker/24hr",
            reqwest::Method::GET,
            Some(params),
            weight,
        )
        .await
    }
}
