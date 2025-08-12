use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::RestResult;

const TICKER_PRICE_ENDPOINT: &str = "/fapi/v1/ticker/price";

/// Request parameters for the Symbol Price Ticker endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct TickerPriceRequest {
    /// Trading symbol (e.g., "BTCUSDT"). If not sent, prices for all symbols will be returned in an array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

/// Represents a single symbol price ticker response.
#[derive(Debug, Clone, Deserialize)]
pub struct TickerPrice {
    /// Trading symbol.
    pub symbol: String,

    /// Current price of the symbol as a string.
    pub price: String,

    /// Transaction time (milliseconds since epoch).
    pub time: u64,
}

/// Response from the Symbol Price Ticker endpoint.
/// Can be either a single ticker or multiple tickers depending on whether symbol parameter is provided.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum TickerPriceResult {
    /// Single ticker price response when symbol is specified.
    Single(TickerPrice),

    /// Multiple ticker prices when symbol parameter is omitted.
    Multiple(Vec<TickerPrice>),
}

impl RestClient {
    /// Symbol Price Ticker
    ///
    /// Latest price for a symbol or symbols.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Price-Ticker)
    ///
    /// Rate limit: 1 for a single symbol; 2 when the symbol parameter is omitted
    ///
    /// # Arguments
    /// * `params` - Request parameters including optional symbol filter
    ///
    /// # Returns
    /// Either a single ticker price or multiple ticker prices depending on the request
    pub async fn get_ticker_price(
        &self,
        params: TickerPriceRequest,
    ) -> RestResult<TickerPriceResult> {
        self.send_get_request(TICKER_PRICE_ENDPOINT, Some(params), 1)
            .await
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticker_price_request_no_symbol() {
        let request = TickerPriceRequest { symbol: None };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_ticker_price_request_with_symbol() {
        let request = TickerPriceRequest {
            symbol: Some("BTCUSDT".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSDT");
    }

    #[test]
    fn test_ticker_price_single_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "price": "45384.10000000",
            "time": 1625184000000
        }"#;

        let ticker: TickerPrice = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "BTCUSDT");
        assert_eq!(ticker.price, "45384.10000000");
        assert_eq!(ticker.time, 1625184000000);
    }

    #[test]
    fn test_ticker_price_result_single_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "price": "45384.10000000",
            "time": 1625184000000
        }"#;

        let result: TickerPriceResult = serde_json::from_str(json).unwrap();
        match result {
            TickerPriceResult::Single(ticker) => {
                assert_eq!(ticker.symbol, "BTCUSDT");
                assert_eq!(ticker.price, "45384.10000000");
                assert_eq!(ticker.time, 1625184000000);
            }
            TickerPriceResult::Multiple(_) => panic!("Expected Single variant"),
        }
    }

    #[test]
    fn test_ticker_price_result_multiple_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "price": "45384.10000000",
                "time": 1625184000000
            },
            {
                "symbol": "ETHUSDT",
                "price": "3072.84000000",
                "time": 1625184000000
            },
            {
                "symbol": "BNBUSDT",
                "price": "350.20000000",
                "time": 1625184000001
            }
        ]"#;

        let result: TickerPriceResult = serde_json::from_str(json).unwrap();
        match result {
            TickerPriceResult::Multiple(tickers) => {
                assert_eq!(tickers.len(), 3);
                assert_eq!(tickers[0].symbol, "BTCUSDT");
                assert_eq!(tickers[0].price, "45384.10000000");
                assert_eq!(tickers[0].time, 1625184000000);
                assert_eq!(tickers[1].symbol, "ETHUSDT");
                assert_eq!(tickers[1].price, "3072.84000000");
                assert_eq!(tickers[1].time, 1625184000000);
                assert_eq!(tickers[2].symbol, "BNBUSDT");
                assert_eq!(tickers[2].price, "350.20000000");
                assert_eq!(tickers[2].time, 1625184000001);
            }
            TickerPriceResult::Single(_) => panic!("Expected Multiple variant"),
        }
    }

    #[test]
    fn test_ticker_price_small_values() {
        let json = r#"{
            "symbol": "SHIBUSDT",
            "price": "0.00001234",
            "time": 1625184000000
        }"#;

        let ticker: TickerPrice = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "SHIBUSDT");
        assert_eq!(ticker.price, "0.00001234");
        assert_eq!(ticker.time, 1625184000000);
    }

    #[test]
    fn test_ticker_price_large_values() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "price": "99999.99999999",
            "time": 1625184000000
        }"#;

        let ticker: TickerPrice = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "BTCUSDT");
        assert_eq!(ticker.price, "99999.99999999");
        assert_eq!(ticker.time, 1625184000000);
    }

    #[test]
    fn test_ticker_price_with_official_example() {
        // Test with official API documentation example
        let json = r#"{
            "symbol": "BTCUSDT",
            "price": "6000.01",
            "time": 1589437530011
        }"#;

        let ticker: TickerPrice = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "BTCUSDT");
        assert_eq!(ticker.price, "6000.01");
        assert_eq!(ticker.time, 1589437530011);
    }

    #[test]
    fn test_ticker_price_result_with_official_example_array() {
        // Test with official API documentation example for array response
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "price": "6000.01",
                "time": 1589437530011
            }
        ]"#;

        let result: TickerPriceResult = serde_json::from_str(json).unwrap();
        match result {
            TickerPriceResult::Multiple(tickers) => {
                assert_eq!(tickers.len(), 1);
                assert_eq!(tickers[0].symbol, "BTCUSDT");
                assert_eq!(tickers[0].price, "6000.01");
                assert_eq!(tickers[0].time, 1589437530011);
            }
            TickerPriceResult::Single(_) => panic!("Expected Multiple variant"),
        }
    }

    #[test]
    fn test_ticker_price_empty_array() {
        let json = r#"[]"#;
        let result: TickerPriceResult = serde_json::from_str(json).unwrap();
        match result {
            TickerPriceResult::Multiple(tickers) => {
                assert_eq!(tickers.len(), 0);
            }
            TickerPriceResult::Single(_) => panic!("Expected Multiple variant"),
        }
    }

    #[test]
    fn test_ticker_price_default_request() {
        let request = TickerPriceRequest::default();
        assert!(request.symbol.is_none());

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }
}
