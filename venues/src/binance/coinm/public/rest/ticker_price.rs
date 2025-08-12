use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

/// Endpoint for Symbol Price Ticker
const SYMBOL_PRICE_TICKER_ENDPOINT: &str = "/dapi/v1/ticker/price";

/// Request parameters for the Symbol Price Ticker endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct TickerPriceRequest {
    /// Trading symbol (e.g., "BTCUSD_200626"). Optional.
    ///
    /// If provided, returns the ticker for the specified symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Trading pair (e.g., "BTCUSD"). Optional.
    ///
    /// If provided, returns tickers for all symbols of the specified pair.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
}

/// Represents a symbol price ticker returned by the API.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerPrice {
    /// Trading symbol (e.g., "BTCUSD_200626").
    pub symbol: String,

    /// Trading pair (e.g., "BTCUSD").
    pub ps: String,

    /// Latest price for the symbol.
    pub price: Decimal,

    /// Timestamp of the price (milliseconds since epoch).
    pub time: i64,
}

impl RestClient {
    /// Symbol Price Ticker
    ///
    /// Returns the latest price for a symbol or symbols.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Symbol-Price-Ticker)
    ///
    /// Rate limit: 1 for a single symbol, 2 when the symbol parameter is omitted
    ///
    /// # Arguments
    /// * `params` - The request parameters for the symbol price ticker endpoint
    ///
    /// # Returns
    /// A vector of `TickerPrice` structs containing the latest price(s) for the requested symbol(s).
    pub async fn get_ticker_price(
        &self,
        params: TickerPriceRequest,
    ) -> RestResult<Vec<TickerPrice>> {
        let weight = if params.symbol.is_some() { 1 } else { 2 };

        // The API always returns an array, even for single symbols
        self.send_get_request(SYMBOL_PRICE_TICKER_ENDPOINT, Some(params), weight)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;
    use serde_json;

    use super::*;

    #[test]
    fn test_ticker_price_request_serialization_symbol() {
        let req = TickerPriceRequest {
            symbol: Some("BTCUSD_200626".to_string()),
            pair: None,
        };
        let serialized = serde_urlencoded::to_string(&req).unwrap();
        assert_eq!(serialized, "symbol=BTCUSD_200626");
    }

    #[test]
    fn test_ticker_price_request_serialization_pair() {
        let req = TickerPriceRequest {
            symbol: None,
            pair: Some("BTCUSD".to_string()),
        };
        let serialized = serde_urlencoded::to_string(&req).unwrap();
        assert_eq!(serialized, "pair=BTCUSD");
    }

    #[test]
    fn test_ticker_price_request_serialization_empty() {
        let req = TickerPriceRequest::default();
        let serialized = serde_urlencoded::to_string(&req).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_ticker_price_response_deserialization() {
        let json =
            r#"[{"symbol":"BTCUSD_200626","ps":"BTCUSD","price":"9647.8","time":1591257246176}]"#;
        let resp: Vec<TickerPrice> = serde_json::from_str(json).unwrap();
        assert_eq!(resp.len(), 1);
        let ticker = &resp[0];
        assert_eq!(ticker.symbol, "BTCUSD_200626");
        assert_eq!(ticker.ps, "BTCUSD");
        assert_eq!(ticker.price, dec!(9647.8));
        assert_eq!(ticker.time, 1591257246176);
    }
}
