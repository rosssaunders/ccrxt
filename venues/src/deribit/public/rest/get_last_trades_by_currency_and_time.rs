//! Implements the /public/get_last_trades_by_currency_and_time endpoint for Deribit.
//!
//! Retrieves the most recent trades for a given currency and instrument kind, filtered by start and end timestamps.

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{
    EndpointType, JsonRpcResult, RestResult,
    enums::{Currency, InstrumentKind, Sorting},
};

const LAST_TRADES_BY_CURRENCY_AND_TIME_ENDPOINT: &str =
    "public/get_last_trades_by_currency_and_time";

/// Request parameters for the get_last_trades_by_currency_and_time endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct GetLastTradesByCurrencyAndTimeRequest {
    /// Currency for which to retrieve trades.
    #[serde(rename = "currency")]
    pub currency: Currency,

    /// Kind of the instrument (future, option, etc.).
    #[serde(rename = "kind")]
    pub kind: InstrumentKind,

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

    /// Trade direction (buy/sell).
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

    /// Tick direction (0, 1, 2, 3).
    #[serde(rename = "tick_direction")]
    pub tick_direction: u8,

    /// Number of contracts traded.
    #[serde(rename = "contracts")]
    pub contracts: f64,
}

/// The result object for get_last_trades_by_currency_and_time.
#[derive(Debug, Clone, Deserialize)]
pub struct GetLastTradesByCurrencyAndTimeResult {
    /// List of trade entries.
    #[serde(rename = "trades")]
    pub trades: Vec<TradeEntry>,

    /// Whether there are more trades available.
    #[serde(rename = "has_more")]
    pub has_more: bool,
}

/// Response for the get_last_trades_by_currency_and_time endpoint.
pub type GetLastTradesByCurrencyAndTimeResponse =
    JsonRpcResult<GetLastTradesByCurrencyAndTimeResult>;

impl RestClient {
    /// Calls the /public/get_last_trades_by_currency_and_time endpoint.
    ///
    /// Retrieves the most recent trades for a given currency and instrument kind, filtered by start and end timestamps.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_last_trades_by_currency_and_time)
    pub async fn get_last_trades_by_currency_and_time(
        &self,
        params: GetLastTradesByCurrencyAndTimeRequest,
    ) -> RestResult<GetLastTradesByCurrencyAndTimeResponse> {
        self.send_request(
            LAST_TRADES_BY_CURRENCY_AND_TIME_ENDPOINT,
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
    use crate::deribit::enums::{Currency, InstrumentKind, Sorting};

    #[test]
    fn test_serialize_request() {
        let req = GetLastTradesByCurrencyAndTimeRequest {
            currency: Currency::BTC,
            kind: InstrumentKind::Future,
            start_timestamp: 1680310000000,
            end_timestamp: 1680310800000,
            count: Some(2),
            sorting: Some(Sorting::Asc),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC"));
        assert!(json.contains("future"));
        assert!(json.contains("1680310000000"));
        assert!(json.contains("1680310800000"));
        assert!(json.contains("asc"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 19,
            "jsonrpc": "2.0",
            "result": {
                "trades": [
                    {
                        "trade_id": "123457",
                        "price": 65000.0,
                        "amount": 0.2,
                        "timestamp": 1680310800000,
                        "direction": "buy",
                        "index_price": 64950.0,
                        "instrument_name": "BTC-PERPETUAL",
                        "trade_seq": 123456789,
                        "mark_price": 65000.5,
                        "tick_direction": 1,
                        "contracts": 2.0
                    }
                ],
                "has_more": false
            }
        }"#;
        let resp: GetLastTradesByCurrencyAndTimeResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 19);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.trades.len(), 1);
        let trade = &resp.result.trades[0];
        assert_eq!(trade.trade_id, "123457");
        assert!((trade.price - 65000.0).abs() < 1e-8);
        assert!((trade.amount - 0.2).abs() < 1e-8);
        assert_eq!(trade.timestamp, 1680310800000);
        assert_eq!(trade.direction, "buy");
        assert!((trade.index_price - 64950.0).abs() < 1e-8);
        assert_eq!(trade.instrument_name, "BTC-PERPETUAL");
        assert_eq!(trade.trade_seq, 123456789);
        assert!((trade.mark_price - 65000.5).abs() < 1e-8);
        assert_eq!(trade.tick_direction, 1);
        assert!((trade.contracts - 2.0).abs() < 1e-8);
    }
}
