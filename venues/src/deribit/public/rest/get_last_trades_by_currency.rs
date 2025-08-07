//! Implements the /public/get_last_trades_by_currency endpoint for Deribit.
//!
//! Retrieves the most recent trades for a given currency and instrument kind.

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{
    EndpointType, JsonRpcResult, RestResult,
    enums::{Currency, InstrumentKind, Sorting},
};

const LAST_TRADES_BY_CURRENCY_ENDPOINT: &str = "public/get_last_trades_by_currency";

/// Request parameters for the get_last_trades_by_currency endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct GetLastTradesByCurrencyRequest {
    /// Currency for which to retrieve trades.
    #[serde(rename = "currency")]
    pub currency: Currency,

    /// Kind of the instrument (future, option, etc.).
    #[serde(rename = "kind")]
    pub kind: InstrumentKind,

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

/// The result object for get_last_trades_by_currency.
#[derive(Debug, Clone, Deserialize)]
pub struct GetLastTradesByCurrencyResult {
    /// List of trade entries.
    #[serde(rename = "trades")]
    pub trades: Vec<TradeEntry>,

    /// Whether there are more trades available.
    #[serde(rename = "has_more")]
    pub has_more: bool,
}

/// Response for the get_last_trades_by_currency endpoint.
pub type GetLastTradesByCurrencyResponse = JsonRpcResult<GetLastTradesByCurrencyResult>;

impl RestClient {
    /// Calls the /public/get_last_trades_by_currency endpoint.
    ///
    /// Retrieves the most recent trades for a given currency and instrument kind.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_last_trades_by_currency)
    pub async fn get_last_trades_by_currency(
        &self,
        params: GetLastTradesByCurrencyRequest,
    ) -> RestResult<GetLastTradesByCurrencyResponse> {
    self.send_post_request(
            LAST_TRADES_BY_CURRENCY_ENDPOINT,
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
        let req = GetLastTradesByCurrencyRequest {
            currency: Currency::BTC,
            kind: InstrumentKind::Future,
            count: Some(2),
            sorting: Some(Sorting::Desc),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC"));
        assert!(json.contains("future"));
        assert!(json.contains("desc"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 18,
            "jsonrpc": "2.0",
            "result": {
                "trades": [
                    {
                        "trade_id": "374718632",
                        "price": 109133.0,
                        "amount": 580.0,
                        "timestamp": 1751999756793,
                        "direction": "buy",
                        "index_price": 109093.57,
                        "instrument_name": "BTC-PERPETUAL",
                        "trade_seq": 253575835,
                        "mark_price": 109138.66,
                        "tick_direction": 0,
                        "contracts": 58.0
                    }
                ],
                "has_more": true
            }
        }"#;
        let resp: GetLastTradesByCurrencyResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 18);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.trades.len(), 1);
        assert!(resp.result.has_more);
        let trade = &resp.result.trades[0];
        assert_eq!(trade.trade_id, "374718632");
        assert!((trade.price - 109133.0).abs() < 1e-8);
        assert!((trade.amount - 580.0).abs() < 1e-8);
        assert_eq!(trade.timestamp, 1751999756793);
        assert_eq!(trade.direction, "buy");
        assert!((trade.index_price - 109093.57).abs() < 1e-8);
        assert_eq!(trade.instrument_name, "BTC-PERPETUAL");
        assert_eq!(trade.trade_seq, 253575835);
        assert!((trade.mark_price - 109138.66).abs() < 1e-8);
        assert_eq!(trade.tick_direction, 0);
        assert!((trade.contracts - 58.0).abs() < 1e-8);
    }
}
