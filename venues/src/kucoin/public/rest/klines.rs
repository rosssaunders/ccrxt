use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::kucoin::{KlineInterval, ResponseHeaders, RestResponse, Result};

use super::RestClient;

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
    ///
    /// # Example
    /// ```rust,no_run
    /// use venues::kucoin::public::rest::{RestClient, GetKlinesRequest};
    /// use venues::kucoin::KlineInterval;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_default();
    ///     let request = GetKlinesRequest {
    ///         symbol: "BTC-USDT".to_string(),
    ///         interval: KlineInterval::OneHour,
    ///         start_time: None,
    ///         end_time: None,
    ///     };
    ///     let (response, _headers) = client.get_klines(request).await?;
    ///     println!("Found {} klines", response.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_klines(
        &self,
        request: GetKlinesRequest,
    ) -> Result<(Vec<Kline>, ResponseHeaders)> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), request.symbol);

        // Serialize the interval enum to get the correct string representation
        let interval_str = serde_json::to_string(&request.interval)?
            .trim_matches('"')
            .to_string();
        params.insert("type".to_string(), interval_str);

        if let Some(start_time) = request.start_time {
            params.insert("startAt".to_string(), start_time.to_string());
        }

        if let Some(end_time) = request.end_time {
            params.insert("endAt".to_string(), end_time.to_string());
        }

        let (response, headers): (RestResponse<Vec<KlineArray>>, ResponseHeaders) =
            self.get(KLINES_ENDPOINT, Some(params)).await?;

        let klines: Vec<Kline> = response.data.into_iter().map(Kline::from).collect();

        Ok((klines, headers))
    }
}
