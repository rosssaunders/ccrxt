//! Implements the /public/get_book_summary_by_currency endpoint for Deribit.
//!
//! Retrieves the summary information such as open interest, 24h volume, etc. for all instruments for the currency (optionally filtered by kind).

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{
    EndpointType, JsonRpcResult, RestResult,
    enums::{Currency, InstrumentKind},
};

const BOOK_SUMMARY_BY_CURRENCY_ENDPOINT: &str = "public/get_book_summary_by_currency";

/// Request parameters for the get_book_summary_by_currency endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct GetBookSummaryByCurrencyRequest {
    /// The currency symbol.
    #[serde(rename = "currency")]
    pub currency: Currency,

    /// Instrument kind, if not provided instruments of all kinds are considered.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<InstrumentKind>,
}

/// Represents a single book summary entry for an instrument.
#[derive(Debug, Clone, Deserialize)]
pub struct BookSummary {
    /// The current best ask price, null if there aren't any asks.
    #[serde(rename = "ask_price")]
    pub ask_price: Option<f64>,

    /// Base currency.
    #[serde(rename = "base_currency")]
    pub base_currency: Currency,

    /// The current best bid price, null if there aren't any bids.
    #[serde(rename = "bid_price")]
    pub bid_price: Option<f64>,

    /// The timestamp (milliseconds since the Unix epoch).
    #[serde(rename = "creation_timestamp")]
    pub creation_timestamp: u64,

    /// Current funding (perpetual only).
    #[serde(rename = "current_funding")]
    pub current_funding: Option<f64>,

    /// Estimated delivery price for the market (derivatives only).
    #[serde(rename = "estimated_delivery_price")]
    pub estimated_delivery_price: Option<f64>,

    /// Funding 8h (perpetual only).
    #[serde(rename = "funding_8h")]
    pub funding_8h: Option<f64>,

    /// Price of the 24h highest trade.
    #[serde(rename = "high")]
    pub high: Option<f64>,

    /// Unique instrument identifier.
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,

    /// Interest rate used in implied volatility calculations (options only).
    #[serde(rename = "interest_rate")]
    pub interest_rate: Option<f64>,

    /// The price of the latest trade, null if there weren't any trades.
    #[serde(rename = "last")]
    pub last: Option<f64>,

    /// Price of the 24h lowest trade, null if there weren't any trades.
    #[serde(rename = "low")]
    pub low: Option<f64>,

    /// (Only for option) implied volatility for mark price.
    #[serde(rename = "mark_iv")]
    pub mark_iv: Option<f64>,

    /// The current instrument market price.
    #[serde(rename = "mark_price")]
    pub mark_price: f64,

    /// The average of the best bid and ask, null if there aren't any asks or bids.
    #[serde(rename = "mid_price")]
    pub mid_price: Option<f64>,

    /// The total amount of outstanding contracts (derivatives only).
    #[serde(rename = "open_interest")]
    pub open_interest: Option<f64>,

    /// 24-hour price change expressed as a percentage, null if there weren't any trades.
    #[serde(rename = "price_change")]
    pub price_change: Option<f64>,

    /// Quote currency.
    #[serde(rename = "quote_currency")]
    pub quote_currency: Currency,

    /// Name of the underlying future, or 'index_price' (options only).
    #[serde(rename = "underlying_index")]
    pub underlying_index: Option<String>,

    /// Underlying price for implied volatility calculations (options only).
    #[serde(rename = "underlying_price")]
    pub underlying_price: Option<f64>,

    /// The total 24h traded volume (in base currency).
    #[serde(rename = "volume")]
    pub volume: f64,

    /// Volume in quote currency (futures and spots only).
    #[serde(rename = "volume_notional")]
    pub volume_notional: Option<f64>,

    /// Volume in USD.
    #[serde(rename = "volume_usd")]
    pub volume_usd: Option<f64>,
}

pub type GetBookSummaryByCurrencyResponse = JsonRpcResult<Vec<BookSummary>>;

impl RestClient {
    /// Calls the /public/get_book_summary_by_currency endpoint.
    ///
    /// Retrieves the summary information such as open interest, 24h volume, etc. for all instruments for the currency (optionally filtered by kind).
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_book_summary_by_currency)
    pub async fn get_book_summary_by_currency(
        &self,
        params: GetBookSummaryByCurrencyRequest,
    ) -> RestResult<GetBookSummaryByCurrencyResponse> {
        self.send_request(
            BOOK_SUMMARY_BY_CURRENCY_ENDPOINT,
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
        let req = GetBookSummaryByCurrencyRequest {
            currency: Currency::BTC,
            kind: Some(InstrumentKind::Future),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC"));
        assert!(json.contains("future"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 1,
            "jsonrpc": "2.0",
            "result": [
                {
                    "ask_price": 100.0,
                    "base_currency": "BTC",
                    "bid_price": 99.0,
                    "creation_timestamp": 1234567890,
                    "current_funding": 0.01,
                    "estimated_delivery_price": 101.0,
                    "funding_8h": 0.02,
                    "high": 110.0,
                    "instrument_name": "BTC-PERPETUAL",
                    "interest_rate": 0.05,
                    "last": 100.5,
                    "low": 90.0,
                    "mark_iv": 0.6,
                    "mark_price": 100.2,
                    "mid_price": 99.5,
                    "open_interest": 10000.0,
                    "price_change": 0.01,
                    "quote_currency": "USDC",
                    "underlying_index": "BTC-USD",
                    "underlying_price": 100.0,
                    "volume": 500.0,
                    "volume_notional": 50000.0,
                    "volume_usd": 50000.0
                }
            ]
        }"#;
        let resp: GetBookSummaryByCurrencyResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 1);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.len(), 1);
        assert_eq!(resp.result[0].instrument_name, "BTC-PERPETUAL");
    }
}
