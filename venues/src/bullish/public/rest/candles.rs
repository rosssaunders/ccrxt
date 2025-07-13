//! Candles endpoint for Bullish Exchange API

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult, enums::CandleInterval};

/// Endpoint URL path for candles
const ENDPOINT_PATH: &str = "/trading-api/v1/markets/{}/candles";

/// Candlestick data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candle {
    /// Opening price
    pub open: String,
    /// Highest price
    pub high: String,
    /// Lowest price
    pub low: String,
    /// Closing price
    pub close: String,
    /// Volume
    pub volume: String,
    /// Quote volume
    pub quote_volume: String,
    /// Candle open time in ISO 8601 format
    pub open_time_datetime: String,
    /// Candle open time as timestamp
    pub open_time_timestamp: String,
    /// Candle close time in ISO 8601 format
    pub close_time_datetime: String,
    /// Candle close time as timestamp
    pub close_time_timestamp: String,
}

/// Parameters for candle requests
#[derive(Debug, Clone, Default)]
pub struct CandleParams {
    /// Candlestick interval
    pub interval: Option<CandleInterval>,
    /// Start time for historical data
    pub start_time: Option<String>,
    /// End time for historical data
    pub end_time: Option<String>,
    /// Number of candles to return (default: 500, max: 1000)
    pub limit: Option<u32>,
}

impl RestClient {
    /// Get candlestick data for a market symbol
    ///
    /// # Arguments
    /// * `symbol` - The market symbol to get candles for
    /// * `params` - Optional parameters for filtering candles
    ///
    /// # Returns
    /// A `RestResult<Vec<Candle>>` containing the candlestick data
    ///
    /// # Errors
    /// Returns an error if the request fails or the response cannot be parsed
    ///
    /// https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v1/markets/-symbol-/candle
    pub async fn get_candles(
        &self,
        symbol: &str,
        params: Option<CandleParams>,
    ) -> RestResult<Vec<Candle>> {
        let mut query_params = Vec::new();

        if let Some(params) = params {
            if let Some(interval) = params.interval {
                query_params.push(("interval".to_string(), interval.to_string()));
            }

            if let Some(start_time) = params.start_time {
                query_params.push(("startTime".to_string(), start_time));
            }

            if let Some(end_time) = params.end_time {
                query_params.push(("endTime".to_string(), end_time));
            }

            if let Some(limit) = params.limit {
                query_params.push(("limit".to_string(), limit.to_string()));
            }
        }

        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!(
                "?{}",
                query_params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("&")
            )
        };

        let endpoint = format!("{}{}", ENDPOINT_PATH.replace("{}", symbol), query_string);

        self.send_request::<Vec<Candle>, ()>(
            &endpoint,
            reqwest::Method::GET,
            None,
            EndpointType::PublicCandles,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candle_deserialization() {
        let json = r#"
        {
            "open": "50000.00",
            "high": "51000.00",
            "low": "49000.00",
            "close": "50500.00",
            "volume": "100.50000000",
            "quoteVolume": "5025000.00000000",
            "openTimeDatetime": "2024-01-01T00:00:00.000Z",
            "openTimeTimestamp": "1704067200000",
            "closeTimeDatetime": "2024-01-01T01:00:00.000Z",
            "closeTimeTimestamp": "1704070800000"
        }
        "#;

        let candle: Candle = serde_json::from_str(json).unwrap();
        assert_eq!(candle.open, "50000.00");
        assert_eq!(candle.high, "51000.00");
        assert_eq!(candle.low, "49000.00");
        assert_eq!(candle.close, "50500.00");
    }

    #[test]
    fn test_candle_interval_serialization() {
        assert_eq!(
            serde_json::to_string(&CandleInterval::OneMinute).unwrap(),
            "\"1m\""
        );
        assert_eq!(
            serde_json::to_string(&CandleInterval::OneHour).unwrap(),
            "\"1h\""
        );
        assert_eq!(
            serde_json::to_string(&CandleInterval::OneDay).unwrap(),
            "\"1d\""
        );
    }
}
