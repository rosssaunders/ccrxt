//! Implements the /public/get_tradingview_chart_data endpoint for Deribit.
//!
//! Retrieves TradingView-compatible OHLCV chart data for a given instrument and time range.
//!
//! [Official API docs](https://docs.deribit.com/#public-get_tradingview_chart_data)

use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, Errors as DeribitError};

const TRADINGVIEW_CHART_DATA_ENDPOINT: &str = "public/get_tradingview_chart_data";

use std::borrow::Cow;

/// Request parameters for the get_tradingview_chart_data endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetTradingviewChartDataRequest {
    /// Instrument name (e.g., "BTC-PERPETUAL").
    #[serde(rename = "instrument_name")]
    pub instrument_name: Cow<'static, str>,

    /// Start timestamp (ms since epoch, inclusive).
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: u64,

    /// End timestamp (ms since epoch, inclusive).
    #[serde(rename = "end_timestamp")]
    pub end_timestamp: u64,

    /// Desired resolution in seconds (e.g., 60 for 1m candles, 3600 for 1h candles).
    #[serde(rename = "resolution")]
    pub resolution: u32,
}

/// The result object for get_tradingview_chart_data.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct GetTradingviewChartDataResult {
    /// List of open prices for each candle.
    #[serde(rename = "o")]
    pub open: Vec<f64>,

    /// List of high prices for each candle.
    #[serde(rename = "h")]
    pub high: Vec<f64>,

    /// List of low prices for each candle.
    #[serde(rename = "l")]
    pub low: Vec<f64>,

    /// List of close prices for each candle.
    #[serde(rename = "c")]
    pub close: Vec<f64>,

    /// List of volume values for each candle.
    #[serde(rename = "v")]
    pub volume: Vec<f64>,

    /// List of timestamps (ms since epoch) for each candle.
    #[serde(rename = "t")]
    pub timestamps: Vec<u64>,
}

/// Response for the get_tradingview_chart_data endpoint.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct GetTradingviewChartDataResponse {
    /// The id that was sent in the request.
    #[serde(rename = "id")]
    pub id: u64,

    /// The JSON-RPC version (2.0).
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: String,

    /// The result object containing the chart data.
    #[serde(rename = "result")]
    pub result: GetTradingviewChartDataResult,
}

impl RestClient {
    /// Calls the /public/get_tradingview_chart_data endpoint.
    ///
    /// Retrieves TradingView-compatible OHLCV chart data for a given instrument and time range.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_tradingview_chart_data)
    pub async fn get_tradingview_chart_data(
        &self,
        params: &GetTradingviewChartDataRequest,
    ) -> Result<GetTradingviewChartDataResponse, DeribitError> {
        self.send_request(
            TRADINGVIEW_CHART_DATA_ENDPOINT,
            Method::GET,
            Some(params),
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_serialize_request() {
        let req = GetTradingviewChartDataRequest {
            instrument_name: Cow::Borrowed("BTC-PERPETUAL"),
            start_timestamp: 1680310800000,
            end_timestamp: 1680314400000,
            resolution: 60,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC-PERPETUAL"));
        assert!(json.contains("1680310800000"));
        assert!(json.contains("60"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
    "id": 42,
    "jsonrpc": "2.0",
    "result": {
        "o": [65000.0, 65100.0],
        "h": [65200.0, 65250.0],
        "l": [64900.0, 65050.0],
        "c": [65100.0, 65200.0],
        "v": [10.5, 12.3],
        "t": [1680310800000, 1680310860000]
    }
}"#;
        let resp: GetTradingviewChartDataResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 42);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.open, vec![65000.0, 65100.0]);
        assert_eq!(resp.result.high, vec![65200.0, 65250.0]);
        assert_eq!(resp.result.low, vec![64900.0, 65050.0]);
        assert_eq!(resp.result.close, vec![65100.0, 65200.0]);
        assert_eq!(resp.result.volume, vec![10.5, 12.3]);
        assert_eq!(resp.result.timestamps, vec![1680310800000, 1680310860000]);
    }
}
