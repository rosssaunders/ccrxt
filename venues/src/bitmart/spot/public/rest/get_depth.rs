use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::{RestResult, rate_limit::EndpointType};

const DEPTH_ENDPOINT: &str = "/spot/quotation/v3/books";

/// Request parameters for getting depth
#[derive(Debug, Serialize)]
pub struct GetDepthRequest {
    /// Trading pair (e.g. BMX_USDT)
    pub symbol: String,
    /// Order book depth per side. Maximum 50, e.g. 50 bids + 50 asks.
    /// Default returns to 35 depth data, e.g. 35 bids + 35 asks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Order book entry [price, amount]
pub type OrderBookEntry = Vec<String>;

/// Depth/Order book data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepthData {
    /// Create time(Timestamp in milliseconds)
    pub ts: String,
    /// Trading pair
    pub symbol: String,
    /// Order book on sell side
    pub asks: Vec<OrderBookEntry>,
    /// Order book on buy side
    pub bids: Vec<OrderBookEntry>,
}

/// Response for depth endpoint
pub type GetDepthResponse = DepthData;

impl RestClient {
    /// Get Depth (V3)
    ///
    /// Get full depth of trading pairs.
    /// Note that the interface is not real-time data, if you need real-time data,
    /// please use websocket to subscribe Depth channel
    ///
    /// [docs](https://developer-pro.bitmart.com/en/spot/#get-depth-v3)
    ///
    /// Rate limit: 10 times/2sec per IP
    ///
    /// # Arguments
    /// * `request` - The request parameters including symbol and optional limit
    ///
    /// # Returns
    /// Full depth data for the specified trading pair
    pub async fn get_depth(&self, request: GetDepthRequest) -> RestResult<GetDepthResponse> {
        self.send_get_request(
            DEPTH_ENDPOINT,
            Some(&request),
            EndpointType::SpotPublicMarket,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_depth_request() {
        let request = GetDepthRequest {
            symbol: "BTC_USDT".to_string(),
            limit: Some(1),
        };

        assert_eq!(request.symbol, "BTC_USDT");
        assert_eq!(request.limit, Some(1));
    }

    #[test]
    fn test_get_depth_request_default_limit() {
        let request = GetDepthRequest {
            symbol: "BTC_USDT".to_string(),
            limit: None,
        };

        assert_eq!(request.symbol, "BTC_USDT");
        assert_eq!(request.limit, None);
    }

    #[test]
    fn test_order_book_entry_parsing() {
        let entry = vec![
            "31012.44".to_string(),    // price
            "69994.75267".to_string(), // amount
        ];

        assert_eq!(entry.first().map(|s| s.as_str()), Some("31012.44"));
        assert_eq!(entry.get(1).map(|s| s.as_str()), Some("69994.75267"));
    }

    #[test]
    fn test_order_book_entry_incomplete() {
        let entry = ["31012.44".to_string()];

        assert_eq!(entry.first().map(|s| s.as_str()), Some("31012.44"));
        assert_eq!(entry.get(1).map(|s| s.as_str()), None);
    }

    #[test]
    fn test_depth_data_structure() {
        let depth = DepthData {
            ts: "1691672864874".to_string(),
            symbol: "BTC_USDT".to_string(),
            asks: vec![vec!["31012.44".to_string(), "69994.75267".to_string()]],
            bids: vec![vec!["30000.00".to_string(), "1.00000".to_string()]],
        };

        assert_eq!(depth.ts, "1691672864874");
        assert_eq!(depth.symbol, "BTC_USDT");
        assert_eq!(depth.asks.len(), 1);
        assert_eq!(depth.bids.len(), 1);
        assert_eq!(depth.asks[0].first().map(|s| s.as_str()), Some("31012.44"));
        assert_eq!(
            depth.asks[0].get(1).map(|s| s.as_str()),
            Some("69994.75267")
        );
        assert_eq!(depth.bids[0].first().map(|s| s.as_str()), Some("30000.00"));
        assert_eq!(depth.bids[0].get(1).map(|s| s.as_str()), Some("1.00000"));
    }

    #[test]
    fn test_depth_data_serialization_roundtrip() {
        let depth = DepthData {
            ts: "1691672864874".to_string(),
            symbol: "ETH_USDT".to_string(),
            asks: vec![
                vec!["1850.00".to_string(), "10.5".to_string()],
                vec!["1851.00".to_string(), "5.2".to_string()],
            ],
            bids: vec![
                vec!["1849.00".to_string(), "8.7".to_string()],
                vec!["1848.00".to_string(), "12.1".to_string()],
            ],
        };

        let serialized = serde_json::to_string(&depth).expect("Failed to serialize depth");
        let deserialized: DepthData =
            serde_json::from_str(&serialized).expect("Failed to deserialize depth");

        assert_eq!(depth.ts, deserialized.ts);
        assert_eq!(depth.symbol, deserialized.symbol);
        assert_eq!(depth.asks.len(), deserialized.asks.len());
        assert_eq!(depth.bids.len(), deserialized.bids.len());
        assert_eq!(depth.asks[0], deserialized.asks[0]);
        assert_eq!(depth.bids[0], deserialized.bids[0]);
    }

    #[test]
    fn test_request_serialization() {
        let request = GetDepthRequest {
            symbol: "BTC_USDT".to_string(),
            limit: Some(50),
        };

        let serialized =
            serde_urlencoded::to_string(&request).expect("Failed to serialize request");
        assert!(serialized.contains("symbol=BTC_USDT"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_request_serialization_no_limit() {
        let request = GetDepthRequest {
            symbol: "BTC_USDT".to_string(),
            limit: None,
        };

        let serialized =
            serde_urlencoded::to_string(&request).expect("Failed to serialize request");
        assert!(serialized.contains("symbol=BTC_USDT"));
        assert!(!serialized.contains("limit"));
    }

    #[test]
    fn test_response_json_parsing() {
        let json = r#"{
            "ts": "1691672864874",
            "symbol": "BTC_USDT",
            "asks": [
                [
                    "31012.44",
                    "69994.75267"
                ]
            ],
            "bids": [
                [
                    "30000.00",
                    "1.00000"
                ]
            ]
        }"#;

        let response: GetDepthResponse =
            serde_json::from_str(json).expect("Failed to deserialize response");
        assert_eq!(response.ts, "1691672864874");
        assert_eq!(response.symbol, "BTC_USDT");
        assert_eq!(response.asks.len(), 1);
        assert_eq!(response.bids.len(), 1);
        assert_eq!(
            response.asks[0].first().map(|s| s.as_str()),
            Some("31012.44")
        );
        assert_eq!(
            response.asks[0].get(1).map(|s| s.as_str()),
            Some("69994.75267")
        );
        assert_eq!(
            response.bids[0].first().map(|s| s.as_str()),
            Some("30000.00")
        );
        assert_eq!(response.bids[0].get(1).map(|s| s.as_str()), Some("1.00000"));
    }

    #[test]
    fn test_empty_order_book() {
        let json = r#"{
            "ts": "1691672864874",
            "symbol": "BTC_USDT",
            "asks": [],
            "bids": []
        }"#;

        let response: GetDepthResponse =
            serde_json::from_str(json).expect("Failed to deserialize response");
        assert_eq!(response.ts, "1691672864874");
        assert_eq!(response.symbol, "BTC_USDT");
        assert_eq!(response.asks.len(), 0);
        assert_eq!(response.bids.len(), 0);
    }

    #[test]
    fn test_large_order_book() {
        let depth = DepthData {
            ts: "1691672864874".to_string(),
            symbol: "BTC_USDT".to_string(),
            asks: (0..50)
                .map(|i| vec![format!("{}.00", 30000 + i), format!("{}.0", i + 1)])
                .collect(),
            bids: (0..50)
                .map(|i| vec![format!("{}.00", 29999 - i), format!("{}.0", i + 1)])
                .collect(),
        };

        assert_eq!(depth.asks.len(), 50);
        assert_eq!(depth.bids.len(), 50);
        assert_eq!(depth.asks[0].first().map(|s| s.as_str()), Some("30000.00"));
        assert_eq!(depth.bids[0].first().map(|s| s.as_str()), Some("29999.00"));
    }
}
