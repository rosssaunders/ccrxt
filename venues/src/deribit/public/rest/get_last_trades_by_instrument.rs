//! Implements the /public/get_last_trades_by_instrument endpoint for Deribit.
//!
//! Retrieves the most recent trades for a given instrument.

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{
    EndpointType, JsonRpcResult, RestResult,
    enums::{Liquidity, Sorting, TickDirection, TradeOrderType},
};

const LAST_TRADES_BY_INSTRUMENT_ENDPOINT: &str = "public/get_last_trades_by_instrument";

/// Request parameters for the get_last_trades_by_instrument endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetLastTradesByInstrumentRequest {
    /// Instrument name for which to retrieve trades.
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,

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

/// The result object for get_last_trades_by_instrument.
#[derive(Debug, Clone, Deserialize)]
pub struct GetLastTradesByInstrumentResult {
    /// List of trade entries.
    #[serde(rename = "trades")]
    pub trades: Vec<TradeEntry>,
}

/// Response for the get_last_trades_by_instrument endpoint.
pub type GetLastTradesByInstrumentResponse = JsonRpcResult<GetLastTradesByInstrumentResult>;

impl RestClient {
    /// Calls the /public/get_last_trades_by_instrument endpoint.
    ///
    /// Retrieves the most recent trades for a given instrument.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_last_trades_by_instrument)
    pub async fn get_last_trades_by_instrument(
        &self,
        params: GetLastTradesByInstrumentRequest,
    ) -> RestResult<GetLastTradesByInstrumentResponse> {
        self.send_request(
            LAST_TRADES_BY_INSTRUMENT_ENDPOINT,
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
    use crate::deribit::enums::{Liquidity, Sorting, TickDirection, TradeOrderType};

    #[test]
    fn test_serialize_request() {
        let req = GetLastTradesByInstrumentRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
            count: Some(2),
            sorting: Some(Sorting::Desc),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC-PERPETUAL"));
        assert!(json.contains("desc"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 20,
            "jsonrpc": "2.0",
            "result": {
                "trades": [
                    {
                        "trade_id": "123458",
                        "price": 65000.0,
                        "amount": 0.3,
                        "timestamp": 1680310800000,
                        "tick_direction": "2",
                        "liquidity": "T",
                        "order_type": "market",
                        "instrument_name": "BTC-PERPETUAL"
                    }
                ]
            }
        }"#;
        let resp: GetLastTradesByInstrumentResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 20);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.trades.len(), 1);
        let trade = &resp.result.trades[0];
        assert_eq!(trade.trade_id, "123458");
        assert!((trade.price - 65000.0).abs() < 1e-8);
        assert!((trade.amount - 0.3).abs() < 1e-8);
        assert_eq!(trade.timestamp, 1680310800000);
        assert_eq!(trade.tick_direction, TickDirection::MinusTick);
        assert_eq!(trade.liquidity, Liquidity::Taker);
        assert_eq!(trade.order_type, TradeOrderType::Market);
        assert_eq!(trade.instrument_name, "BTC-PERPETUAL");
    }
}
