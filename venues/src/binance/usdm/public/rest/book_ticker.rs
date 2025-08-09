use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::RestResult;

/// Endpoint for getting best price/quantity on the order book.
const BOOK_TICKER_ENDPOINT: &str = "/fapi/v1/ticker/bookTicker";

/// Request parameters for the symbol order book ticker endpoint.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BookTickerRequest {
    /// Trading pair symbol (e.g., "BTCUSDT"). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Cow<'static, str>>,
}

/// Represents a symbol order book ticker response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookTicker {
    /// Trading pair symbol.
    pub symbol: Cow<'static, str>,

    /// Bid price.
    pub bid_price: String,

    /// Bid quantity.
    pub bid_qty: String,

    /// Ask price.
    pub ask_price: String,

    /// Ask quantity.
    pub ask_qty: String,

    /// Timestamp (milliseconds since epoch). Optional.
    pub time: Option<u64>,

    /// Last update ID. Optional.
    pub last_update_id: Option<u64>,
}

/// Result for the book ticker endpoint, either a single ticker or multiple tickers.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum BookTickerResult {
    /// Multiple tickers for multiple symbols.
    Multiple(Vec<BookTicker>),

    /// Single ticker for one symbol.
    Single(BookTicker),
}

impl RestClient {
    /// Symbol Order Book Ticker
    ///
    /// Get best price/quantity on the order book for a symbol or symbols.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Order-Book-Ticker
    ///
    /// Rate limit: 2
    ///
    /// # Arguments
    /// * `params` - `BookTickerRequest` containing an optional symbol.
    ///
    /// # Returns
    /// `BookTickerResult` in a `RestResult`.
    pub async fn get_book_ticker(&self, params: BookTickerRequest) -> RestResult<BookTickerResult> {
        self.send_get_request(BOOK_TICKER_ENDPOINT, Some(params), 2)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_ticker_request_serialization() {
        let request = BookTickerRequest {
            symbol: Some("BTCUSDT".into()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSDT");
    }

    #[test]
    fn test_book_ticker_request_no_symbol() {
        let request = BookTickerRequest { symbol: None };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_book_ticker_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "bidPrice": "45380.10",
            "bidQty": "2.500",
            "askPrice": "45380.20",
            "askQty": "1.200",
            "time": 1625184000000,
            "lastUpdateId": 12345678
        }"#;

        let ticker: BookTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "BTCUSDT");
        assert_eq!(ticker.bid_price, "45380.10");
        assert_eq!(ticker.bid_qty, "2.500");
        assert_eq!(ticker.ask_price, "45380.20");
        assert_eq!(ticker.ask_qty, "1.200");
        assert_eq!(ticker.time, Some(1625184000000));
        assert_eq!(ticker.last_update_id, Some(12345678));
    }

    #[test]
    fn test_book_ticker_response_without_optional_fields() {
        let json = r#"{
            "symbol": "ETHUSDT",
            "bidPrice": "3070.50",
            "bidQty": "10.000",
            "askPrice": "3070.60",
            "askQty": "8.500"
        }"#;

        let ticker: BookTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "ETHUSDT");
        assert_eq!(ticker.bid_price, "3070.50");
        assert_eq!(ticker.bid_qty, "10.000");
        assert_eq!(ticker.ask_price, "3070.60");
        assert_eq!(ticker.ask_qty, "8.500");
        assert_eq!(ticker.time, None);
        assert_eq!(ticker.last_update_id, None);
    }

    #[test]
    fn test_book_ticker_result_single_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "bidPrice": "45380.10",
            "bidQty": "2.500",
            "askPrice": "45380.20",
            "askQty": "1.200",
            "time": 1625184000000,
            "lastUpdateId": 12345678
        }"#;

        let result: BookTickerResult = serde_json::from_str(json).unwrap();
        match result {
            BookTickerResult::Single(ticker) => {
                assert_eq!(ticker.symbol, "BTCUSDT");
                assert_eq!(ticker.bid_price, "45380.10");
            }
            BookTickerResult::Multiple(_) => panic!("Expected Single variant"),
        }
    }

    #[test]
    fn test_book_ticker_result_multiple_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "bidPrice": "45380.10",
                "bidQty": "2.500",
                "askPrice": "45380.20",
                "askQty": "1.200",
                "time": 1625184000000,
                "lastUpdateId": 12345678
            },
            {
                "symbol": "ETHUSDT",
                "bidPrice": "3070.50",
                "bidQty": "10.000",
                "askPrice": "3070.60",
                "askQty": "8.500",
                "time": 1625184000000,
                "lastUpdateId": 12345679
            }
        ]"#;

        let result: BookTickerResult = serde_json::from_str(json).unwrap();
        match result {
            BookTickerResult::Multiple(tickers) => {
                assert_eq!(tickers.len(), 2);
                assert_eq!(tickers[0].symbol, "BTCUSDT");
                assert_eq!(tickers[0].bid_price, "45380.10");
                assert_eq!(tickers[1].symbol, "ETHUSDT");
                assert_eq!(tickers[1].bid_price, "3070.50");
            }
            BookTickerResult::Single(_) => panic!("Expected Multiple variant"),
        }
    }

    #[test]
    fn test_book_ticker_tight_spread() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "bidPrice": "45380.10",
            "bidQty": "100.000",
            "askPrice": "45380.11",
            "askQty": "100.000"
        }"#;

        let ticker: BookTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.bid_price, "45380.10");
        assert_eq!(ticker.ask_price, "45380.11");
        // Very tight spread of 0.01
    }
}
