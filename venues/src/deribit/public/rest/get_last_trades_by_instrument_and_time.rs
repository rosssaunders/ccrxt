//! Implements the /public/get_last_trades_by_instrument_and_time endpoint for Deribit.
//!
//! Retrieves the most recent trades for a given instrument, filtered by start and end timestamps.

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult, enums::Sorting};

const LAST_TRADES_BY_INSTRUMENT_AND_TIME_ENDPOINT: &str =
    "public/get_last_trades_by_instrument_and_time";

/// Request parameters for the get_last_trades_by_instrument_and_time endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetLastTradesByInstrumentAndTimeRequest {
    /// Instrument name for which to retrieve trades.
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,

    /// Start timestamp in milliseconds since epoch (inclusive).
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: u64,

    /// End timestamp in milliseconds since epoch (inclusive).
    #[serde(rename = "end_timestamp")]
    pub end_timestamp: u64,

    /// Number of results to return (default: 10, max: 1000).
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,

    /// Sorting direction (asc, desc, default).
    #[serde(rename = "sorting", skip_serializing_if = "Option::is_none")]
    pub sorting: Option<Sorting>,
}

/// Represents a single trade entry.
#[derive(Debug, Clone, Deserialize)]
pub struct TradeEntry {
    /// Trade ID.
    #[serde(rename = "trade_id")]
    pub trade_id: String,

    /// Price at which the trade occurred.
    #[serde(rename = "price")]
    pub price: f64,

    /// Amount traded.
    #[serde(rename = "amount")]
    pub amount: f64,

    /// Timestamp in milliseconds since epoch.
    #[serde(rename = "timestamp")]
    pub timestamp: u64,

    /// Direction of the trade (buy/sell).
    #[serde(rename = "direction")]
    pub direction: String,

    /// Index price at the time of the trade.
    #[serde(rename = "index_price")]
    pub index_price: f64,

    /// Instrument name.
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,

    /// Trade sequence number.
    #[serde(rename = "trade_seq")]
    pub trade_seq: u64,

    /// Mark price at the time of the trade.
    #[serde(rename = "mark_price")]
    pub mark_price: f64,

    /// Tick direction (enum).
    #[serde(rename = "tick_direction")]
    pub tick_direction: u8,

    /// Number of contracts.
    #[serde(rename = "contracts")]
    pub contracts: f64,
}

/// The result object for get_last_trades_by_instrument_and_time.
#[derive(Debug, Clone, Deserialize)]
pub struct GetLastTradesByInstrumentAndTimeResult {
    /// List of trade entries.
    #[serde(rename = "trades")]
    pub trades: Vec<TradeEntry>,

    /// Whether there are more trades available.
    #[serde(rename = "has_more")]
    pub has_more: bool,
}

/// Response for the get_last_trades_by_instrument_and_time endpoint.
pub type GetLastTradesByInstrumentAndTimeResponse =
    JsonRpcResult<GetLastTradesByInstrumentAndTimeResult>;

impl RestClient {
    /// Calls the /public/get_last_trades_by_instrument_and_time endpoint.
    ///
    /// Retrieves the most recent trades for a given instrument, filtered by start and end timestamps.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_last_trades_by_instrument_and_time)
    pub async fn get_last_trades_by_instrument_and_time(
        &self,
        params: GetLastTradesByInstrumentAndTimeRequest,
    ) -> RestResult<GetLastTradesByInstrumentAndTimeResponse> {
        self.send_request(
            LAST_TRADES_BY_INSTRUMENT_AND_TIME_ENDPOINT,
            Some(&params),
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
        let req = GetLastTradesByInstrumentAndTimeRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
            start_timestamp: 1680310000000,
            end_timestamp: 1680310800000,
            count: Some(2),
            sorting: Some(Sorting::Asc),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC-PERPETUAL"));
        assert!(json.contains("1680310000000"));
        assert!(json.contains("1680310800000"));
        assert!(json.contains("asc"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 21,
            "jsonrpc": "2.0",
            "result": {
                "trades": [
                    {
                        "timestamp": 1752128880144,
                        "price": 111356.0,
                        "amount": 200.0,
                        "direction": "buy",
                        "index_price": 111341.24,
                        "instrument_name": "BTC-PERPETUAL",
                        "trade_seq": 253672185,
                        "mark_price": 111358.33,
                        "tick_direction": 1,
                        "trade_id": "374887289",
                        "contracts": 20.0
                    }
                ],
                "has_more": true
            }
        }"#;
        let resp: GetLastTradesByInstrumentAndTimeResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 21);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.trades.len(), 1);
        assert!(resp.result.has_more);
        let trade = &resp.result.trades[0];
        assert_eq!(trade.trade_id, "374887289");
        assert!((trade.price - 111356.0).abs() < 1e-8);
        assert!((trade.amount - 200.0).abs() < 1e-8);
        assert_eq!(trade.timestamp, 1752128880144);
        assert_eq!(trade.tick_direction, 1);
        assert_eq!(trade.direction, "buy");
        assert!((trade.index_price - 111341.24).abs() < 1e-8);
        assert!((trade.mark_price - 111358.33).abs() < 1e-8);
        assert_eq!(trade.instrument_name, "BTC-PERPETUAL");
    }
}
