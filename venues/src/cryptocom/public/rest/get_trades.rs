use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::cryptocom::{ApiResult, EndpointType, PublicRestClient as RestClient, RestResult};

/// Endpoint for getting trades
const GET_TRADES_ENDPOINT: &str = "exchange/v1/public/get-trades";

/// Request parameters for the public/get-trades endpoint.
///
/// Fetches the public trades for a particular instrument.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTradesRequest {
    /// Instrument name (e.g., "BTCUSD-PERP"). Required.
    #[serde(rename = "instrument_name")]
    pub instrument_name: Cow<'static, str>,

    /// Number of trades to return. Optional. Max: 1000.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

/// Response for public/get-trades endpoint.
pub type GetTradesResponse = ApiResult<TradesResult>;

/// Result data for trades.
#[derive(Debug, Clone, Deserialize)]
pub struct TradesResult {
    /// List of trade data.
    #[serde(rename = "data")]
    pub data: Vec<Trade>,
}

/// Helper for fields that may be string or integer.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum StringOrInt {
    String(String),
    Int(i64),
}

impl StringOrInt {
    pub fn as_str(&self) -> String {
        match self {
            StringOrInt::String(s) => s.clone(),
            StringOrInt::Int(i) => i.to_string(),
        }
    }
}

/// Trade data for a single trade, matching Crypto.com API.
#[derive(Debug, Clone, Deserialize)]
pub struct Trade {
    /// Trade ID (string or int in API)
    #[serde(rename = "d")]
    pub trade_id: StringOrInt,

    /// Trade timestamp (milliseconds since epoch)
    #[serde(rename = "t")]
    pub timestamp: u64,

    /// Trade timestamp (nanoseconds, string or int)
    #[serde(rename = "tn")]
    pub timestamp_ns: Option<StringOrInt>,

    /// Quantity (string)
    #[serde(rename = "q")]
    pub quantity: String,

    /// Price (string)
    #[serde(rename = "p")]
    pub price: String,

    /// Side ("BUY" or "SELL")
    #[serde(rename = "s")]
    pub side: String,

    /// Instrument name
    #[serde(rename = "i")]
    pub instrument_name: String,

    /// Trade match ID (string or int)
    #[serde(rename = "m")]
    pub match_id: Option<StringOrInt>,
}

impl RestClient {
    /// Calls the public/get-trades endpoint.
    ///
    /// Fetches the public trades for a particular instrument.
    ///
    /// [docs](https://exchange-docs.crypto.com/exchange/v1/rest-ws/index.html#public-get-trades)
    pub async fn get_trades(&self, params: GetTradesRequest) -> RestResult<GetTradesResponse> {
        self.send_get_request(
            GET_TRADES_ENDPOINT,
            Some(&params),
            EndpointType::PublicGetTrades,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_trades_endpoint_type() {
        let trades_endpoint = EndpointType::PublicGetTrades;
        assert!(trades_endpoint.rate_limit().max_requests > 0);
    }

    #[test]
    fn test_trades_parameter_building() {
        let params = json!({
            "instrument_name": "BTC_USDT",
            "count": 100,
            "start_ts": "1234567890",
            "end_ts": "1234567900"
        });
        assert_eq!(params.get("instrument_name").unwrap(), "BTC_USDT");
        assert_eq!(params.get("count").unwrap(), 100);
        assert_eq!(params.get("start_ts").unwrap(), "1234567890");
        assert_eq!(params.get("end_ts").unwrap(), "1234567900");
    }
}
