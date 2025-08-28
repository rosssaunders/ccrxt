use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::cryptocom::{
    ApiResult, EndpointType, PublicRestClient as RestClient, RestResult, Timeframe,
};

/// Endpoint for getting candlestick data
const GET_CANDLESTICK_ENDPOINT: &str = "exchange/v1/public/get-candlestick";

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
pub type GetCandlestickResponse = ApiResult<CandlestickResult>;

/// Result data for candlesticks.
#[derive(Debug, Clone, Deserialize)]
pub struct CandlestickResult {
    /// Interval for candlesticks (e.g., "M5").
    #[serde(rename = "interval")]
    pub interval: Cow<'static, str>,

    /// List of candlestick data objects.
    #[serde(rename = "data")]
    pub data: Vec<CandlestickData>,

    /// Instrument name.
    #[serde(rename = "instrument_name")]
    pub instrument_name: Cow<'static, str>,
}

/// A single candlestick data point.
#[derive(Debug, Clone, Deserialize)]
pub struct CandlestickData {
    /// Open price.
    #[serde(rename = "o")]
    pub o: String,

    /// High price.
    #[serde(rename = "h")]
    pub h: String,

    /// Low price.
    #[serde(rename = "l")]
    pub l: String,

    /// Close price.
    #[serde(rename = "c")]
    pub c: String,

    /// Volume.
    #[serde(rename = "v")]
    pub v: String,

    /// Start time (timestamp in ms).
    #[serde(rename = "t")]
    pub t: u64,
}

impl RestClient {
    /// Calls the public/get-candlestick endpoint.
    ///
    /// Retrieves candlestick (k-line) data for a given instrument and timeframe.
    ///
    /// [docs](https://exchange-docs.crypto.com/exchange/v1/rest-ws/index.html#public-get-candlestick)
    pub async fn get_candlestick(
        &self,
        params: GetCandlestickRequest,
    ) -> RestResult<GetCandlestickResponse> {
        self.send_get_request(
            GET_CANDLESTICK_ENDPOINT,
            Some(&params),
            EndpointType::PublicGetCandlestick,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

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
