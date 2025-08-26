use serde::{Deserialize, Serialize};

use crate::binance::usdm::PublicRestClient as RestClient;
use crate::binance::usdm::RestResult;

const TICKER_PRICE_V2_ENDPOINT: &str = "/fapi/v2/ticker/price";

/// Request parameters for the Symbol Price Ticker V2 endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct TickerPriceV2Request {
    /// Trading symbol (e.g., "BTCUSDT"). If not sent, prices for all symbols will be returned in an array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

/// Represents a single ticker price response.
#[derive(Debug, Clone, Deserialize)]
pub struct TickerPriceV2Response {
    /// Trading symbol.
    pub symbol: String,

    /// Current price of the symbol as a string.
    pub price: String,

    /// Transaction time (milliseconds since epoch).
    pub time: u64,
}

/// Response from the Symbol Price Ticker V2 endpoint.
/// Can be either a single ticker or multiple tickers depending on whether symbol parameter is provided.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum TickerPriceV2Result {
    /// Single ticker price response when symbol is specified.
    Single(TickerPriceV2Response),

    /// Multiple ticker prices when symbol parameter is omitted.
    Multiple(Vec<TickerPriceV2Response>),
}

impl RestClient {
    /// Symbol Price Ticker V2
    ///
    /// Latest price for a symbol or symbols.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Price-Ticker-v2)
    ///
    /// Rate limit: 1 for a single symbol; 2 when the symbol parameter is omitted
    ///
    /// # Arguments
    /// * `params` - Request parameters including optional symbol filter
    ///
    /// # Returns
    /// Either a single ticker price or multiple ticker prices depending on the request
    pub async fn ticker_price_v2(
        &self,
        params: TickerPriceV2Request,
    ) -> RestResult<TickerPriceV2Result> {
        self.send_get_request(TICKER_PRICE_V2_ENDPOINT, Some(params), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticker_price_v2_request_serialization_with_symbol() {
        let request = TickerPriceV2Request {
            symbol: Some("BTCUSDT".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSDT");
    }

    #[test]
    fn test_ticker_price_v2_request_serialization_without_symbol() {
        let request = TickerPriceV2Request { symbol: None };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_ticker_price_v2_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "price": "45380.10",
            "time": 1625184000000
        }"#;

        let response: TickerPriceV2Response = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.price, "45380.10");
        assert_eq!(response.time, 1625184000000);
    }

    #[test]
    fn test_ticker_price_v2_result_single_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "price": "45380.10",
            "time": 1625184000000
        }"#;

        let result: TickerPriceV2Result = serde_json::from_str(json).unwrap();
        match result {
            TickerPriceV2Result::Single(ticker) => {
                assert_eq!(ticker.symbol, "BTCUSDT");
                assert_eq!(ticker.price, "45380.10");
                assert_eq!(ticker.time, 1625184000000);
            }
            TickerPriceV2Result::Multiple(_) => panic!("Expected Single variant"),
        }
    }

    #[test]
    fn test_ticker_price_v2_result_multiple_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "price": "45380.10",
                "time": 1625184000000
            },
            {
                "symbol": "ETHUSDT",
                "price": "3070.50",
                "time": 1625184000001
            }
        ]"#;

        let result: TickerPriceV2Result = serde_json::from_str(json).unwrap();
        match result {
            TickerPriceV2Result::Multiple(tickers) => {
                assert_eq!(tickers.len(), 2);
                assert_eq!(tickers[0].symbol, "BTCUSDT");
                assert_eq!(tickers[0].price, "45380.10");
                assert_eq!(tickers[0].time, 1625184000000);
                assert_eq!(tickers[1].symbol, "ETHUSDT");
                assert_eq!(tickers[1].price, "3070.50");
                assert_eq!(tickers[1].time, 1625184000001);
            }
            TickerPriceV2Result::Single(_) => panic!("Expected Multiple variant"),
        }
    }

    #[test]
    fn test_ticker_price_v2_high_precision_price() {
        let json = r#"{
            "symbol": "SHIBUSDT",
            "price": "0.00001234",
            "time": 1625184000000
        }"#;

        let response: TickerPriceV2Response = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "SHIBUSDT");
        assert_eq!(response.price, "0.00001234");
    }

    #[test]
    fn test_ticker_price_v2_large_price() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "price": "999999.99",
            "time": 1625184000000
        }"#;

        let response: TickerPriceV2Response = serde_json::from_str(json).unwrap();
        assert_eq!(response.price, "999999.99");
    }

    #[test]
    fn test_ticker_price_v2_empty_array() {
        let json = r#"[]"#;
        let result: TickerPriceV2Result = serde_json::from_str(json).unwrap();
        match result {
            TickerPriceV2Result::Multiple(tickers) => {
                assert_eq!(tickers.len(), 0);
            }
            TickerPriceV2Result::Single(_) => panic!("Expected Multiple variant"),
        }
    }

    #[test]
    fn test_ticker_price_v2_default_request() {
        let request = TickerPriceV2Request::default();
        assert!(request.symbol.is_none());

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_ticker_price_v2_response_with_official_example() {
        // Test with official API documentation example
        let json = r#"{
            "symbol": "BTCUSDT",
            "price": "6000.01",
            "time": 1589437530011
        }"#;

        let response: TickerPriceV2Response = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.price, "6000.01");
        assert_eq!(response.time, 1589437530011);
    }

    #[test]
    fn test_ticker_price_v2_result_with_official_example_array() {
        // Test with official API documentation example for array response
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "price": "6000.01",
                "time": 1589437530011
            }
        ]"#;

        let result: TickerPriceV2Result = serde_json::from_str(json).unwrap();
        match result {
            TickerPriceV2Result::Multiple(tickers) => {
                assert_eq!(tickers.len(), 1);
                assert_eq!(tickers[0].symbol, "BTCUSDT");
                assert_eq!(tickers[0].price, "6000.01");
                assert_eq!(tickers[0].time, 1589437530011);
            }
            TickerPriceV2Result::Single(_) => panic!("Expected Multiple variant"),
        }
    }
}
