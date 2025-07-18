use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

/// Parameters for Symbol Order Book Ticker
#[derive(Debug, Clone, Serialize, Default)]
pub struct BookTickerRequestBySymbol {
    /// Symbol name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

/// Parameters for Symbol Order Book Ticker
#[derive(Debug, Clone, Serialize, Default)]
pub struct BookTickerRequestByPair {
    /// Pair name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
}

#[derive(Serialize)]
pub enum BookTickerRequest {
    BySymbol(BookTickerRequestBySymbol),
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
    /// Get symbol order book ticker by symbol
    ///
    /// Weight: 2 for a single symbol; 5 when the symbol parameter is omitted
    async fn get_book_ticker_by_symbol(
        &self,
        params: BookTickerRequestBySymbol,
    ) -> RestResult<Vec<BookTicker>> {
        let weight = if params.symbol.is_some() { 2 } else { 5 };

        self.send_request(
            "/dapi/v1/ticker/bookTicker",
            reqwest::Method::GET,
            Some(params),
            weight,
        )
        .await
    }

    /// Get symbol order book ticker by pair
    ///
    /// Weight: 2 for a single pair; 5 when the pair parameter is omitted
    async fn get_book_ticker_by_pair(
        &self,
        params: BookTickerRequestByPair,
    ) -> RestResult<Vec<BookTicker>> {
        let weight = if params.pair.is_some() { 2 } else { 5 };

        self.send_request(
            "/dapi/v1/ticker/bookTicker",
            reqwest::Method::GET,
            Some(params),
            weight,
        )
        .await
    }

    /// Get symbol order book ticker
    ///
    /// Weight: 2 for a single symbol; 5 when the symbol parameter is omitted
    pub async fn get_book_ticker(&self, params: BookTickerRequest) -> RestResult<Vec<BookTicker>> {
        match params {
            BookTickerRequest::BySymbol(by_symbol) => {
                self.get_book_ticker_by_symbol(by_symbol).await
            }
            BookTickerRequest::ByPair(by_pair) => self.get_book_ticker_by_pair(by_pair).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::FromPrimitive;

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
