use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::RestResult;

const TICKER_PRICE_ENDPOINT: &str = "/api/v3/ticker/price";

/// Request parameters for symbol price ticker endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct TickerPriceRequest {
    /// Single symbol (e.g., "BTCUSDT"). Cannot be used with symbols parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Multiple symbols in JSON array format (e.g., ["BTCUSDT","BNBUSDT"]). Cannot be used with symbol parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
}

/// Symbol price ticker response.
#[derive(Debug, Clone, Deserialize)]
pub struct TickerPrice {
    /// Trading symbol.
    pub symbol: String,

    /// Latest price for the symbol.
    pub price: Decimal,
}

/// Response wrapper for ticker price endpoint that can return either single or multiple tickers.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum TickerPriceResponse {
    /// Single ticker response when symbol parameter is used.
    Single(TickerPrice),
    /// Multiple tickers response when symbols parameter is used or no parameters.
    Multiple(Vec<TickerPrice>),
}

impl RestClient {
    /// Symbol price ticker
    ///
    /// Latest price for a symbol or symbols.
    ///
    /// [docs]: https://developers.binance.com/docs/binance-spot-api-docs/testnet/rest-api/market-data-endpoints#symbol-price-ticker
    ///
    /// Rate limit: 2 for single symbol, 4 for multiple symbols or all symbols
    ///
    /// # Arguments
    /// * `request` - The ticker price request parameters
    ///
    /// # Returns
    /// Either a single ticker or multiple tickers depending on request parameters
    pub async fn ticker_price(
        &self,
        params: TickerPriceRequest,
    ) -> RestResult<TickerPriceResponse> {
        // Calculate weight: 2 for single symbol, 4 for multiple symbols or all symbols
        let weight = if params.symbol.is_some() {
            2
        } else {
            4
        };

        self.send_public_request(TICKER_PRICE_ENDPOINT, reqwest::Method::GET, Some(params), weight)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticker_price_request_serialization_empty() {
        let request = TickerPriceRequest::default();
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_ticker_price_request_serialization_single_symbol() {
        let request = TickerPriceRequest {
            symbol: Some("BTCUSDT".to_string()),
            symbols: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSDT");
    }

    #[test]
    fn test_ticker_price_request_serialization_multiple_symbols() {
        let request = TickerPriceRequest {
            symbol: None,
            symbols: Some("[\"BTCUSDT\",\"ETHUSDT\"]".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbols=%5B%22BTCUSDT%22%2C%22ETHUSDT%22%5D"));
    }

    #[test]
    fn test_ticker_price_deserialization_single() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "price": "45380.10000000"
        }"#;

        let response: TickerPriceResponse = serde_json::from_str(json).unwrap();
        match response {
            TickerPriceResponse::Single(ticker) => {
                assert_eq!(ticker.symbol, "BTCUSDT");
                assert_eq!(ticker.price.to_string(), "45380.10000000");
            }
            TickerPriceResponse::Multiple(_) => panic!("Expected single ticker response"),
        }
    }

    #[test]
    fn test_ticker_price_deserialization_multiple() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "price": "45380.10000000"
            },
            {
                "symbol": "ETHUSDT",
                "price": "3070.50000000"
            },
            {
                "symbol": "ADAUSDT",
                "price": "0.45670000"
            }
        ]"#;

        let response: TickerPriceResponse = serde_json::from_str(json).unwrap();
        match response {
            TickerPriceResponse::Single(_) => panic!("Expected multiple ticker response"),
            TickerPriceResponse::Multiple(tickers) => {
                assert_eq!(tickers.len(), 3);

                assert_eq!(tickers[0].symbol, "BTCUSDT");
                assert_eq!(tickers[0].price.to_string(), "45380.10000000");

                assert_eq!(tickers[1].symbol, "ETHUSDT");
                assert_eq!(tickers[1].price.to_string(), "3070.50000000");

                assert_eq!(tickers[2].symbol, "ADAUSDT");
                assert_eq!(tickers[2].price.to_string(), "0.45670000");
            }
        }
    }

    #[test]
    fn test_ticker_price_high_precision() {
        let json = r#"{
            "symbol": "DOGEUSDT",
            "price": "0.12345678"
        }"#;

        let response: TickerPriceResponse = serde_json::from_str(json).unwrap();
        match response {
            TickerPriceResponse::Single(ticker) => {
                assert_eq!(ticker.symbol, "DOGEUSDT");
                assert_eq!(ticker.price.to_string(), "0.12345678");
            }
            TickerPriceResponse::Multiple(_) => panic!("Expected single ticker response"),
        }
    }

    #[test]
    fn test_ticker_price_very_small_value() {
        let json = r#"{
            "symbol": "SHIBUSDT",
            "price": "0.00001234"
        }"#;

        let response: TickerPriceResponse = serde_json::from_str(json).unwrap();
        match response {
            TickerPriceResponse::Single(ticker) => {
                assert_eq!(ticker.symbol, "SHIBUSDT");
                assert_eq!(ticker.price.to_string(), "0.00001234");
            }
            TickerPriceResponse::Multiple(_) => panic!("Expected single ticker response"),
        }
    }

    #[test]
    fn test_ticker_price_large_value() {
        let json = r#"{
            "symbol": "BTCEUR",
            "price": "42000.00000000"
        }"#;

        let response: TickerPriceResponse = serde_json::from_str(json).unwrap();
        match response {
            TickerPriceResponse::Single(ticker) => {
                assert_eq!(ticker.symbol, "BTCEUR");
                assert_eq!(ticker.price.to_string(), "42000.00000000");
            }
            TickerPriceResponse::Multiple(_) => panic!("Expected single ticker response"),
        }
    }

    #[test]
    fn test_ticker_price_stable_coin() {
        let json = r#"{
            "symbol": "USDCUSDT",
            "price": "1.00010000"
        }"#;

        let response: TickerPriceResponse = serde_json::from_str(json).unwrap();
        match response {
            TickerPriceResponse::Single(ticker) => {
                assert_eq!(ticker.symbol, "USDCUSDT");
                assert_eq!(ticker.price.to_string(), "1.00010000");
            }
            TickerPriceResponse::Multiple(_) => panic!("Expected single ticker response"),
        }
    }

    #[test]
    fn test_ticker_price_empty_array() {
        let json = r#"[]"#;
        let response: TickerPriceResponse = serde_json::from_str(json).unwrap();
        match response {
            TickerPriceResponse::Single(_) => panic!("Expected multiple ticker response"),
            TickerPriceResponse::Multiple(tickers) => {
                assert_eq!(tickers.len(), 0);
            }
        }
    }

    #[test]
    fn test_ticker_price_request_multiple_symbols_large() {
        let request = TickerPriceRequest {
            symbol: None,
            symbols: Some(
                "[\"BTCUSDT\",\"ETHUSDT\",\"ADAUSDT\",\"BNBUSDT\",\"XRPUSDT\"]".to_string(),
            ),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbols="));
        assert!(serialized.contains("BTCUSDT"));
        assert!(serialized.contains("ETHUSDT"));
        assert!(serialized.contains("ADAUSDT"));
        assert!(serialized.contains("BNBUSDT"));
        assert!(serialized.contains("XRPUSDT"));
    }
}
