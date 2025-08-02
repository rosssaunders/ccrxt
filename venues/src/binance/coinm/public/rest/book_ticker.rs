use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

/// Symbol Order Book Ticker endpoint path
const BOOK_TICKER_ENDPOINT: &str = "/dapi/v1/ticker/bookTicker";

/// Parameters for Symbol Order Book Ticker by symbol.
#[derive(Debug, Clone, Serialize, Default)]
pub struct BookTickerRequestBySymbol {
    /// Symbol name (optional). When provided, returns ticker for a single symbol.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

/// Parameters for Symbol Order Book Ticker by pair.
#[derive(Debug, Clone, Serialize, Default)]
pub struct BookTickerRequestByPair {
    /// Pair name (optional). When provided, returns ticker for a single pair.
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
}

/// Request parameters for the Symbol Order Book Ticker endpoint.
#[derive(Debug, Clone, Serialize)]
pub enum BookTickerRequest {
    /// Request by symbol
    BySymbol(BookTickerRequestBySymbol),
    /// Request by pair
    ByPair(BookTickerRequestByPair),
}

/// Symbol order book ticker
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookTicker {
    /// Symbol name
    pub symbol: String,

    /// Pair name
    pub pair: String,

    /// Best bid price
    pub bid_price: Decimal,

    /// Best bid quantity
    pub bid_qty: Decimal,

    /// Best ask price
    pub ask_price: Decimal,

    /// Best ask quantity
    pub ask_qty: Decimal,

    /// Timestamp
    pub time: i64,
}

impl RestClient {
    /// Get symbol order book ticker
    ///
    /// Retrieves the best bid and ask price and quantity for a given symbol or pair.
    ///
    /// [docs]:  https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Symbol-Order-Book-Ticker
    ///
    /// Rate limit: 2 (single symbol/pair), 5 (all symbols/pairs)
    ///
    /// # Arguments
    /// * `params` - The `BookTickerRequest` specifying a symbol or pair filter.
    ///
    /// # Returns
    /// A list of `BookTicker` records for the requested symbol(s) or pair(s).
    pub async fn get_book_ticker(&self, params: BookTickerRequest) -> RestResult<Vec<BookTicker>> {
        match params {
            BookTickerRequest::BySymbol(by_symbol) => {
                let weight = if by_symbol.symbol.is_some() { 2 } else { 5 };
                self.send_request(
                    BOOK_TICKER_ENDPOINT,
                    reqwest::Method::GET,
                    Some(by_symbol),
                    weight,
                )
                .await
            }
            BookTickerRequest::ByPair(by_pair) => {
                let weight = if by_pair.pair.is_some() { 2 } else { 5 };
                self.send_request(
                    BOOK_TICKER_ENDPOINT,
                    reqwest::Method::GET,
                    Some(by_pair),
                    weight,
                )
                .await
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::prelude::FromPrimitive;

    use super::*;

    #[test]
    fn test_book_ticker_request_by_symbol_serialization() {
        let request = BookTickerRequestBySymbol {
            symbol: Some("BTCUSD_PERP".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSD_PERP");
    }

    #[test]
    fn test_book_ticker_request_by_symbol_empty_serialization() {
        let request = BookTickerRequestBySymbol { symbol: None };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_book_ticker_request_by_pair_serialization() {
        let request = BookTickerRequestByPair {
            pair: Some("BTCUSD".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "pair=BTCUSD");
    }

    #[test]
    fn test_book_ticker_deserialization() {
        let json = r#"{
            "symbol": "BTCUSD_PERP",
            "pair": "BTCUSD",
            "bidPrice": 50000.25,
            "bidQty": 10.5,
            "askPrice": 50001.00,
            "askQty": 8.25,
            "time": 1625097600000
        }"#;

        let ticker: BookTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "BTCUSD_PERP");
        assert_eq!(ticker.pair, "BTCUSD");
        assert_eq!(ticker.bid_price, Decimal::from_f64(50000.25).unwrap());
        assert_eq!(ticker.bid_qty, Decimal::from_f64(10.5).unwrap());
        assert_eq!(ticker.ask_price, Decimal::from_f64(50001.00).unwrap());
        assert_eq!(ticker.ask_qty, Decimal::from_f64(8.25).unwrap());
        assert_eq!(ticker.time, 1625097600000);
    }

    #[test]
    fn test_book_ticker_list_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSD_PERP",
                "pair": "BTCUSD",
                "bidPrice": 50000.25,
                "bidQty": 10.5,
                "askPrice": 50001.00,
                "askQty": 8.25,
                "time": 1625097600000
            },
            {
                "symbol": "ETHUSD_PERP",
                "pair": "ETHUSD",
                "bidPrice": 3000.50,
                "bidQty": 100.0,
                "askPrice": 3001.00,
                "askQty": 95.5,
                "time": 1625097600000
            }
        ]"#;

        let tickers: Vec<BookTicker> = serde_json::from_str(json).unwrap();
        assert_eq!(tickers.len(), 2);

        assert_eq!(tickers[0].symbol, "BTCUSD_PERP");
        assert_eq!(tickers[0].bid_price, Decimal::from_f64(50000.25).unwrap());

        assert_eq!(tickers[1].symbol, "ETHUSD_PERP");
        assert_eq!(tickers[1].ask_price, Decimal::from_f64(3001.00).unwrap());
    }
}
