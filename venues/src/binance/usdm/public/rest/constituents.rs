use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::binance::usdm::PublicRestClient as RestClient;
use crate::binance::usdm::RestResult;

/// Endpoint for querying index price constituents.
const INDEX_CONSTITUENTS_ENDPOINT: &str = "/fapi/v1/constituents";

/// Request parameters for index constituents.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstituentsRequest {
    /// Trading pair symbol (e.g., "BTCUSDT").
    pub symbol: Cow<'static, str>,
}

/// Represents a single constituent in the index.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Constituent {
    /// Exchange name.
    pub exchange: Cow<'static, str>,

    /// Trading pair symbol for the exchange.
    pub symbol: Cow<'static, str>,

    /// Last price for the symbol.
    pub price: Cow<'static, str>,

    /// Weight of the symbol in the index.
    pub weight: Cow<'static, str>,
}

/// Represents the index constituents response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstituentsResponse {
    /// Trading pair symbol.
    pub symbol: Cow<'static, str>,

    /// Response timestamp (milliseconds since epoch).
    pub time: u64,

    /// List of constituents in the index.
    pub constituents: Vec<Constituent>,
}

impl RestClient {
    /// Index Constituents
    ///
    /// GET /fapi/v1/constituents
    ///
    /// Query index price constituents.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Index-Constituents)
    ///
    /// Rate limit: 2 weight
    ///
    /// # Arguments
    ///
    /// * `params` - Request parameters for index constituents.
    ///
    /// # Returns
    ///
    /// [`RestResult<ConstituentsResponse>`] containing the index constituents response.
    pub async fn get_constituents(
        &self,
        params: ConstituentsRequest,
    ) -> RestResult<ConstituentsResponse> {
        self.send_get_request(INDEX_CONSTITUENTS_ENDPOINT, Some(params), 2)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constituents_request_serialization() {
        let request = ConstituentsRequest {
            symbol: "BTCUSDT".into(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSDT");
    }

    #[test]
    fn test_constituent_deserialization() {
        let json = r#"{
            "exchange": "Binance",
            "symbol": "BTCUSDT",
            "price": "45380.10",
            "weight": "0.25"
        }"#;

        let constituent: Constituent = serde_json::from_str(json).unwrap();
        assert_eq!(constituent.exchange, "Binance");
        assert_eq!(constituent.symbol, "BTCUSDT");
        assert_eq!(constituent.price, "45380.10");
        assert_eq!(constituent.weight, "0.25");
    }

    #[test]
    fn test_constituents_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "time": 1625184000000,
            "constituents": [
                {
                    "exchange": "Binance",
                    "symbol": "BTCUSDT",
                    "price": "45380.10",
                    "weight": "0.25"
                },
                {
                    "exchange": "Coinbase",
                    "symbol": "BTCUSD",
                    "price": "45385.50",
                    "weight": "0.25"
                },
                {
                    "exchange": "Kraken",
                    "symbol": "XBTUSD",
                    "price": "45390.00",
                    "weight": "0.25"
                },
                {
                    "exchange": "Bitstamp",
                    "symbol": "BTCUSD",
                    "price": "45387.20",
                    "weight": "0.25"
                }
            ]
        }"#;

        let response: ConstituentsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.time, 1625184000000);
        assert_eq!(response.constituents.len(), 4);

        assert_eq!(response.constituents[0].exchange, "Binance");
        assert_eq!(response.constituents[0].price, "45380.10");
        assert_eq!(response.constituents[0].weight, "0.25");

        assert_eq!(response.constituents[1].exchange, "Coinbase");
        assert_eq!(response.constituents[1].symbol, "BTCUSD");
    }

    #[test]
    fn test_constituents_response_single_constituent() {
        let json = r#"{
            "symbol": "ETHUSDT",
            "time": 1625184000000,
            "constituents": [
                {
                    "exchange": "Binance",
                    "symbol": "ETHUSDT",
                    "price": "3070.50",
                    "weight": "1.0"
                }
            ]
        }"#;

        let response: ConstituentsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.constituents.len(), 1);
        assert_eq!(response.constituents[0].weight, "1.0");
    }

    #[test]
    fn test_constituents_response_empty_constituents() {
        let json = r#"{
            "symbol": "UNKNOWN",
            "time": 1625184000000,
            "constituents": []
        }"#;

        let response: ConstituentsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "UNKNOWN");
        assert_eq!(response.constituents.len(), 0);
    }

    #[test]
    fn test_constituent_high_precision_values() {
        let json = r#"{
            "exchange": "Exchange",
            "symbol": "SYMBOL",
            "price": "0.00001234567890",
            "weight": "0.123456789"
        }"#;

        let constituent: Constituent = serde_json::from_str(json).unwrap();
        assert_eq!(constituent.price, "0.00001234567890");
        assert_eq!(constituent.weight, "0.123456789");
    }

    #[test]
    fn test_constituents_different_weights() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "time": 1625184000000,
            "constituents": [
                {
                    "exchange": "Exchange1",
                    "symbol": "BTC1",
                    "price": "45000.00",
                    "weight": "0.40"
                },
                {
                    "exchange": "Exchange2",
                    "symbol": "BTC2",
                    "price": "45100.00",
                    "weight": "0.35"
                },
                {
                    "exchange": "Exchange3",
                    "symbol": "BTC3",
                    "price": "45200.00",
                    "weight": "0.25"
                }
            ]
        }"#;

        let response: ConstituentsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.constituents.len(), 3);

        // Verify weights sum to 1.0
        let weight_sum: f64 = response
            .constituents
            .iter()
            .map(|c| c.weight.parse::<f64>().unwrap())
            .sum();
        assert!((weight_sum - 1.0).abs() < 0.0001);
    }
}
