//! Implements the /public/get_last_trades_by_currency endpoint for Deribit.
//!
//! Retrieves the most recent trades for a given currency and instrument kind.

use super::RestClient;
use crate::deribit::RestResult;
use crate::deribit::enums::{Currency, InstrumentKind, Liquidity, Sorting, TickDirection, TradeOrderType};

use serde::{Deserialize, Serialize};

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

/// The result object for get_last_trades_by_currency.
#[derive(Debug, Clone, Deserialize)]
pub struct GetLastTradesByCurrencyResult {
    /// List of trade entries.
    #[serde(rename = "trades")]
    pub trades: Vec<TradeEntry>,
}

/// Response for the get_last_trades_by_currency endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetLastTradesByCurrencyResponse {
    /// The id that was sent in the request.
    #[serde(rename = "id")]
    pub id: u64,

    /// The JSON-RPC version (2.0).
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: String,

    /// The result object containing the trades.
    #[serde(rename = "result")]
    pub result: GetLastTradesByCurrencyResult,
}

impl RestClient {
    /// Calls the /public/get_last_trades_by_currency endpoint.
    ///
    /// Retrieves the most recent trades for a given currency and instrument kind.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_last_trades_by_currency)
    pub async fn get_last_trades_by_currency(&self, params: GetLastTradesByCurrencyRequest) -> RestResult<GetLastTradesByCurrencyResponse> {
        self.call_public("get_last_trades_by_currency", &params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deribit::enums::{Currency, InstrumentKind, Liquidity, Sorting, TickDirection, TradeOrderType};
    use serde_json;

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
            \"id\": 18,
            \"jsonrpc\": \"2.0\",
            \"result\": {
                \"trades\": [
                    {
                        \"trade_id\": \"123456\",
                        \"price\": 65000.0,
                        \"amount\": 0.1,
                        \"timestamp\": 1680310800000,
                        \"tick_direction\": "0",
                        \"liquidity\": "M",
                        \"order_type\": "limit",
                        \"instrument_name\": "BTC-PERPETUAL"
                    }
                ]
            }
        }"#;
        let resp: GetLastTradesByCurrencyResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 18);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.trades.len(), 1);
        let trade = &resp.result.trades[0];
        assert_eq!(trade.trade_id, "123456");
        assert!((trade.price - 65000.0).abs() < 1e-8);
        assert!((trade.amount - 0.1).abs() < 1e-8);
        assert_eq!(trade.timestamp, 1680310800000);
        assert_eq!(trade.tick_direction, TickDirection::PlusTick);
        assert_eq!(trade.liquidity, Liquidity::Maker);
        assert_eq!(trade.order_type, TradeOrderType::Limit);
        assert_eq!(trade.instrument_name, "BTC-PERPETUAL");
    }
}
