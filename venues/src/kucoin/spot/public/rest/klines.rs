use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{KlineInterval, ResponseHeaders, RestResponse, Result};

const KLINES_ENDPOINT: &str = "/api/v1/market/candles";

/// Request for getting klines/candlestick data
#[derive(Debug, Clone, Serialize)]
pub struct GetKlinesRequest {
    /// Trading symbol (e.g., "BTC-USDT")
    pub symbol: String,

    /// Kline interval
    #[serde(rename = "type")]
    pub interval: KlineInterval,

    /// Start time (Unix timestamp in seconds, optional)
    #[serde(rename = "startAt")]
    pub start_time: Option<i64>,

    /// End time (Unix timestamp in seconds, optional)
    #[serde(rename = "endAt")]
    pub end_time: Option<i64>,
}

/// Kline/Candlestick data
#[derive(Debug, Clone, Deserialize)]
pub struct Kline {
    /// Open time (Unix timestamp)
    pub open_time: i64,

    /// Open price
    pub open: String,

    /// Close price
    pub close: String,

    /// High price
    pub high: String,

    /// Low price
    pub low: String,

    /// Volume
    pub volume: String,

    /// Quote volume
    pub quote_volume: String,
}

// KuCoin returns klines as arrays: [timestamp, open, close, high, low, volume, quote_volume]
type KlineArray = [String; 7];

impl From<KlineArray> for Kline {
    fn from(arr: KlineArray) -> Self {
        Self {
            open_time: arr[0].parse().unwrap_or(0),
            open: arr[1].clone(),
            close: arr[2].clone(),
            high: arr[3].clone(),
            low: arr[4].clone(),
            volume: arr[5].clone(),
            quote_volume: arr[6].clone(),
        }
    }
}

impl RestClient {
    /// Get klines/candlestick data for a symbol
    pub async fn get_klines(
        &self,
        request: GetKlinesRequest,
    ) -> Result<(Vec<Kline>, ResponseHeaders)> {
        let (response, headers): (RestResponse<Vec<KlineArray>>, ResponseHeaders) =
            self.get_with_request(KLINES_ENDPOINT, &request).await?;

        let klines: Vec<Kline> = response.data.into_iter().map(Kline::from).collect();

        Ok((klines, headers))
    }
}
