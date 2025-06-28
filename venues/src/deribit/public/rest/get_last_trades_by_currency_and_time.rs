//! Implements the /public/get_last_trades_by_currency_and_time endpoint for Deribit.
//!
//! Retrieves the most recent trades for a given currency and instrument kind, filtered by start and end timestamps.

use super::RestClient;
use crate::deribit::enums::{
    Currency, InstrumentKind, Liquidity, Sorting, TickDirection, TradeOrderType,
};
use crate::deribit::{EndpointType, RestResult};

use reqwest::Method;
use serde::{Deserialize, Serialize};

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

    /// Tick direction (enum).
    #[serde(rename = "tick_direction")]
    pub tick_direction: TickDirection,

    /// Liquidity role (maker/taker).
    #[serde(rename = "liquidity")]
    pub liquidity: Liquidity,

    /// Trade order type (limit/market/liquidation).
    #[serde(rename = "order_type")]
    pub order_type: TradeOrderType,

    /// Instrument name.
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
}

/// The result object for get_last_trades_by_currency_and_time.
#[derive(Debug, Clone, Deserialize)]
pub struct GetLastTradesByCurrencyAndTimeResult {
    /// List of trade entries.
    #[serde(rename = "trades")]
    pub trades: Vec<TradeEntry>,
}

/// Response for the get_last_trades_by_currency_and_time endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetLastTradesByCurrencyAndTimeResponse {
    /// The id that was sent in the request.
    #[serde(rename = "id")]
    pub id: u64,

    /// The JSON-RPC version (2.0).
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: String,

    /// The result object containing the trades.
    #[serde(rename = "result")]
    pub result: GetLastTradesByCurrencyAndTimeResult,
}

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
            "get_last_trades_by_currency_and_time",
            Method::POST,
            Some(&params),
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deribit::enums::{
        Currency, InstrumentKind, Liquidity, Sorting, TickDirection, TradeOrderType,
    };
    use serde_json;

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
                        "tick_direction": "1",
                        "liquidity": "T",
                        "order_type": "market",
                        "instrument_name": "BTC-PERPETUAL"
                    }
                ]
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
        assert_eq!(trade.tick_direction, TickDirection::ZeroPlusTick);
        assert_eq!(trade.liquidity, Liquidity::Taker);
        assert_eq!(trade.order_type, TradeOrderType::Market);
        assert_eq!(trade.instrument_name, "BTC-PERPETUAL");
    }
}
