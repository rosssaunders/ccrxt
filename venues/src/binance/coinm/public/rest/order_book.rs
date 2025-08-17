use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

/// Endpoint path for the order book API.
const ORDER_BOOK_ENDPOINT: &str = "/dapi/v1/depth";

/// Request parameters for the order book endpoint.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP"). Required.
    /// Must be a valid symbol listed on Binance Coin-M Futures.
    pub symbol: Cow<'static, str>,

    /// Limit for number of levels returned. Optional.
    /// Default: 500. Valid values: 5, 10, 20, 50, 100, 500, 1000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Represents a single price level in the order book.
#[derive(Debug, Clone, Deserialize)]
pub struct OrderBookLevel {
    /// Price for this level.
    pub price: Cow<'static, str>,

    /// Quantity at this price level.
    pub qty: Cow<'static, str>,
}

/// Response from the order book endpoint.
///
/// Contains the current order book snapshot for a symbol.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookResponse {
    /// Last update ID for the order book.
    pub last_update_id: u64,

    /// Trading symbol.
    pub symbol: Cow<'static, str>,

    /// Trading pair.
    pub pair: Cow<'static, str>,

    /// Message output time (milliseconds since epoch).
    #[serde(rename = "E")]
    pub event_time: u64,

    /// Transaction time (milliseconds since epoch).
    #[serde(rename = "T")]
    pub transaction_time: u64,

    /// Array of bid levels. Each entry is [price, qty].
    #[serde(deserialize_with = "deserialize_levels")]
    pub bids: Vec<OrderBookLevel>,

    /// Array of ask levels. Each entry is [price, qty].
    #[serde(deserialize_with = "deserialize_levels")]
    pub asks: Vec<OrderBookLevel>,
}

/// Custom deserializer for order book levels.
fn deserialize_levels<'de, D>(deserializer: D) -> Result<Vec<OrderBookLevel>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use std::fmt;

    use serde::de::{SeqAccess, Visitor};

    struct LevelsVisitor;
    impl<'de> Visitor<'de> for LevelsVisitor {
        type Value = Vec<OrderBookLevel>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a list of [price, qty] arrays")
        }
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut levels = Vec::new();
            while let Some(pair) = seq.next_element::<(String, String)>()? {
                levels.push(OrderBookLevel {
                    price: Cow::Owned(pair.0),
                    qty: Cow::Owned(pair.1),
                });
            }
            Ok(levels)
        }
    }
    deserializer.deserialize_seq(LevelsVisitor)
}

impl RestClient {
    /// Order Book
    ///
    /// Query orderbook on specific symbol.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Order-Book)
    ///
    /// Weight varies based on limit:
    /// - 5, 10, 20, 50: 2
    /// - 100: 5
    /// - 500: 10
    /// - 1000: 20
    ///
    /// # Arguments
    /// * `params` - The request parameters for the order book endpoint.
    ///
    /// # Returns
    /// Returns [`OrderBookResponse`] containing the current order book snapshot for the symbol.
    pub async fn get_order_book(&self, params: OrderBookRequest) -> RestResult<OrderBookResponse> {
        let weight = match params.limit.unwrap_or(500) {
            5 | 10 | 20 | 50 => 2,
            100 => 5,
            500 => 10,
            1000 => 20,
            _ => 10, // Default weight for unspecified limit
        };
        self.send_get_request(ORDER_BOOK_ENDPOINT, Some(params), weight)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binance::shared::venue_trait::VenueConfig;

    #[tokio::test]
    async fn test_get_order_book_parameters() {
        let http_client = std::sync::Arc::new(rest::native::NativeHttpClient::default());
        let config = crate::binance::coinm::CoinmConfig;
        let rate_limiter = crate::binance::shared::RateLimiter::new(config.rate_limits());
        let _rest_client = RestClient::new("https://dapi.binance.com", http_client, rate_limiter);

        let request = OrderBookRequest {
            symbol: Cow::Borrowed("BTCUSD_PERP"),
            limit: None,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(!serialized.contains("limit="));

        let request_with_limit = OrderBookRequest {
            symbol: Cow::Borrowed("ETHUSD_PERP"),
            limit: Some(100),
        };
        let serialized_with_limit = serde_urlencoded::to_string(&request_with_limit).unwrap();
        assert!(serialized_with_limit.contains("symbol=ETHUSD_PERP"));
        assert!(serialized_with_limit.contains("limit=100"));

        let test_cases = vec![
            (Some(5), 2),
            (Some(50), 2),
            (Some(100), 5),
            (Some(500), 10),
            (Some(1000), 20),
            (None, 10),
        ];
        for (limit, expected_weight) in test_cases {
            let weight = match limit {
                Some(5) | Some(10) | Some(20) | Some(50) => 2,
                Some(100) => 5,
                Some(500) => 10,
                Some(1000) => 20,
                None => 10,
                _ => 10,
            };
            assert_eq!(weight, expected_weight);
        }
    }

    #[test]
    fn test_order_book_request_serialization() {
        let request = OrderBookRequest {
            symbol: Cow::Borrowed("BTCUSD_PERP"),
            limit: Some(100),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"symbol\":\"BTCUSD_PERP\""));
        assert!(json.contains("\"limit\":100"));
    }

    #[test]
    fn test_order_book_response_deserialization() {
        let sample_response = r#"{
            "lastUpdateId": 1027024,
            "symbol": "BTCUSD_PERP",
            "pair": "BTCUSD",
            "E": 1589436922972,
            "T": 1589436922959,
            "bids": [
                ["4.00000000", "431.00000000"]
            ],
            "asks": [
                ["4.00000200", "12.00000000"]
            ]
        }"#;
        let response: OrderBookResponse = serde_json::from_str(sample_response).unwrap();
        assert_eq!(response.last_update_id, 1027024);
        assert_eq!(response.symbol, "BTCUSD_PERP");
        assert_eq!(response.pair, "BTCUSD");
        assert_eq!(response.event_time, 1589436922972);
        assert_eq!(response.transaction_time, 1589436922959);
        assert_eq!(response.bids.len(), 1);
        assert_eq!(response.asks.len(), 1);
        assert_eq!(response.bids[0].price, "4.00000000");
        assert_eq!(response.bids[0].qty, "431.00000000");
        assert_eq!(response.asks[0].price, "4.00000200");
        assert_eq!(response.asks[0].qty, "12.00000000");
    }
}
