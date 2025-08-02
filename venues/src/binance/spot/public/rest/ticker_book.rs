use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::RestResult;

const TICKER_BOOK_ENDPOINT: &str = "/api/v3/ticker/bookTicker";

/// Request parameters for symbol order book ticker
#[derive(Debug, Clone, Serialize, Default)]
pub struct TickerBookRequest {
    /// Single symbol
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Multiple symbols (JSON array format)
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
}

/// Symbol order book ticker
#[derive(Debug, Clone, Deserialize)]
pub struct TickerBook {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Best bid price
    #[serde(rename = "bidPrice")]
    pub bid_price: Decimal,

    /// Best bid quantity
    #[serde(rename = "bidQty")]
    pub bid_qty: Decimal,

    /// Best ask price
    #[serde(rename = "askPrice")]
    pub ask_price: Decimal,

    /// Best ask quantity
    #[serde(rename = "askQty")]
    pub ask_qty: Decimal,
}

impl RestClient {
    /// Get symbol order book ticker
    ///
    /// Best price/qty on the order book for a symbol or symbols.
    ///
    /// [docs]: (https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#symbol-order-book-ticker)
    /// Method: GET /api/v3/ticker/bookTicker
    /// Weight: 2 for single symbol, 4 for multiple symbols
    /// Security: None
    pub async fn get_book_ticker(
        &self,
        params: Option<TickerBookRequest>,
    ) -> RestResult<serde_json::Value> {
        let weight = if let Some(ref p) = params {
            if p.symbol.is_some() {
                2 // Single symbol
            } else {
                4 // Multiple symbols or default
            }
        } else {
            4 // All symbols
        };

        self.send_public_request(TICKER_BOOK_ENDPOINT, reqwest::Method::GET, params, weight)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticker_book_request_serialization_empty() {
        let request = TickerBookRequest::default();
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_ticker_book_request_serialization_single_symbol() {
        let request = TickerBookRequest {
            symbol: Some("BTCUSDT".to_string()),
            symbols: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSDT");
    }

    #[test]
    fn test_ticker_book_request_serialization_multiple_symbols() {
        let request = TickerBookRequest {
            symbol: None,
            symbols: Some("[\"BTCUSDT\",\"ETHUSDT\"]".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbols=%5B%22BTCUSDT%22%2C%22ETHUSDT%22%5D"));
    }

    #[test]
    fn test_ticker_book_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "bidPrice": "45000.00000000",
            "bidQty": "2.50000000",
            "askPrice": "45001.00000000",
            "askQty": "1.80000000"
        }"#;

        let ticker: TickerBook = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "BTCUSDT");
        assert_eq!(ticker.bid_price.to_string(), "45000.00000000");
        assert_eq!(ticker.bid_qty.to_string(), "2.50000000");
        assert_eq!(ticker.ask_price.to_string(), "45001.00000000");
        assert_eq!(ticker.ask_qty.to_string(), "1.80000000");
    }

    #[test]
    fn test_ticker_book_array_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "bidPrice": "45000.00000000",
                "bidQty": "2.50000000",
                "askPrice": "45001.00000000",
                "askQty": "1.80000000"
            },
            {
                "symbol": "ETHUSDT",
                "bidPrice": "3070.00000000",
                "bidQty": "10.00000000",
                "askPrice": "3070.50000000",
                "askQty": "8.50000000"
            }
        ]"#;

        let tickers: Vec<TickerBook> = serde_json::from_str(json).unwrap();
        assert_eq!(tickers.len(), 2);

        assert_eq!(tickers[0].symbol, "BTCUSDT");
        assert_eq!(tickers[0].bid_price.to_string(), "45000.00000000");
        assert_eq!(tickers[0].ask_price.to_string(), "45001.00000000");

        assert_eq!(tickers[1].symbol, "ETHUSDT");
        assert_eq!(tickers[1].bid_price.to_string(), "3070.00000000");
        assert_eq!(tickers[1].ask_price.to_string(), "3070.50000000");
    }

    #[test]
    fn test_ticker_book_tight_spread() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "bidPrice": "45000.00000000",
            "bidQty": "5.00000000",
            "askPrice": "45000.01000000",
            "askQty": "5.00000000"
        }"#;

        let ticker: TickerBook = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.bid_price.to_string(), "45000.00000000");
        assert_eq!(ticker.ask_price.to_string(), "45000.01000000");
        // Very tight spread of 0.01
    }

    #[test]
    fn test_ticker_book_high_precision_altcoin() {
        let json = r#"{
            "symbol": "DOGEUSDT",
            "bidPrice": "0.12345678",
            "bidQty": "100000.00000000",
            "askPrice": "0.12345679",
            "askQty": "50000.00000000"
        }"#;

        let ticker: TickerBook = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "DOGEUSDT");
        assert_eq!(ticker.bid_price.to_string(), "0.12345678");
        assert_eq!(ticker.bid_qty.to_string(), "100000.00000000");
        assert_eq!(ticker.ask_price.to_string(), "0.12345679");
        assert_eq!(ticker.ask_qty.to_string(), "50000.00000000");
    }

    #[test]
    fn test_ticker_book_zero_quantity() {
        let json = r#"{
            "symbol": "RAREUSDT",
            "bidPrice": "1.23456789",
            "bidQty": "0.00000000",
            "askPrice": "1.23456790",
            "askQty": "0.00000000"
        }"#;

        let ticker: TickerBook = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.bid_qty.to_string(), "0.00000000");
        assert_eq!(ticker.ask_qty.to_string(), "0.00000000");
    }

    #[test]
    fn test_ticker_book_large_quantities() {
        let json = r#"{
            "symbol": "SHIBUSDT",
            "bidPrice": "0.00001234",
            "bidQty": "10000000.00000000",
            "askPrice": "0.00001235",
            "askQty": "5000000.00000000"
        }"#;

        let ticker: TickerBook = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.bid_qty.to_string(), "10000000.00000000");
        assert_eq!(ticker.ask_qty.to_string(), "5000000.00000000");
    }

    #[test]
    fn test_ticker_book_request_multiple_symbols_json() {
        let request = TickerBookRequest {
            symbol: None,
            symbols: Some("[\"BTCUSDT\",\"ETHUSDT\",\"ADAUSDT\"]".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbols="));
        assert!(!serialized.contains("symbol="));
    }

    #[test]
    fn test_ticker_book_equal_bid_ask() {
        let json = r#"{
            "symbol": "STABLEUSDT",
            "bidPrice": "1.00000000",
            "bidQty": "1000.00000000",
            "askPrice": "1.00000000",
            "askQty": "1000.00000000"
        }"#;

        let ticker: TickerBook = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.bid_price, ticker.ask_price);
        assert_eq!(ticker.bid_price.to_string(), "1.00000000");
    }
}
