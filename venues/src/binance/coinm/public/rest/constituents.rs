use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

/// Index Price Constituents endpoint path
const CONSTITUENTS_ENDPOINT: &str = "/dapi/v1/constituents";

/// Parameters for Query Index Price Constituents
#[derive(Debug, Clone, Serialize, Default)]
pub struct ConstituentsRequest {
    /// Symbol name
    #[serde(rename = "symbol")]
    pub symbol: String,
}

/// Index price constituent
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Constituent {
    /// Exchange name
    #[serde(rename = "exchange")]
    pub exchange: String,

    /// Symbol name
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Weight
    #[serde(rename = "weight")]
    pub weight: Decimal,
}

/// Index price constituents response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Constituents {
    /// Symbol name
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Timestamp
    #[serde(rename = "time")]
    pub time: i64,

    /// Constituents
    #[serde(rename = "constituents")]
    pub constituents: Vec<Constituent>,
}

impl RestClient {
    /// Query index price constituents
    ///
    /// Retrieves the list of constituents that make up an index price for a given symbol.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Index-Constituents
    ///
    /// Rate limit: 2
    ///
    /// # Arguments
    /// * `params` - The `ConstituentsRequest` specifying the symbol to query.
    ///
    /// # Returns
    /// A `Constituents` object containing the symbol, timestamp, and list of constituents.
    pub async fn get_constituents(&self, params: ConstituentsRequest) -> RestResult<Constituents> {
        self.send_request(CONSTITUENTS_ENDPOINT, reqwest::Method::GET, Some(params), 2)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::FromPrimitive;

    #[test]
    fn test_constituents_request_serialization() {
        let request = ConstituentsRequest {
            symbol: "BTCUSD".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSD");
    }

    #[test]
    fn test_constituent_deserialization() {
        let json = r#"{
            "exchange": "Binance",
            "symbol": "BTCUSDT",
            "price": 50000.50,
            "weight": 0.25
        }"#;

        let constituent: Constituent = serde_json::from_str(json).unwrap();
        assert_eq!(constituent.exchange, "Binance");
        assert_eq!(constituent.symbol, "BTCUSDT");
        assert_eq!(constituent.price, Decimal::from_f64(50000.50).unwrap());
        assert_eq!(constituent.weight, Decimal::from_f64(0.25).unwrap());
    }

    #[test]
    fn test_constituents_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSD",
            "time": 1625097600000,
            "constituents": [
                {
                    "exchange": "Binance",
                    "symbol": "BTCUSDT",
                    "price": 50000.50,
                    "weight": 0.25
                },
                {
                    "exchange": "Coinbase",
                    "symbol": "BTC-USD",
                    "price": 50100.00,
                    "weight": 0.30
                },
                {
                    "exchange": "Kraken",
                    "symbol": "XBTUSD",
                    "price": 50050.25,
                    "weight": 0.20
                },
                {
                    "exchange": "Bitstamp",
                    "symbol": "BTCUSD",
                    "price": 49950.00,
                    "weight": 0.25
                }
            ]
        }"#;

        let constituents: Constituents = serde_json::from_str(json).unwrap();
        assert_eq!(constituents.symbol, "BTCUSD");
        assert_eq!(constituents.time, 1625097600000);
        assert_eq!(constituents.constituents.len(), 4);

        assert_eq!(constituents.constituents[0].exchange, "Binance");
        assert_eq!(
            constituents.constituents[0].price,
            Decimal::from_f64(50000.50).unwrap()
        );
        assert_eq!(
            constituents.constituents[0].weight,
            Decimal::from_f64(0.25).unwrap()
        );

        assert_eq!(constituents.constituents[1].exchange, "Coinbase");
        assert_eq!(constituents.constituents[1].symbol, "BTC-USD");
        assert_eq!(
            constituents.constituents[1].weight,
            Decimal::from_f64(0.30).unwrap()
        );

        // Verify weights sum to 1.0
        let total_weight: Decimal = constituents.constituents.iter().map(|c| c.weight).sum();
        assert_eq!(total_weight, Decimal::from_f64(1.0).unwrap());
    }
}
