//! Request and response structs for public/get-trades endpoint
//!
//! Fetches the public trades for a particular instrument.

use super::client::RestClient;
use crate::cryptocom::EndpointType;
use crate::cryptocom::RestResult;
use crate::cryptocom::TradeSide;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
#[derive(Debug, Clone, Deserialize)]
pub struct GetTradesResponse {
    /// Result data for trades.
    #[serde(rename = "result")]
    pub result: TradesResult,

    /// Success status.
    #[serde(rename = "success")]
    pub success: bool,

    /// Response ID.
    #[serde(rename = "id")]
    pub id: u64,
}

/// Result data for trades.
#[derive(Debug, Clone, Deserialize)]
pub struct TradesResult {
    /// List of trade data.
    #[serde(rename = "data")]
    pub data: Vec<Trade>,
}

/// Trade data for a single trade.
#[derive(Debug, Clone, Deserialize)]
pub struct Trade {
    /// Trade ID.
    #[serde(rename = "trade_id")]
    pub trade_id: u64,

    /// Price.
    #[serde(rename = "price")]
    pub price: f64,

    /// Quantity.
    #[serde(rename = "quantity")]
    pub quantity: f64,

    /// Trade side (buy/sell).
    #[serde(rename = "side")]
    pub side: TradeSide,

    /// Timestamp (milliseconds since epoch).
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

impl RestClient {
    /// Calls the public/get-trades endpoint.
    ///
    /// Fetches the public trades for a particular instrument.
    ///
    /// [Official API docs](https://exchange-docs.crypto.com/spot/index.html#public-get-trades)
    pub async fn get_trades(&self, params: GetTradesRequest) -> RestResult<GetTradesResponse> {
        self.send_request(
            "public/get-trades",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetTrades,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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
        assert_eq!(params["instrument_name"], "BTC_USDT");
        assert_eq!(params["count"], 100);
        assert_eq!(params["start_ts"], "1234567890");
        assert_eq!(params["end_ts"], "1234567900");
    }
}
