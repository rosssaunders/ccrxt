//! Request and response structs for public/get-candlestick endpoint
//!
//! Retrieves candlesticks (k-line data history) over a given period for an instrument.

use super::client::RestClient;
use crate::cryptocom::EndpointType;
use crate::cryptocom::RestResult;
use crate::cryptocom::Timeframe;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Request parameters for the public/get-candlestick endpoint.
///
/// Retrieves candlestick (k-line) data for a given instrument and timeframe.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetCandlestickRequest {
    /// Instrument name (e.g., "BTCUSD-PERP"). Required.
    #[serde(rename = "instrument_name")]
    pub instrument_name: Cow<'static, str>,

    /// Timeframe for candlesticks (e.g., "1m", "5m", "1h"). Required.
    #[serde(rename = "timeframe")]
    pub timeframe: Timeframe,

    /// Number of candlesticks to return. Optional. Max: 1500.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,

    /// Start timestamp (milliseconds since epoch). Optional.
    #[serde(rename = "start_ts", skip_serializing_if = "Option::is_none")]
    pub start_ts: Option<u64>,

    /// End timestamp (milliseconds since epoch). Optional.
    #[serde(rename = "end_ts", skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<u64>,
}

/// Response for public/get-candlestick endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetCandlestickResponse {
    /// Result data for candlesticks.
    #[serde(rename = "result")]
    pub result: CandlestickResult,

    /// Success status.
    #[serde(rename = "success")]
    pub success: bool,

    /// Response ID.
    #[serde(rename = "id")]
    pub id: u64,
}

/// Result data for candlesticks.
#[derive(Debug, Clone, Deserialize)]
pub struct CandlestickResult {
    /// Instrument name.
    #[serde(rename = "instrument_name")]
    pub instrument_name: Cow<'static, str>,

    /// List of candlestick data arrays: [timestamp, open, high, low, close, volume].
    #[serde(rename = "data")]
    pub data: Vec<[f64; 6]>,
}

impl RestClient {
    /// Calls the public/get-candlestick endpoint.
    ///
    /// Retrieves candlestick (k-line) data for a given instrument and timeframe.
    ///
    /// [Official API docs](https://exchange-docs.crypto.com/spot/index.html#public-get-candlestick)
    pub async fn get_candlestick(
        &self,
        params: GetCandlestickRequest,
    ) -> RestResult<GetCandlestickResponse> {
        self.send_request(
            "public/get-candlestick",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetCandlestick,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_candlestick_endpoint_type() {
        let candlestick_endpoint = EndpointType::PublicGetCandlestick;
        assert!(candlestick_endpoint.rate_limit().max_requests > 0);
    }

    #[test]
    fn test_candlestick_parameter_building() {
        let params = json!({
            "instrument_name": "BTC_USDT",
            "timeframe": "1h",
            "count": 25,
            "start_ts": 1234567890,
            "end_ts": 1234567900
        });
        assert_eq!(params.get("instrument_name").unwrap(), "BTC_USDT");
        assert_eq!(params.get("timeframe").unwrap(), "1h");
        assert_eq!(params.get("count").unwrap(), 25);
        assert_eq!(params.get("start_ts").unwrap(), 1234567890);
        assert_eq!(params.get("end_ts").unwrap(), 1234567900);
    }
}
