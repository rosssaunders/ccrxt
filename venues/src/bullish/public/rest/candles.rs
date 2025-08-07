//! Candles endpoint for Bullish Exchange API

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult, enums::CandleInterval};

/// Endpoint URL path for candles
const CANDLES_ENDPOINT: &str = "/trading-api/v1/markets/{}/candles";

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

/// Request parameters for getting candles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCandlesRequest {
    /// The market symbol to get candles for
    pub symbol: String,

    /// Candlestick interval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<CandleInterval>,

    /// Start time for historical data
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,

    /// End time for historical data
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,

    /// Number of candles to return (default: 500, max: 1000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl RestClient {
    /// Get candlestick data for a market symbol
    ///
    /// Retrieves historical candlestick data for a specific market.
    ///
    /// [docs]: https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v1/markets/-symbol-/candles
    ///
    /// # Arguments
    /// * `request` - Request parameters containing the market symbol and optional filters
    ///
    /// # Returns
    /// A `RestResult<Vec<Candle>>` containing the candlestick data
    ///
    /// # Errors
    /// Returns an error if the request fails or the response cannot be parsed
    pub async fn get_candles(&self, request: &GetCandlesRequest) -> RestResult<Vec<Candle>> {
        let endpoint = CANDLES_ENDPOINT.replace("{}", &request.symbol);

        // Create query params from the request, excluding the symbol
        let query_params = serde_urlencoded::to_string(
            &[
                (
                    "interval",
                    request.interval.as_ref().map(|i| i.to_string()).as_deref(),
                ),
                ("startTime", request.start_time.as_deref()),
                ("endTime", request.end_time.as_deref()),
                (
                    "limit",
                    request.limit.as_ref().map(|l| l.to_string()).as_deref(),
                ),
            ]
            .into_iter()
            .filter_map(|(k, v)| v.map(|val| (k, val)))
            .collect::<Vec<_>>(),
        )
        .unwrap_or_default();

        let full_endpoint = if query_params.is_empty() {
            endpoint
        } else {
            format!("{}?{}", endpoint, query_params)
        };

        self.send_request::<Vec<Candle>, ()>(
            &full_endpoint,
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
