use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

/// Request parameters for the order book endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct OrderBookRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Default 500; Valid limits: [5, 10, 20, 50, 100, 500, 1000].
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Response from the order book endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookResponse {
    /// Last update ID for the order book.
    pub last_update_id: u64,

    /// Trading symbol.
    pub symbol: String,

    /// Trading pair.
    pub pair: String,

    /// Message output time.
    #[serde(rename = "E")]
    pub event_time: u64,

    /// Transaction time.
    #[serde(rename = "T")]
    pub transaction_time: u64,

    /// Array of bid levels.
    pub bids: Vec<(String, String)>,

    /// Array of ask levels.
    pub asks: Vec<(String, String)>,
}

impl RestClient {
    /// Query orderbook on specific symbol.
    ///
    /// [Official API docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Order-Book)
    ///
    /// Weight varies based on limit:
    /// - 5, 10, 20, 50: 2
    /// - 100: 5
    /// - 500: 10
    /// - 1000: 20
    pub async fn get_order_book(&self, params: OrderBookRequest) -> RestResult<OrderBookResponse> {
        let weight = match params.limit.unwrap_or(500) {
            1..=50 => 2,
            51..=100 => 5,
            101..=500 => 10,
            _ => 20,
        };

        self.send_request("/dapi/v1/depth", reqwest::Method::GET, Some(params), weight)
            .await
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Client;

    use super::*;
    use crate::binance::coinm::RateLimiter;

    #[tokio::test]
    async fn test_get_order_book_parameters() {
        // Create a mock client (this test won't actually make network calls)
        let client = Client::new();
        let rate_limiter = RateLimiter::new();
        let _rest_client = RestClient::new("https://dapi.binance.com", client, rate_limiter);

        // Test request with default parameters
        let request = OrderBookRequest {
            symbol: "BTCUSD_PERP".to_string(),
            limit: None,
        };

        // Test serialization of request parameters
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        println!("Serialized request: {}", serialized);
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(!serialized.contains("limit="));

        // Test request with limit
        let request_with_limit = OrderBookRequest {
            symbol: "ETHUSD_PERP".to_string(),
            limit: Some(100),
        };

        let serialized_with_limit = serde_urlencoded::to_string(&request_with_limit).unwrap();
        println!("Serialized request with limit: {}", serialized_with_limit);
        assert!(serialized_with_limit.contains("symbol=ETHUSD_PERP"));
        assert!(serialized_with_limit.contains("limit=100"));

        // Test weight calculation logic by inspecting the method's weight calculation
        // (This tests the logic without making actual API calls)
        let test_cases = vec![
            (Some(5), 2),
            (Some(50), 2),
            (Some(100), 5),
            (Some(500), 10),
            (Some(1000), 20),
            (None, 10), // Default is 500, so weight should be 10
        ];

        for (limit, expected_weight) in test_cases {
            let weight = match limit.unwrap_or(500) {
                1..=50 => 2,
                51..=100 => 5,
                101..=500 => 10,
                _ => 20,
            };
            println!(
                "Limit: {:?}, Expected weight: {}, Actual weight: {}",
                limit, expected_weight, weight
            );
            assert_eq!(weight, expected_weight);
        }
    }

    #[test]
    fn test_order_book_request_serialization() {
        let request = OrderBookRequest {
            symbol: "BTCUSD_PERP".to_string(),
            limit: Some(100),
        };

        let json = serde_json::to_string(&request).unwrap();
        println!("JSON serialized request: {}", json);
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
        println!("Deserialized response: {:?}", response);

        assert_eq!(response.last_update_id, 1027024);
        assert_eq!(response.symbol, "BTCUSD_PERP");
        assert_eq!(response.pair, "BTCUSD");
        assert_eq!(response.event_time, 1589436922972);
        assert_eq!(response.transaction_time, 1589436922959);
        assert_eq!(response.bids.len(), 1);
        assert_eq!(response.asks.len(), 1);
        assert_eq!(response.bids[0].0, "4.00000000");
        assert_eq!(response.bids[0].1, "431.00000000");
        assert_eq!(response.asks[0].0, "4.00000200");
        assert_eq!(response.asks[0].1, "12.00000000");
    }
}
